//! Endpoint to deposit liquidity for a given [`Pool`] (for either constant
//! product or stable curves). Each user is allowed to deposit liquidity.
//! Moreover, this endpoint computes the necessary amount of tokens that
//! need to be deposited, in order to respect the current pool ratio, as
//! well as the amount of LP tokens to be minted, accordingly.
//! When a [`Pool`] is created by an admin, the amount of LP tokens to be
//! minted corresponds to the minimum value of tokens deposited.

use crate::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct DepositLiquidity<'info> {
    /// User to deposit funds from
    pub user: Signer<'info>,
    /// Pool to deposit funds
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    /// CHECK: UNSAFE_CODES.md#signer
    #[account(
        seeds = [Pool::SIGNER_PDA_PREFIX, pool.key().as_ref()],
        bump
    )]
    pub pool_signer_pda: AccountInfo<'info>,
    #[account(
        mut,
        constraint = lp_mint.key() == pool.mint.key()
            @ err::acc("LP mint must match pool's mint")
    )]
    pub lp_mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = lp_token_wallet.mint == pool.mint.key()
            @ err::acc("LP wallet must be of the same mint as pool's mint"),
    )]
    pub lp_token_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}
