use crate::err::AmmError;
use crate::libraries::MulDiv;
use crate::models::bound::BoundPool;
use crate::models::fees::{REFERRER_POINTS_DENOMINATOR, REFERRER_POINTS_NUMERATOR};
use crate::models::points_epoch::PointsEpoch;
use crate::models::staked_lp::MemeTicket;
use crate::models::user_points::UserPoints;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::cmp::min;

#[derive(Accounts)]
#[instruction(coin_in_amount: u64, coin_x_min_value: u64, _ticket_number: u64)]
pub struct SwapCoinY<'info> {
    #[account(mut)]
    pool: Box<Account<'info, BoundPool>>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == quote_vault.key()
    )]
    quote_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    user_sol: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = owner,
        space = MemeTicket::space(),
        seeds = [pool.key().as_ref(), owner.key().as_ref(), _ticket_number.to_le_bytes().as_ref()],
        bump,
    )]
    meme_ticket: Account<'info, MemeTicket>,
    #[account(
        mut,
        seeds = [UserPoints::USER_POINTS_PREFIX, owner.key().as_ref()],
        bump
    )]
    user_points: Account<'info, UserPoints>,
    #[account(
        mut,
        seeds = [UserPoints::USER_POINTS_PREFIX, user_points.referrer.as_ref()],
        bump
    )]
    referrer_points: Option<Account<'info, UserPoints>>,
    points_epoch: Account<'info, PointsEpoch>,
    #[account(mut)]
    global_ref: Option<Account<'info, TokenAccount>>,
    #[account(mut)]
    local_ref: Option<Account<'info, TokenAccount>>,
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

    let available_points_amt = accs.points_epoch.points_total - accs.points_epoch.points_given;

    let points = get_swap_points(
        swap_amount.amount_in + swap_amount.admin_fee_in,
        &accs.points_epoch,
    );
    let clamped_points = min(available_points_amt, points);

    if clamped_points > 0 {
        accs.user_points.points += clamped_points;

        if let Some(referrer_points) = &mut accs.referrer_points {
            let ref_points = clamped_points
                .mul_div_floor(REFERRER_POINTS_NUMERATOR, REFERRER_POINTS_DENOMINATOR)
                .unwrap();
            if ref_points > 0 {
                let ref_points = min(available_points_amt - clamped_points, ref_points);
                referrer_points.points += ref_points;
            }
        }
    }

    let pool = &mut accs.pool;

    pool.admin_fees_quote += swap_amount.admin_fee_in;
    pool.admin_fees_meme += swap_amount.admin_fee_out;

    pool.quote_reserve.tokens += swap_amount.amount_in;
    pool.meme_reserve.tokens -= swap_amount.amount_out + swap_amount.admin_fee_out;

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

pub fn get_swap_points(buy_amount: u64, points_epoch: &PointsEpoch) -> u64 {
    return buy_amount
        .mul_div_floor(
            points_epoch.points_per_sol_num,
            points_epoch.points_per_sol_denom,
        )
        .unwrap();
}
