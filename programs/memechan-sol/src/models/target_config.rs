use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[account]
pub struct TargetConfig {
    pub token_target_amount: u64,
    pub token_mint: Pubkey,
}

impl TargetConfig {
    pub const CONFIG_PREFIX: &'static [u8; 6] = b"config";

    pub fn space() -> usize {
        let discriminant = 8;
        let token_target_amount = 8;
        let token_mint = 32;

        discriminant + token_target_amount + token_mint
    }
}
