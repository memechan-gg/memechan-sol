use crate::err::AmmError;
use crate::models::bound::BoundPool;
use crate::models::meme_ticket::MemeTicket;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SwapCoinX<'info> {
    #[account(mut)]
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        has_one = pool,
        has_one = owner
    )]
    pub meme_ticket: Account<'info, MemeTicket>,
    #[account(mut)]
    pub user_sol: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == quote_vault.key()
    )]
    pub quote_vault: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> SwapCoinX<'info> {
    fn send_tokens_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.quote_vault.to_account_info(),
            to: self.user_sol.to_account_info(),
            authority: self.pool_signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<SwapCoinX>, coin_in_amount: u64, coin_y_min_value: u64) -> Result<()> {
    let accs = ctx.accounts;

    if coin_in_amount == 0 {
        return Err(error!(AmmError::NoZeroTokens));
    }

    let user_ticket = &mut accs.meme_ticket;

    if !user_ticket.is_unlocked() {
        return Err(error!(AmmError::TicketTokensLocked));
    }

    if coin_in_amount > user_ticket.amount {
        return Err(error!(AmmError::NotEnoughTicketTokens));
    }

    let pool_state = &mut accs.pool;

    if pool_state.locked {
        return Err(error!(AmmError::PoolIsLocked));
    }

    let swap_amount = pool_state.swap_amounts(coin_in_amount, coin_y_min_value, false);

    pool_state.admin_fees_meme += swap_amount.admin_fee_in;
    pool_state.admin_fees_quote += swap_amount.admin_fee_out;

    pool_state.meme_reserve.tokens += swap_amount.amount_in;
    pool_state.quote_reserve.tokens -= swap_amount.amount_out + swap_amount.admin_fee_out;

    user_ticket.amount -= coin_in_amount;
    user_ticket.vesting.notional -= coin_in_amount;

    let seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[ctx.bumps.pool_signer],
    ];

    let signer_seeds = &[&seeds[..]];

    token::transfer(
        accs.send_tokens_to_user().with_signer(signer_seeds),
        swap_amount.amount_out,
    )
    .unwrap();

    msg!(
        "swapped_in: {}\n swapped_out: {}",
        swap_amount.amount_in,
        swap_amount.amount_out
    );

    Ok(())
}
