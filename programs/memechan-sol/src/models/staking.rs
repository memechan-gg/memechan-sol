use crate::vesting::VestingConfig;
use anchor_lang::prelude::*;
use std::mem;

#[derive(Default)]
#[account]
pub struct StakingPool {
    pub pool: Pubkey,
    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,
    pub lp_vault: Pubkey,
    pub lp_mint: Pubkey,
    pub quote_vault: Pubkey,
    pub quote_mint: Pubkey,
    pub chan_vault: Pubkey,
    pub amm_pool: Pubkey,
    pub chan_amm_pool: Pubkey,
    pub vesting_config: VestingConfig,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
    pub fees_z_total: u64,
    pub is_active: bool,
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
        let quote_mint = 32;
        let amm_pool = 32;
        let chan_amm_pool = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let stakes_total = 8;
        let fees_x_total = 8;
        let fees_y_total = 8;
        let fees_z_total = 8;
        let is_active = 1;
        let padding = 128;

        discriminant
            + pool
            + meme_vault
            + meme_mint
            + lp_vault
            + lp_mint
            + quote_vault
            + quote_mint
            + amm_pool
            + chan_amm_pool
            + vesting_config
            + stakes_total
            + fees_x_total
            + fees_y_total
            + fees_z_total
            + is_active
        // + padding
    }
}
