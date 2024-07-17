use crate::consts::{ADMIN_KEY, CHAN_MINT};
use crate::err;
use crate::models::chan_swap::ChanSwap;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct NewChanSwap<'info> {
    #[account(
        mut,
        constraint = sender.key() == ADMIN_KEY.key()
    )]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = ChanSwap::space(),
        seeds = [ChanSwap::CHAN_SWAP_PREFIX, chan_vault.mint.as_ref()],
        bump
    )]
    pub chan_swap: Account<'info, ChanSwap>,
    ///CHECK: chan_swap signer pda
    #[account(
        seeds = [ChanSwap::SIGNER_PDA_PREFIX.as_bytes()],
        bump
    )]
    pub chan_swap_signer_pda: AccountInfo<'info>,
    #[account(
        mut,
        constraint = chan_vault.owner == chan_swap_signer_pda.key()
            @ err::acc("Chan vault authority must match staking pool signer"),
        constraint = chan_vault.mint == CHAN_MINT
            @ err::acc("Chan vault must be of CHAN mint"),
        constraint = chan_vault.close_authority == COption::None
            @ err::acc("Chan vault must not have close authority"),
        constraint = chan_vault.delegate == COption::None
            @ err::acc("Chan vault must not have delegate"),
    )]
    pub chan_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
}

pub fn handle<'info>(
    ctx: Context<NewChanSwap<'info>>,
    price_num: u64,
    price_denom: u64,
) -> Result<()> {
    let accs = ctx.accounts;

    let chan_swap = &mut accs.chan_swap;
    chan_swap.chan_vault = accs.chan_vault.key();
    chan_swap.chan_sol_price_num = price_num;
    chan_swap.chan_sol_price_denom = price_denom;

    Ok(())
}
