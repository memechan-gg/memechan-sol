use crate::vesting::VestingConfig;
use anchor_lang::prelude::*;
use std::mem;

#[account]
pub struct StakingPool {
    pub pool: Pubkey,
    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,
    pub wsol_vault: Pubkey,
    pub vesting_config: VestingConfig,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
}

impl StakingPool {
    pub const SIGNER_PDA_PREFIX: &'static [u8; 7] = b"staking";

    pub fn space() -> usize {
        let discriminant = 8;
        let pool = 32;
        let meme_vault = 32;
        let meme_mint = 32;
        let wsol_vault = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let stakes_total = 8;
        let fees_x_total = 8;
        let fees_y_total = 8;

        discriminant
            + pool
            + meme_vault
            + meme_mint
            + wsol_vault
            + vesting_config
            + stakes_total
            + fees_x_total
            + fees_y_total
    }
}
