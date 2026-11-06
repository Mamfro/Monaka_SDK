//! The busy block beside a stereo channel ([`monaka_contract::busy`]): a producer says what slow
//! work it is doing and the viewer reads it. Either side may create it. Every access waits at most
//! a couple of milliseconds; a miss is not an error.

use crate::shared::SharedBlock;
use crate::{ChannelName, Result};
use monaka_contract::busy::{Amount, Busy, BusyBlock};
use monaka_contract::protocol::{Block, Versioned};

const WAIT_MS: u32 = 2;

pub struct BusyChannel {
    block: SharedBlock<BusyBlock>,
}

impl BusyChannel {
    pub fn open(channel: &ChannelName) -> Result<Self> {
        let (name, lock) = channel.block(Block::Busy);
        Ok(Self { block: SharedBlock::<BusyBlock>::open_versioned(&name, &lock, WAIT_MS * 50)? })
    }

    /// Producer: work going on now.
    pub fn report(&mut self, text: &str, amount: Amount) {
        if let Some(mut record) = self.block.lock(WAIT_MS).tolerant() {
            record.report(text, amount, crate::tick());
        }
    }

    /// Producer: the work is over. Waits longer than a report: a report missed is followed by the
    /// next, this is not.
    pub fn clear(&mut self) {
        if let Some(mut record) = self.block.lock(WAIT_MS * 50).tolerant() {
            record.clear();
        }
    }

    /// Viewer: the work going on, if any (the block busy: none this time).
    pub fn read(&mut self) -> Option<Busy> {
        let record = self.block.lock(WAIT_MS).tolerant()?;
        if !record.is_compatible() {
            return None;
        }
        record.read(crate::tick())
    }
}

#[cfg(test)]
mod tests;
