pub mod bound;
pub mod chan_swap;
pub mod fee_distribution;
pub mod fees;
pub mod points_epoch;
pub mod staked_lp;
pub mod staking;
pub mod target_config;

use anchor_lang::prelude::*;

extern crate std;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenLimit {
    pub mint: Pubkey,
    pub tokens: TokenAmount,
}

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd,
)]
pub struct TokenAmount {
    pub amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Reserve {
    pub tokens: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
}

pub struct SwapAmount {
    pub amount_in: u64,
    pub amount_out: u64,
    pub admin_fee_in: u64,
    pub admin_fee_out: u64,
}
