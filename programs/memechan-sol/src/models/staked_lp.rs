use crate::vesting::VestingData;
use anchor_lang::prelude::*;
use std::mem;

#[cfg(not(feature = "dev"))]
pub const LOCK_TIME: i64 = 60 * 60 * 4; // 4 hours

#[cfg(feature = "dev")]
pub const LOCK_TIME: i64 = 2; // 2 seconds - for testing

#[derive(Default)]
#[account]
pub struct MemeTicket {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
    pub withdraws_meme: u64,
    pub withdraws_quote: u64,
    pub until_timestamp: i64,
    pub vesting: VestingData,
}

impl MemeTicket {
    pub const ADMIN_TICKET_PREFIX: &'static [u8; 12] = b"admin_ticket";

    pub fn space() -> usize {
        let discriminant = 8;
        let owner = 32;
        let pool = 32;
        let amount = 8;
        let withdraws_meme = 8;
        let withdraws_quote = 8;
        let until_timestamp = 8;
        let vesting = mem::size_of::<VestingData>();

        discriminant
            + owner
            + pool
            + amount
            + withdraws_meme
            + withdraws_quote
            + until_timestamp
            + vesting
    }

    pub fn is_unlocked(&self) -> bool {
        self.until_timestamp <= Clock::get().unwrap().unix_timestamp
    }

    pub fn setup(&mut self, pool: Pubkey, owner: Pubkey, amount: u64) {
        self.pool = pool;
        self.owner = owner;
        self.amount = amount;
        self.withdraws_meme = 0;
        self.withdraws_quote = 0;
        self.until_timestamp = Clock::get().unwrap().unix_timestamp + LOCK_TIME;
        msg!(&self.until_timestamp.to_string());
        self.vesting = VestingData {
            notional: amount,
            released: 0,
        };
    }
}
