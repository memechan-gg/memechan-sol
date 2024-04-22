use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use crate::fee_distribution::FeeState;
use crate::staked_lp::StakedLP;

#[account]
pub struct FeeState {
    meme_vault: Pubkey,
    wsol_vault: Pubkey,
    vesting_config: VestingConfig,
    stakes_total: u64,
    fees_x_total: u64,
    fees_y_total: u64,
}

#[derive(Accounts)]
struct InitFeeState<'info> {
    fee_state: Account<'info, crate::fee_distribution::FeeState>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
}

pub fn new(ctx: Context<InitFeeState>) -> Result<()> {
    let accs = ctx.accounts;
    let fs = &mut ctx.accounts.fee_state;

    fs.meme_vault = accs.meme_vault.key();
    fs.wsol_vault = accs.wsol_vault.key();
    fs.stakes_total = STAKES_MAX;
    fs.vesting_config = VestingConfig
    fs.fees_x_total = 0;
    fs.fees_y_total = 0;

    Ok(())
}

#[derive(Accounts)]
struct Unstake<'info> {
    fee_state: Account<'info, FeeState>,
    lp_ticket: Account<'info, StakedLP>,
    user_meme_acc: Account<'info, TokenAccount>,
    user_wsol_acc: Account<'info, TokenAccount>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
    signer: AccountInfo<'info>
}

pub fn unstake(
ctx: Context<Unstake>,
) -> Result<()> {

    let vesting_data = ctx.accounts.lp_ticket.vesting_data;

let amount_available_to_release = vesting::to_release(
vesting_data,
&staking_pool.vesting_config,
clock::timestamp_ms(clock)
);

// let release_amount = token::value(&coin_x);
// assert!(release_amount <= amount_available_to_release, 0);
// let vesting_data = table::borrow_mut(&mut staking_pool.vesting_data, sender(ctx));
//
// let vesting_old = vesting::current_stake(vesting_data);
//
// let (balance_meme, balance_sui) = fee_distribution::update_stake(vesting_old, release_amount, &mut staking_pool.fee_state, ctx);
//
// vesting::release(vesting_data, release_amount);
//
// balance::join(&mut staking_pool.balance_x, token_ir::into_balance(policy, coin_x, ctx));
//
// balance::join(&mut balance_meme, balance::split(&mut staking_pool.balance_meme, release_amount));

// (
// coin::from_balance(balance_meme, ctx),
// coin::from_balance(balance_sui, ctx)
// )

Ok(())
}

#[derive(Accounts)]
struct WithdrawFees<'info> {
    fee_state: Account<'info, FeeState>,
    lp_ticket: Account<'info, StakedLP>,
    signer: AccountInfo<'info>,
}

pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {

let vesting_data = ctx.accounts.lp_ticket.vesting_data;

let (balance_meme, balance_sui) = fee_distribution::withdraw(&mut staking_pool.fee_state, vesting::current_stake(vesting_data), ctx);

// (
// coin::from_balance(balance_meme, ctx),
// coin::from_balance(balance_sui, ctx)
// )
    Ok(())
}

pub fn add_fees<CoinX: key, Meme: key, LP: key>(staking_pool: &mut StakingPool, coin_meme: Coin<Meme>, coin_sui: Coin<LP>) {
fee_distribution::add_fees(&mut staking_pool.fee_state, coin_meme, coin_sui);
}