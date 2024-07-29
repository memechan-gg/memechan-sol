use crate::err::AmmError;
use crate::models::bound::BoundPool;
use crate::models::meme_ticket::MemeTicket;
use crate::models::user_stats::UserStats;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
#[instruction(coin_in_amount: u64, coin_x_min_value: u64, _ticket_number: u64)]
pub struct SwapCoinY<'info> {
    #[account(mut)]
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == quote_vault.key()
    )]
    pub quote_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_sol: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        space = MemeTicket::space(),
        seeds = [pool.key().as_ref(), owner.key().as_ref(), _ticket_number.to_le_bytes().as_ref()],
        bump,
    )]
    pub meme_ticket: Account<'info, MemeTicket>,
    #[account(mut)]
    pub user_stats: Account<'info, UserStats>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> SwapCoinY<'info> {
    fn send_user_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_sol.to_account_info(),
            to: self.quote_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(
    ctx: Context<SwapCoinY>,
    coin_in_amount: u64,
    coin_x_min_value: u64,
    _ticket_number: u64,
) -> Result<()> {
    let accs = ctx.accounts;

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

    let pool = &mut accs.pool;

    pool.admin_fees_quote += swap_amount.admin_fee_in;
    pool.admin_fees_meme += swap_amount.admin_fee_out;

    pool.quote_reserve.tokens += swap_amount.amount_in;
    pool.meme_reserve.tokens -= swap_amount.amount_out + swap_amount.admin_fee_out;

    let user_stats = &mut accs.user_stats;

    user_stats.quote_fees += swap_amount.admin_fee_in;
    user_stats.meme_fees += swap_amount.admin_fee_out;

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
