use anchor_lang::prelude::*;
use crate::vesting::VestingData;

pub const LOCK_TIME: i64 = 4 * 3600;

#[account]
pub struct StakedLP {
    pub owner: Pubkey,
    pub amount: u64,
    pub withdraws_meme: u64,
    pub withdraws_wsol: u64,
    pub until_timestamp: i64,
    pub vesting: VestingData,
}
