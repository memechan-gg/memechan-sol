use anchor_lang::prelude::*;
use crate::consts::{DEFAULT_CLIFF, DEFAULT_LINEAR};


#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct VestingConfig {
    pub start_ts: i64,
    pub cliff_ts: i64,
    pub end_ts: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct VestingData {
    pub released: u64,
    pub notional: u64,
}

pub fn default_config() -> VestingConfig {
    let current_ts = Clock::get().unwrap().unix_timestamp;

    VestingConfig {
        start_ts: current_ts,
        cliff_ts: current_ts + DEFAULT_CLIFF,
        end_ts: current_ts + DEFAULT_CLIFF + DEFAULT_LINEAR, // TODO: Unit test
    }
}

impl VestingData {
    pub fn total_vested(&self, config: &VestingConfig, current_ts: i64) -> u64 {
        if current_ts <= config.cliff_ts {
            return 0;
        }

        if current_ts >= config.end_ts {
            return self.notional;
        }

        let available_vested = (self.notional as u128 * (current_ts - config.cliff_ts) as u128) / config.duration() as u128;
        available_vested as u64
    }

    pub fn to_release(&self, config: &VestingConfig, current_ts: i64) -> u64 {
        let to_release = self.total_vested(config, current_ts) - self.released;

        to_release
    }

    pub fn release(&mut self, amount: u64) {
        self.released += amount;
    }

    pub fn current_stake(&self) -> u64 {
        self.notional - self.released
    }
}

impl VestingConfig {
    pub fn duration(&self) -> i64 {
        self.end_ts - self.cliff_ts
    }
}
