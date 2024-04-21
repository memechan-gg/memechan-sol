use anchor_lang::prelude::*;

pub const LOCK_TIME: i64 = 4 * 3600;

#[account]
pub struct StakedLP {
    pub owner: Pubkey,
    pub amount: u64,
    pub until_timestamp: i64,
}
