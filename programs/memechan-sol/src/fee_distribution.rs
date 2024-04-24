use crate::err::AmmError;
use crate::staked_lp::StakedLP;
use crate::staking::StakingPool;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

const PRECISION: u128 = 1_000_000_000_000_000;

#[derive(Accounts)]
struct AddFees<'info> {
    fee_state: Account<'info, StakingPool>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
    meme_fees: Account<'info, TokenAccount>,
    wsol_fees: Account<'info, TokenAccount>,
    signer: Signer<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> AddFees<'info> {
    fn send_fees(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn add_fees(ctx: Context<AddFees>) {
    let accs = ctx.accounts;
    let state = &mut accs.fee_state;
    state.fees_x_total += accs.meme_fees.amount;
    state.fees_y_total += accs.wsol_fees.amount;

    transfer(
        accs.send_fees(&accs.meme_fees, &accs.meme_vault),
        accs.meme_fees.amount,
    )
    .unwrap();
    transfer(
        accs.send_fees(&accs.wsol_fees, &accs.wsol_vault),
        accs.meme_fees.amount,
    )
    .unwrap();
}

pub struct Withdrawal {
    pub max_withdrawal_meme: u64,
    pub max_withdrawal_wsol: u64,
}

pub fn calc_withdraw(
    fee_state: &Account<StakingPool>,
    lp_ticket: &Account<StakedLP>,
) -> Result<Withdrawal> {
    let user_stake: u64 = lp_ticket.amount;
    let user_withdrawals_meme = lp_ticket.withdraws_meme;
    let user_withdrawals_wsol = lp_ticket.withdraws_wsol;

    let max_withdrawal_meme = get_max_withdraw(
        user_withdrawals_meme,
        fee_state.fees_x_total,
        user_stake,
        fee_state.stakes_total,
    )
    .unwrap();

    let max_withdrawal_wsol = get_max_withdraw(
        user_withdrawals_wsol,
        fee_state.fees_y_total,
        user_stake,
        fee_state.stakes_total,
    )
    .unwrap();

    Ok(Withdrawal {
        max_withdrawal_meme,
        max_withdrawal_wsol,
    })
}

pub fn update_stake(
    state: &mut Account<StakingPool>,
    lp_ticket: &mut Account<StakedLP>,
    user_old_stake: u64,
    user_stake_diff: u64,
) -> Result<Withdrawal> {
    let withdrawal = calc_withdraw(state, lp_ticket).unwrap();

    let stake_diff = ((user_stake_diff as u128) * PRECISION) / (user_old_stake as u128);

    let user_withdrawals_x = &mut lp_ticket.withdraws_meme;
    let withdraw_diff_x = get_withdraw_diff(*user_withdrawals_x, stake_diff);
    *user_withdrawals_x -= withdraw_diff_x;

    let user_withdrawals_y = &mut lp_ticket.withdraws_wsol;
    let withdraw_diff_y = get_withdraw_diff(*user_withdrawals_y, stake_diff);
    *user_withdrawals_y = withdraw_diff_y;

    state.stakes_total -= user_stake_diff;

    Ok(withdrawal)
}

fn get_max_withdraw(
    user_withdrawals: u64,
    fees_total: u64,
    user_stake: u64,
    stakes_total: u64,
) -> Result<u64> {
    let (user_withdrawals_total, fees_total, user_stake, stakes_total) = (
        user_withdrawals as u128,
        fees_total as u128,
        user_stake as u128,
        stakes_total as u128,
    );

    let max_user_withdrawal = fees_total * ((user_stake * PRECISION) / stakes_total);

    if max_user_withdrawal <= user_withdrawals_total * PRECISION {
        return Err(error!(AmmError::NoTokensToWithdraw));
    }

    let allowed_withdrawal = max_user_withdrawal - user_withdrawals_total;

    Ok((allowed_withdrawal / PRECISION) as u64)
}

fn get_withdraw_diff(user_withdrawals: u64, stake_diff: u128) -> u64 {
    let withdraw_diff_x = ((user_withdrawals as u128) * stake_diff) / PRECISION;
    withdraw_diff_x as u64
}
