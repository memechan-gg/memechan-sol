use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

const DEFAULT_CLIFF: i64 = 172800000; // 48 hours; TODO: test
const DEFAULT_LINEAR: i64 = 1209600000; // 14 days; TODO: test

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct VestingConfig {
    start_ts: i64,
    cliff_ts: i64,
    end_ts: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct VestingData {
    released: u64,
    notional: u64,
}

pub fn default_config() -> VestingConfig {
    let current_ts = clock::Clock::get().unwrap().unix_timestamp;

    VestingConfig {
        start_ts: current_ts,
        cliff_ts: current_ts + DEFAULT_CLIFF,
        end_ts: current_ts + DEFAULT_CLIFF + DEFAULT_LINEAR, // TODO: Unit test
    }
}

impl VestingData {
    pub fn total_vested(self: &VestingData, config: &VestingConfig, current_ts: i64) -> u64 {
        if current_ts < config.cliff_ts {
            return 0;
        }

        if current_ts > config.end_ts {
            return self.notional;
        }

        (self.notional * (current_ts - config.start_ts) as u64) / duration(config)
    }

    pub fn duration(config: &VestingConfig) -> i64 {
        config.end_ts - config.start_ts
    }

    pub fn to_release(self: &VestingData, config: &VestingConfig, current_ts: u64) -> u64 {
        let to_release = total_vested(self, config, current_ts) - self.released;

        to_release
    }

    // Unchecked
    pub fn release(self: &mut VestingData, amount: u64) {
        self.released = self.released + amount;
    }

    // Getters
    pub fn released(self: &VestingData) -> u64 {
        self.released
    }

    pub fn notional(self: &VestingData) -> u64 {
        self.notional
    }

    pub fn current_stake(self: &VestingData) -> u64 {
        self.notional - self.released
    }
}
