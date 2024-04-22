use anchor_spl::token_interface::spl_pod::solana_program::example_mocks::solana_sdk::account::Account;
use crate::fee_distribution::FeeState;

struct StakingPool{
balance_meme: Balance<Meme>,
balance_lp: Balance<LP>,
balance_x: Balance<CoinX>,

vesting_config: VestingConfig,
fee_state: FeeState,
}

#[derive(Accounts)]
struct Unstake<'info> {
    fee_state: Account<'info, FeeState>,
}

pub fn unstake(
staking_pool: &mut StakingPool,
coin_x: Token<CoinX>,
policy: &TokenPolicy<CoinX>,
clock: &Clock,
ctx: &mut TxContext,
)-> (Coin<Meme>, Coin<LP>) {
let vesting_data = table::borrow(&staking_pool.vesting_data, sender(ctx));

let amount_available_to_release = vesting::to_release(
vesting_data,
&staking_pool.vesting_config,
clock::timestamp_ms(clock)
);

let release_amount = token::value(&coin_x);
assert!(release_amount <= amount_available_to_release, 0);
let vesting_data = table::borrow_mut(&mut staking_pool.vesting_data, sender(ctx));

let vesting_old = vesting::current_stake(vesting_data);

let (balance_meme, balance_sui) = fee_distribution::update_stake(vesting_old, release_amount, &mut staking_pool.fee_state, ctx);

vesting::release(vesting_data, release_amount);

balance::join(&mut staking_pool.balance_x, token_ir::into_balance(policy, coin_x, ctx));

balance::join(&mut balance_meme, balance::split(&mut staking_pool.balance_meme, release_amount));

(
coin::from_balance(balance_meme, ctx),
coin::from_balance(balance_sui, ctx)
)
}

pub fn withdraw_fees<CoinX: key, Meme: key, LP: key>(staking_pool: &mut StakingPool, ctx: &mut TxContext): (Coin<Meme>, Coin<LP>) {

let vesting_data = table::borrow(&staking_pool.vesting_data, sender(ctx));

let (balance_meme, balance_sui) = fee_distribution::withdraw(&mut staking_pool.fee_state, vesting::current_stake(vesting_data), ctx);

(
coin::from_balance(balance_meme, ctx),
coin::from_balance(balance_sui, ctx)
)
}

pub fn add_fees<CoinX: key, Meme: key, LP: key>(staking_pool: &mut StakingPool, coin_meme: Coin<Meme>, coin_sui: Coin<LP>) {
fee_distribution::add_fees(&mut staking_pool.fee_state, coin_meme, coin_sui);
}