//! A process's running time on the GPU, as Windows counts it (the `GPU Engine` counters), whatever
//! API it draws with: only the time the GPU really worked for it, none of the gaps between.

use windows::Win32::System::Performance::*;
use windows::core::w;

#[cfg(test)]
mod tests;

/// `PDH_MORE_DATA`: the buffer asked for must be larger.
const MORE_DATA: u32 = 0x8000_07D2;

/// Whether counter instance `name` is process `pid`'s 3D engine (`pid_123_luid_..._engtype_3D`).
fn is_3d_engine_of(name: &str, pid: u32) -> bool {
    name.strip_prefix("pid_").and_then(|rest| rest.split_once('_')).is_some_and(|(id, _)| id.parse() == Ok(pid)) && name.to_ascii_lowercase().ends_with("_engtype_3d")
}

/// A process's running time on the GPU's 3D engines.
pub struct GpuClock {
    query: PDH_HQUERY,
    counter: PDH_HCOUNTER,
    pid: u32,
}

impl GpuClock {
    /// The clock of process `pid`; none where the counters are not there.
    pub fn open(pid: u32) -> Option<Self> {
        let mut query = PDH_HQUERY::default();
        let mut counter = PDH_HCOUNTER::default();
        // SAFETY: out-pointers to locals, a NUL-terminated path, the query closed on failure.
        unsafe {
            if PdhOpenQueryW(None, 0, &mut query) != 0 {
                return None;
            }
            if PdhAddEnglishCounterW(query, w!(r"\GPU Engine(*)\Running Time"), 0, &mut counter) != 0 {
                let _ = PdhCloseQuery(query);
                return None;
            }
        }
        Some(Self { query, counter, pid })
    }

    /// Its running time so far (ticks of 100 ns), summed over the engines.
    pub fn read(&mut self) -> Option<u64> {
        // SAFETY: our own query.
        if unsafe { PdhCollectQueryData(self.query) } != 0 {
            return None;
        }
        let (mut bytes, mut count) = (0u32, 0u32);
        // SAFETY: asks the size; no buffer is written.
        if unsafe { PdhGetRawCounterArrayW(self.counter, &mut bytes, &mut count, None) } != MORE_DATA {
            return None;
        }
        // In 8-byte words: the items hold 64-bit values.
        let mut buffer = vec![0u64; (bytes as usize).div_ceil(8)];
        let items = buffer.as_mut_ptr().cast::<PDH_RAW_COUNTER_ITEM_W>();
        // SAFETY: a buffer of the size asked for, aligned for the items.
        if unsafe { PdhGetRawCounterArrayW(self.counter, &mut bytes, &mut count, Some(items)) } != 0 {
            return None;
        }
        // SAFETY: `count` items were written at its start; their names point into the buffer.
        let items = unsafe { std::slice::from_raw_parts(items, count as usize) };
        let mut total = 0u64;
        for item in items {
            // SAFETY: as above, a NUL-terminated name.
            let name = unsafe { item.szName.to_string() }.unwrap_or_default();
            if is_3d_engine_of(&name, self.pid) {
                total = total.wrapping_add(item.RawValue.FirstValue as u64);
            }
        }
        Some(total)
    }
}

impl Drop for GpuClock {
    fn drop(&mut self) {
        // SAFETY: our own query, closed once.
        let _ = unsafe { PdhCloseQuery(self.query) };
    }
}
