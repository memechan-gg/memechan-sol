use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserStats {
    pub is_initialized: bool,
    pub pool: Pubkey,
    pub referral: Pubkey,
    pub meme_fees: u64,
    pub quote_fees: u64,
    pub meme_received: u64,
    pub quote_received: u64,
    pub chan_received: u64,
    padding: [u8; 8],
}

impl UserStats {
    pub const STATS_PREFIX: &'static [u8; 6] = b"ustats";

    pub fn space() -> usize {
        let discriminant = 8;

        let is_initialized = 1;
        let pool = 32;
        let referral = 32;
        let base_spent = 8;
        let quote_spent = 8;
        let base_received = 8;
        let quote_received = 8;
        let chan_received = 8;

        let padding = 64;

        discriminant
            + is_initialized
            + pool
            + referral
            + base_spent
            + quote_spent
            + base_received
            + quote_received
            + chan_received
            + padding
    }
}
