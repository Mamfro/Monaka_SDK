//! Telling the player about slow work: the viewer shows what is said here on a panel of its own
//! while the game's frames have stopped, and does not take the silence for a game gone. Call
//! [`known`] as work of a known size goes along, or [`unknown`] for work of an unknown size, from
//! any thread (the render thread in the middle of a long stall too), and [`finished`] at its end.

use monaka_channel::ChannelName;
use monaka_channel::busy::BusyChannel;
use monaka_contract::busy::Amount;
use monaka_core::sync::Locked;
use std::sync::Mutex;

static CHANNEL: Mutex<Option<BusyChannel>> = Mutex::new(None);

/// Opens the block on the run's channel.
pub(crate) fn open(channel: &ChannelName) {
    match BusyChannel::open(channel) {
        Ok(opened) => *CHANNEL.locked() = Some(opened),
        Err(e) => crate::log!("busy block: not opened ({e})"),
    }
}

/// The work is over, with the run.
pub(crate) fn close() {
    finished();
    *CHANNEL.locked() = None;
}

/// `text`: `done` of `total` steps are done.
pub fn known(text: &str, done: u32, total: u32) {
    report(text, Amount::Known { done, total });
}

/// `text`: work goes on, with no telling how much is left.
pub fn unknown(text: &str) {
    report(text, Amount::Unknown);
}

/// The work is over.
pub fn finished() {
    if let Some(channel) = CHANNEL.locked().as_mut() {
        channel.clear();
    }
}

fn report(text: &str, amount: Amount) {
    if let Some(channel) = CHANNEL.locked().as_mut() {
        channel.report(text, amount);
    }
}
