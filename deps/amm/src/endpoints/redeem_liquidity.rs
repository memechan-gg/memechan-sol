//! Endpoint to redeem liquidity for a given [`Pool`] (for either constant
//! product or stable curves). Each user is allowed to redeem liquidity.
//! Moreover, this endpoint computes the necessary amount of tokens that
//! need to be redeem, given the amount of LP tokens the user wants to burn,
//! such that the redemption respects the current pool ratio.

use crate::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct RedeemLiquidity<'info> {
    /// User to redeem funds
    pub user: Signer<'info>,
    /// Pool to redeem funds from
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    /// CHECK: UNSAFE_CODES.md#signer
    #[account(
        seeds = [Pool::SIGNER_PDA_PREFIX, pool.key().as_ref()],
        bump
    )]
    pub pool_signer: AccountInfo<'info>,
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
