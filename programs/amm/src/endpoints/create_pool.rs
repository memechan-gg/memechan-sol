//! Creates a new [`Pool`] account. This endpoint is generic and can be used for
//! constant product curve, in which case the amplifier input is going to be
//! zero, and for stable curve.
//!
//! The number of remaining accounts determine how many reserves does the pool
//! have, ie. for multi-asset pools provide up to 4 remaining accounts.
//!
//! The remaining accounts must be vaults, ie. token accounts owned by the pool
//! signers. The order of the accounts does not matter.

use crate::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = Pool::space()
    )]
    pub pool: Account<'info, Pool>,
    /// CHECK: UNSAFE_CODES.md#signer
    #[account(
        seeds = [Pool::SIGNER_PDA_PREFIX, pool.key().as_ref()],
        bump
    )]
    pub pool_signer: AccountInfo<'info>,
    #[account(
        seeds = [ProgramToll::PDA_SEED],
        bump,
    )]
    pub program_toll: Account<'info, ProgramToll>,
    #[account(
        constraint = program_toll_wallet.mint == lp_mint.key()
            @ err::acc("Toll wallet must be of LP mint"),
        constraint = program_toll_wallet.owner == program_toll.authority
            @ err::acc(
                "Toll wallet authority must match \
                program toll authority"
            ),
    )]
    pub program_toll_wallet: Account<'info, TokenAccount>,
    #[account(
        constraint = lp_mint.mint_authority == COption::Some(pool_signer.key())
            @ err::acc("LP mint authority must be the pool signer"),
        constraint = lp_mint.freeze_authority == COption::None
            @ err::acc("LP mint mustn't have a freeze authority"),
    )]
    pub lp_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
