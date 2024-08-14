use crate::consts::{
    BOOSTED_POINTS_AMOUNT, BOOSTED_SOL_AMOUNT, MAX_POINTS_AVAILABLE, POINTS_MINT, POINTS_PDA,
};
use crate::err::AmmError;
use crate::libraries::MulDiv;
use crate::models::bound::BoundPool;
use crate::models::staked_lp::MemeTicket;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::cmp::min;

#[derive(Accounts)]
#[instruction(coin_in_amount: u64, coin_x_min_value: u64, _ticket_number: u64)]
pub struct SwapCoinY<'info> {
    #[account(mut)]
    pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == quote_vault.key()
    )]
    quote_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    user_sol: Account<'info, TokenAccount>,
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
        token::mint = points_mint,
        token::authority = owner,
    )]
    user_points: Account<'info, TokenAccount>,
    #[account(
        mut,
        token::mint = points_mint,
        constraint = referrer_points.owner != user_points.owner
    )]
    referrer_points: Option<Account<'info, TokenAccount>>,
    #[account(mut, constraint = points_mint.key() == POINTS_MINT.key())]
    points_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = points_mint,
        token::authority = points_pda
    )]
    points_acc: Account<'info, TokenAccount>,
    #[account(mut)]
    owner: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [POINTS_PDA], bump)]
    points_pda: AccountInfo<'info>,
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

    fn send_user_points(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.points_acc.to_account_info(),
            to: self.user_points.to_account_info(),
            authority: self.points_pda.to_account_info(),
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

    let point_pda: &[&[u8]] = &[POINTS_PDA, &[ctx.bumps.points_pda]];

    let point_pda_seeds = &[&point_pda[..]];

    let available_points_amt = accs.points_acc.amount;

    let points = get_swap_points(
        available_points_amt,
        swap_amount.amount_in + swap_amount.admin_fee_in,
    );
    let clamped_points = min(available_points_amt, points);
    if clamped_points > 0 {
        token::transfer(
            accs.send_user_points().with_signer(point_pda_seeds),
            clamped_points,
        )
        .unwrap();

        if let Some(referrer) = &mut accs.referrer_points {
            let available_points_amt = if available_points_amt > clamped_points {
                available_points_amt - clamped_points
            } else {
                0
            };
            let referrer_points = clamped_points.mul_div_floor(25_000, 100_000).unwrap();
            let clamped_referrer_points = min(available_points_amt, referrer_points);

            if clamped_referrer_points > 0 {
                let cpi_accounts = Transfer {
                    from: accs.points_acc.to_account_info(),
                    to: referrer.to_account_info(),
                    authority: accs.points_pda.to_account_info(),
                };

                let cpi_program = accs.token_program.to_account_info();

                token::transfer(
                    CpiContext::new(cpi_program, cpi_accounts).with_signer(point_pda_seeds),
                    clamped_referrer_points,
                )
                .unwrap();
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

fn get_swap_points(current_available: u64, buy_amount: u64) -> u64 {
    let current_points = MAX_POINTS_AVAILABLE - current_available;
    let current_sol = get_sol_for_points(current_points);
    let next_points = get_points_for_sol(current_sol + buy_amount);

    msg!(
        "curp {} curs {} nexp {}",
        current_points,
        current_sol,
        next_points
    );

    if next_points > current_points {
        return next_points - current_points;
    }
    return 0;
}

fn get_points_for_sol(sol_amount: u64) -> u64 {
    if sol_amount < BOOSTED_SOL_AMOUNT {
        return sol_amount
            .mul_div_floor(BOOSTED_POINTS_AMOUNT, BOOSTED_SOL_AMOUNT)
            .unwrap();
    }

    return BOOSTED_POINTS_AMOUNT + (sol_amount - BOOSTED_SOL_AMOUNT);
}

fn get_sol_for_points(points_amount: u64) -> u64 {
    if points_amount < BOOSTED_POINTS_AMOUNT {
        return points_amount
            .mul_div_floor(BOOSTED_SOL_AMOUNT, BOOSTED_POINTS_AMOUNT)
            .unwrap();
    }

    return BOOSTED_SOL_AMOUNT + (points_amount - BOOSTED_POINTS_AMOUNT);
}
