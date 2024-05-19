use crate::vesting::VestingConfig;
use anchor_lang::prelude::*;
use std::mem;

#[account]
pub struct StakingPool {
    pub pool: Pubkey,
    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,
    pub lp_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub quote_vault: Pubkey,
    pub vesting_config: VestingConfig,
    pub lp_tokens_withdrawn: u64,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
}

impl StakingPool {
    pub const POOL_PREFIX: &'static [u8; 12] = b"staking_pool";
    pub const SIGNER_PDA_PREFIX: &'static [u8; 7] = b"staking";

    pub fn space() -> usize {
        let discriminant = 8;
        let pool = 32;
        let meme_vault = 32;
        let meme_mint = 32;
        let lp_vault = 32;
        let lp_mint = 32;
        let quote_vault = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let lp_tokens_withdrawn = 8;
        let stakes_total = 8;
        let fees_x_total = 8;
        let fees_y_total = 8;

        discriminant
            + pool
            + meme_vault
            + meme_mint
            + lp_vault
            + lp_mint
            + quote_vault
            + vesting_config
            + lp_tokens_withdrawn
            + stakes_total
            + fees_x_total
            + fees_y_total
    }
}
