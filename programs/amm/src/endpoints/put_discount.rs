//! Either creates a [`Discount`] model for a user - if it doesn't exist yet -
//! or updates an existing one. In the former scenario, the authority must be
//! mutable so that we can transfer rent to the new account.
//!
//! See the [`crate::models::discount`] module for more info.

use crate::prelude::*;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct PutDiscount<'info> {
    #[account(
        constraint = authority.key() == discount_settings.authority
            @ err::acc("The authority must be the discount settings authority"),
    )]
    pub authority: Signer<'info>,
    /// CHECK: we create the discount account if it does not exist yet in the
    /// [`handle`] fn
    #[account(
        mut,
        seeds = [Discount::PDA_PREFIX, user.as_ref()],
        bump,
    )]
    pub discount: AccountInfo<'info>,
    #[account(
        seeds = [DiscountSettings::PDA_SEED],
        bump,
    )]
    pub discount_settings: Account<'info, DiscountSettings>,
    pub system_program: Program<'info, System>,
}
