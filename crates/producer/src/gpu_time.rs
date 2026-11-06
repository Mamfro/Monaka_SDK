//! How long the GPU really worked on each of the game's frames, for the Stats panel. The time
//! between frames says nothing about that once frames are held to the headset's refresh, and a
//! timer inside the game counts the GPU's idle gaps too. Windows keeps each process's running time
//! on each GPU engine, whatever API drew: read here about once a second, on a thread of its own,
//! while the Stats panel shows, and shared out over the frames presented meanwhile.

use monaka_channel::gpu_clock::GpuClock;
use monaka_core::sync::Locked;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering::*};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests;

/// How often the running time is read, and how often the thread looks up meanwhile.
const SAMPLE_EVERY: Duration = Duration::from_secs(1);
const NAP: Duration = Duration::from_millis(50);
/// What the counter counts in, per millisecond.
const TICKS_PER_MS: f64 = 10_000.0;

/// The GPU's work a frame (ms, as `f32` bits; 0: not known yet).
static PER_FRAME_MS: AtomicU32 = AtomicU32::new(0);
/// The game's frames presented.
static PRESENTS: AtomicU64 = AtomicU64::new(0);
static CLOSING: AtomicBool = AtomicBool::new(false);
static THREAD: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

/// The GPU's time per frame (ms) between two readings, none when no frame came between them.
fn per_frame_ms(before: (u64, u64), now: (u64, u64)) -> Option<f32> {
    let frames = now.1.checked_sub(before.1).filter(|frames| *frames > 0)?;
    Some((now.0.wrapping_sub(before.0) as f64 / TICKS_PER_MS / frames as f64) as f32)
}

/// A frame was presented. The thread reading the clock starts with the first frame timed.
pub(crate) fn presented(timed: bool) {
    PRESENTS.fetch_add(1, Relaxed);
    if timed && let Ok(mut thread) = THREAD.try_lock()
        && thread.is_none()
    {
        CLOSING.store(false, Relaxed);
        *thread = std::thread::Builder::new().name("monaka gpu time".into()).spawn(watch).ok();
    }
}

/// The GPU's work a frame (ms; 0: not known yet).
pub(crate) fn frame_ms() -> f32 {
    f32::from_bits(PER_FRAME_MS.load(Relaxed))
}

/// The thread ends, with the run.
pub(crate) fn stop() {
    CLOSING.store(true, Relaxed);
    if let Some(thread) = THREAD.locked().take() {
        let _ = thread.join();
    }
    PER_FRAME_MS.store(0, Relaxed);
}

fn watch() {
    let Some(mut clock) = GpuClock::open(std::process::id()) else {
        crate::log!("GPU time: the GPU Engine counters are not there; not measured this run");
        return;
    };
    let mut last: Option<(u64, u64)> = None;
    let mut sampled = Instant::now();
    while !CLOSING.load(Relaxed) {
        std::thread::sleep(NAP);
        if sampled.elapsed() < SAMPLE_EVERY {
            continue;
        }
        sampled = Instant::now();
        if !crate::debug::timing() {
            last = None;
            PER_FRAME_MS.store(0, Relaxed);
            continue;
        }
        let now = clock.read().map(|running| (running, PRESENTS.load(Relaxed)));
        if let Some(ms) = last.zip(now).and_then(|(before, now)| per_frame_ms(before, now)) {
            PER_FRAME_MS.store(ms.to_bits(), Relaxed);
        }
        last = now;
    }
}
