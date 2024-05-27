use crate::err;
use crate::err::AmmError;
use crate::models::bound::BoundPool;
use crate::models::staked_lp::MemeTicket;
use crate::{consts::SLERF_MINT, utils::check_slerf_mint};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SwapCoinY<'info> {
    #[account(mut)]
    pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == quote_vault.key()
    )]
    quote_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = check_slerf_mint(user_quote_wallet.mint)
            @ err::acc("Quote mint should be SLERF mint")
    )]
    user_quote_wallet: Account<'info, TokenAccount>,
    #[account(init, payer = owner, space = MemeTicket::space())]
    meme_ticket: Account<'info, MemeTicket>,
    #[account(mut)]
    owner: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pool_signer_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> SwapCoinY<'info> {
    fn send_user_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_quote_wallet.to_account_info(),
            to: self.quote_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<SwapCoinY>, coin_in_amount: u64, coin_x_min_value: u64) -> Result<()> {
    let accs = ctx.accounts;

    msg!(
        "pool.meme_reserve.tokens BEFORE: {}",
        accs.pool.meme_reserve.tokens
    );

    if coin_in_amount == 0 {
        return Err(error!(AmmError::NoZeroTokens));
    }

    if accs.pool.locked {
        return Err(error!(AmmError::PoolIsLocked));
    }

    let swap_amount = accs
        .pool
        .swap_amounts(coin_in_amount, coin_x_min_value, true);

    token::transfer(
        accs.send_user_tokens(),
        swap_amount.amount_in + swap_amount.admin_fee_in,
    )
    .unwrap();

    msg!(
        "fees out: {}",
        swap_amount.admin_fee_out
    );

    let pool = &mut accs.pool;

    pool.admin_fees_quote += swap_amount.admin_fee_in;
    pool.admin_fees_meme += swap_amount.admin_fee_out;

    pool.quote_reserve.tokens += swap_amount.amount_in;
    pool.meme_reserve.tokens -= swap_amount.amount_out + swap_amount.admin_fee_out;

    msg!(
        "pool.meme_reserve.tokens AFTER: {}",
        pool.meme_reserve.tokens
    );
    if pool.meme_reserve.tokens == 0 {
        pool.locked = true;
    };

    let swap_amount_out = swap_amount.amount_out;

    let meme_ticket = &mut accs.meme_ticket;

    meme_ticket.setup(pool.key(), accs.owner.key(), swap_amount_out);

    msg!(
        "swapped_in: {}\n swapped_out: {}",
        swap_amount.amount_in,
        swap_amount.amount_out
    );

    return Ok(());
}
