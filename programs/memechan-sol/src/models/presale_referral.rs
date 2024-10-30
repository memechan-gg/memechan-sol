use anchor_lang::prelude::*;

#[account]
pub struct PresaleReferral {
    pub amount: u64,
    pub referrer: Pubkey,
    pub padding: [u8; 8],
}
