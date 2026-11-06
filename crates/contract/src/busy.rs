//! Slow work a producer is doing, beside the run's channel: what it is and how far along, so the
//! viewer can show it while the game's own frames have stopped. Work of a known size says how much
//! is done; work of an unknown size only that it goes on. While it is fresh the viewer does not
//! count the game's silence against it.

use crate::protocol::{Stamp, Versioned};

#[cfg(test)]
mod tests;

pub const BUSY_MAGIC: u32 = 0x5953_5542;
pub const BUSY_MAJOR: u16 = 1;
/// Bytes of UTF-8 in the work's description (cut at a character).
pub const BUSY_TEXT_BYTES: usize = 64;
/// Work not heard of for this long (ms) is over: its producer stopped. Longer than any one step of
/// slow work, as the work may be held up inside a driver's call between reports.
pub const BUSY_FRESH_MS: u64 = 60_000;

/// How much of the work is done.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amount {
    /// `done` of `total` steps.
    Known { done: u32, total: u32 },
    /// No telling how much is left.
    Unknown,
}

impl Amount {
    /// How far along, 0 to 1; none when that cannot be told.
    pub fn fraction(self) -> Option<f32> {
        match self {
            Amount::Known { done, total } if total > 0 => Some((done as f32 / total as f32).clamp(0.0, 1.0)),
            _ => None,
        }
    }
}

/// Work going on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Busy {
    pub text: String,
    pub amount: Amount,
}

/// The block's record.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BusyBlock {
    pub stamp: Stamp,
    /// 1 while work goes on.
    pub active: u32,
    pub done: u32,
    /// 0: an unknown amount.
    pub total: u32,
    pub text_len: u32,
    /// When it was last written (`monaka_channel::tick`, ms).
    pub at: u64,
    pub text: [u8; BUSY_TEXT_BYTES],
}

impl BusyBlock {
    /// Work going on now, `at` the time.
    pub fn report(&mut self, text: &str, amount: Amount, at: u64) {
        let mut length = text.len().min(BUSY_TEXT_BYTES);
        while !text.is_char_boundary(length) {
            length -= 1;
        }
        self.text[..length].copy_from_slice(&text.as_bytes()[..length]);
        self.text_len = length as u32;
        (self.done, self.total) = match amount {
            Amount::Known { done, total } => (done, total),
            Amount::Unknown => (0, 0),
        };
        self.at = at;
        self.active = 1;
    }

    /// The work is over.
    pub fn clear(&mut self) {
        self.active = 0;
    }

    /// The work going on, while its producer keeps saying so (`now`: the time).
    pub fn read(&self, now: u64) -> Option<Busy> {
        if self.active == 0 || now.saturating_sub(self.at) > BUSY_FRESH_MS {
            return None;
        }
        let length = (self.text_len as usize).min(BUSY_TEXT_BYTES);
        let text = String::from_utf8_lossy(&self.text[..length]).into_owned();
        let amount = if self.total > 0 { Amount::Known { done: self.done, total: self.total } } else { Amount::Unknown };
        Some(Busy { text, amount })
    }
}

impl Versioned for BusyBlock {
    const MAGIC: u32 = BUSY_MAGIC;
    const MAJOR: u16 = BUSY_MAJOR;
    const MINOR: u16 = 0;
    const WHAT: &'static str = "busy block";
    const CAPACITY: usize = 1024;

    fn stamp(&self) -> Stamp {
        self.stamp
    }

    fn start(&mut self) {
        *self = Self { stamp: Stamp::of::<Self>(), active: 0, done: 0, total: 0, text_len: 0, at: 0, text: [0; BUSY_TEXT_BYTES] };
    }
}
