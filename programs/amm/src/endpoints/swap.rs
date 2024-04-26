//! Given sell (base) and buy (quote) tokens which belong to the pool (ie. there
//! are two reserves with the relevant mints), we calculate based on the curve
//! and current pool's state how many tokens should the user get in return.
//!
//! The user pays a fee for the swap, which is scaled down by the [`Discount`]
//! associated with this user. A fraction of the swap fee is sent to program
//! owner's wallet in LP tokens.

use crate::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct Swap<'info> {
    /// Authority over the sell wallet.
    pub user: Signer<'info>,
    /// CHECK: The user's discount might not be initialized, and that's fine,
    /// we are conditionally parsing this account and only if it's valid
    /// will we consider the discount.
    #[account(
        seeds = [Discount::PDA_PREFIX, user.key().as_ref()],
        bump,
    )]
    pub discount: AccountInfo<'info>,
    #[account(mut)]
    pub pool: Box<Account<'info, Pool>>,
    /// CHECK: pda signer
    #[account(
        seeds = [Pool::SIGNER_PDA_PREFIX, pool.key().as_ref()],
        bump,
    )]
    pub pool_signer: AccountInfo<'info>,
    /// Tokens to SELL flow FROM this account.
    #[account(
        mut,
        constraint = sell_wallet.mint != buy_wallet.mint
            @ err::acc("Mint to swap from mustn't equal the mint to swap to"),
        constraint = sell_wallet.mint == sell_vault.mint
            @ err::acc("Sell wallet mint must match sell vault mint"),
    )]
    pub sell_wallet: Box<Account<'info, TokenAccount>>,
    /// Tokens to BUY flow INTO this account.
    #[account(
        mut,
        constraint = buy_wallet.mint == buy_vault.mint
            @ err::acc("Buy wallet mint must match buy vault mint"),
    )]
    pub buy_wallet: Box<Account<'info, TokenAccount>>,
    /// Tokens to SELL flow INTO this account.
    // #[account(
    //     mut,
    //     // either the mint is not any reserve's mint, or the vault doesn't match
    //     constraint = pool.reserve_vault(sell_vault.mint) == Some(sell_vault.key())
    //         @ err::acc("Sell vault is not reserve's vault"),
    // )]
    pub sell_vault: Box<Account<'info, TokenAccount>>,
    /// Tokens to BUY flow FROM this account.
    // #[account(
    //     mut,
    //     // either the mint is not any reserve's mint, or the vault doesn't match
    //     constraint = pool.reserve_vault(buy_vault.mint) == Some(buy_vault.key())
    //         @ err::acc("Buy vault is not reserve's vault"),
    // )]
    pub buy_vault: Box<Account<'info, TokenAccount>>,
    /// We mint LPs into `program_toll_wallet`
    #[account(
        mut,
        constraint = pool.mint == lp_mint.key() @ err::acc("LP mint mismatch"),
        constraint = lp_mint.supply > 0 @ err::acc("No liquidity provided yet"),
    )]
    pub lp_mint: Box<Account<'info, Mint>>,
    /// Part of the fee is the program owner's cut, and is payed in LPs.
    #[account(
        mut,
        constraint = pool.program_toll_wallet == program_toll_wallet.key()
            @ err::acc("Program toll wallet mismatch"),
    )]
    pub program_toll_wallet: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}
