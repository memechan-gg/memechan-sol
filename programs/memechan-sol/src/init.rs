use crate::staking::StakingPool;
use crate::PoolState;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

//const ADMIN_ADDR: address = @0xfff; // TODO

const SUI_THRESHOLD: u64 = 30_000;
const BPS: u64 = 10_000;
const LOCKED: u64 = 8_000;

const A: u128 = 400_000;
const GAMMA: u128 = 145_000_000_000_000;

const ALLOWED_EXTRA_PROFIT: u128 = 2000000000000; // 18 decimals
const ADJUSTMENT_STEP: u128 = 146000000000000; // 18 decimals
const MA_TIME: u128 = 600_000; // 10 minutes

const MID_FEE: u128 = 260_000_000_000_000_000; // (0.26%) swap fee when the pool is balanced
const OUT_FEE: u128 = 450_000_000_000_000_000; // (0.45%) swap fee when the pool is out balance
const GAMMA_FEE: u128 = 200_000_000_000_000; //  (0.0002%) speed rate that fee increases mid_fee => out_fee

/// The amount of Mist per Sui token based on the fact that mist is
/// 10^-9 of a Sui token
const MIST_PER_SUI: u64 = 1_000_000_000;

const LAUNCH_FEE: u128 = 50_000_000_000_000_000; // 5%
const PRECISION: u128 = 1_000_000_000_000_000_000;

pub fn sui(mist: u64) -> u64 {
    MIST_PER_SUI * mist
}

#[derive(Accounts)]
pub struct GoLive<'info> {
    pub pool: Account<'info, PoolState>,
    pub fee_state: Account<'info, StakingPool>,
    pub ticket_vault: Account<'info, TokenAccount>,
    pub sol_vault: Account<'info, TokenAccount>,
    pub admin_ticket_vault: Account<'info, TokenAccount>,
    pub admin_sol_vault: Account<'info, TokenAccount>,
    /// CHECK: pda signer
    #[account(seeds = [PoolState::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> GoLive<'info> {
    fn send_admin_fee_ticket(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.ticket_vault.to_account_info(),
            to: self.admin_ticket_vault.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_admin_fee_sol(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.sol_vault.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn go_live_handler(ctx: Context<GoLive>) -> Result<()> {
    let accs = ctx.accounts;

    if accs.pool.admin_fees_ticket != 0 {
        //balance::join(&mut pool_state.admin_balance_x, token_ir::into_balance(policy, token::split(&mut coin_x, swap_amount.admin_fee_in, ctx), ctx));
        token::transfer(accs.send_admin_fee_ticket(), accs.pool.admin_fees_ticket).unwrap();
    };

    if accs.pool.admin_fees_sol != 0 {
        //balance::join(&mut pool_state.admin_balance_y, balance::split(&mut pool_state.balance_y, swap_amount.admin_fee_out));
        token::transfer(accs.send_admin_fee_sol(), accs.pool.admin_fees_sol).unwrap();
    };

    // let (
    // xmeme_balance,
    // sui_balance,
    // admin_xmeme_balance,
    // admin_sui_balance,
    // meme_balance,
    // _,
    // locked,
    // fields,
    // ) = accs.pool_state;
    //
    // assert!(locked, 0);
    // assert!(balance::value(&xmeme_balance) == 0, 0);
    // balance::destroy_zero(xmeme_balance);
    //
    // // 0. Transfer admin funds to admin
    // transfer::public_transfer(coin::from_balance(admin_xmeme_balance, ctx), ADMIN_ADDR);
    // transfer::public_transfer(coin::from_balance(admin_sui_balance, ctx), ADMIN_ADDR);
    //
    // // 1. Verify if we reached the threshold of SUI amount raised
    // let sui_supply = balance::value(&sui_balance);
    // assert!(sui_supply == sui(SUI_THRESHOLD), 0);
    //
    // // 2. Collect live fees
    // let live_fee_amt = (mul_div_up((sui_supply as u256), LAUNCH_FEE, PRECISION) as u64);
    // transfer::public_transfer(coin::from_balance(balance::split(&mut sui_balance, live_fee_amt), ctx), ADMIN_ADDR);
    //
    // // 3. Split MEME balance amounts into 80/20
    // let meme_supply = balance::value(&meme_balance);
    // let meme_supply_80 = div_mul(meme_supply, BPS, LOCKED);
    //
    // let amm_meme_balance = balance::split(&mut meme_balance, meme_supply_80);
    // let decimals = coin_decimals::new(ctx);
    //
    // coin_decimals::add(&mut decimals, sui_meta);
    // coin_decimals::add(&mut decimals, meme_meta);
    //
    // // 4. Create AMM Pool
    // let (lp_tokens, pool_id) = volatile::new_2_pool(
    // clock,
    // coin::from_balance(sui_balance, ctx), // coin_a
    // coin::from_balance(amm_meme_balance, ctx), // coin_b
    // &decimals,
    // coin::treasury_into_supply(treasury_cap),
    // vector[A, GAMMA],
    // vector[ALLOWED_EXTRA_PROFIT, ADJUSTMENT_STEP, MA_TIME],
    // 100, // price
    // vector[MID_FEE, OUT_FEE, GAMMA_FEE],
    // ctx
    // );
    //
    // // 5. Create staking pool
    // let staking_pool = staking_pool::new<CoinX, Meme, LP>(
    // pool_id,
    // meme_balance,
    // coin::into_balance(lp_tokens),
    // vesting::default_config(clock),
    // fields,
    // ctx,
    // );
    //
    // transfer::public_share_object(staking_pool);
    //
    // // Cleanup
    // coin_decimals::destroy_decimals(coin_decimals::remove<SUI>(&mut decimals));
    // coin_decimals::destroy_decimals(coin_decimals::remove<Meme>(&mut decimals));
    //
    // coin_decimals::destroy_coin_decimals(decimals);

    Ok(())
}
