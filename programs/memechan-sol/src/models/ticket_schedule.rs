use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[account]
pub struct TicketSchedule {
    pub meme_ticket: Pubkey,
    pub withdrawn: u64,
    pub until_ts: i64,
}

impl TicketSchedule {
    pub const SCHEDULE_PREFIX: &'static [u8; 8] = b"schedule";

    pub fn space() -> usize {
        let discriminant = 8;
        let meme_ticket = 32;
        let withdrawn = 8;
        let until_ts = 8;
        let padding = 64;

        discriminant + meme_ticket + withdrawn + until_ts + padding
    }
}
