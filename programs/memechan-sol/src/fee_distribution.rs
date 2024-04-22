use anchor_lang::prelude::*;
const PRECISION: u256 = 1_000_000_000_000_000;

#[account]
pub struct FeeState {
    fees_x: Balance<CoinX>,
    fees_y: Balance<CoinY>,
    user_withdrawals_x: Table<address, u64>,
    user_withdrawals_y: Table<address, u64>,
    stakes_total: u64,
    fees_x_total: u64,
    fees_y_total: u64,
}

#[derive(Accounts)]
struct InitFeeState<'info> {
    fee_state: Account<'info, FeeState>,
}

pub fn new(ctx: Context<InitFeeState>) -> Result<()> {
    let fs = &mut ctx.accounts.fee_state;

    fs.fees_x = 0;
    fs.fees_y = 0;
    fs.stakes_total = STAKES_MAX;
    fs.fees_x_total = 0;
    fs.fees_y_total = 0;

    Ok(())
}

#[derive(Accounts)]
struct AddFees<'info> {
    fee_state: Account<'info, FeeState>,
}

pub fn add_fees(ctx: Context<AddFees>) {
    state.fees_x_total = state.fees_x_total + coin::value(&coinX);
    state.fees_y_total = state.fees_y_total + coin::value(&coinY);

    balance::join(&mut state.fees_x, coin::into_balance(coinX));
    balance::join(&mut state.fees_y, coin::into_balance(coinY));
}

#[derive(Accounts)]
struct WithdrawStake<'info> {
    fee_state: Account<'info, FeeState>,
}

pub fn withdraw(ctx: Context<WithdrawStake>) -> Result<()> {
    let sender = tx_context::sender(ctx);

    let user_withdrawals_x = table::borrow_mut(&mut state.user_withdrawals_x, sender);
    let max_withdrawal_x = get_max_withdraw(
        *user_withdrawals_x,
        state.fees_x_total,
        user_stake,
        state.stakes_total,
    );
    *user_withdrawals_x = ((*user_withdrawals_x + max_withdrawal_x) as u64);

    let user_withdrawals_y = table::borrow_mut(&mut state.user_withdrawals_y, sender);
    let max_withdrawal_y = get_max_withdraw(
        *user_withdrawals_y,
        state.fees_y_total,
        user_stake,
        state.stakes_total,
    );
    *user_withdrawals_y = ((*user_withdrawals_y + max_withdrawal_y) as u64);

    (
        balance::split(&mut state.fees_x, max_withdrawal_x),
        balance::split(&mut state.fees_y, max_withdrawal_y),
    );
}

pub fn update_stake(
    ctx: &Context<UpdateStake>,
    user_old_stake: u64,
    user_stake_diff: u64,
    state: &mut FeeState,
) -> (Balance<CoinX>, Balance<CoinY>) {
    let (coin_x, coin_y) = withdraw(state, user_old_stake, ctx);

    let sender = tx_context::sender(ctx);

    let stake_diff = ((user_stake_diff as u256) * PRECISION) / (user_old_stake as u256);

    let user_withdrawals_x = table::borrow_mut(&mut state.user_withdrawals_x, sender);
    let withdraw_diff_x = get_withdraw_diff(*user_withdrawals_x, stake_diff);
    *user_withdrawals_x = *user_withdrawals_x - withdraw_diff_x;

    let user_withdrawals_y = table::borrow_mut(&mut state.user_withdrawals_y, sender);
    let withdraw_diff_y = get_withdraw_diff(*user_withdrawals_y, stake_diff);
    *user_withdrawals_y = *user_withdrawals_y - withdraw_diff_y;

    state.stakes_total = state.stakes_total - user_stake_diff;

    (coin_x, coin_y)
}

fn get_max_withdraw(
    user_withdrawals: u64,
    fees_total: u64,
    user_stake: u64,
    stakes_total: u64,
) -> u64 {
    let (user_withdrawals_total, fees_total, user_stake, stakes_total) = (
        (user_withdrawals as u256),
        (fees_total as u256),
        (user_stake as u256),
        (stakes_total as u256),
    );

    let max_user_withdrawal = fees_total * ((user_stake * PRECISION) / stakes_total);

    assert!(
        max_user_withdrawal <= user_withdrawals_total * PRECISION,
        errors::no_funds_to_withdraw()
    );

    let allowed_withdrawal = max_user_withdrawal - user_withdrawals_total;

    ((allowed_withdrawal / PRECISION) as u64)
}

fn get_withdraw_diff(user_withdrawals: u64, stake_diff: u256) -> u64 {
    let withdraw_diff_x = ((user_withdrawals as u256) * stake_diff) / PRECISION;
    (withdraw_diff_x as u64)
}
