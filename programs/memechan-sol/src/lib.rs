mod err;
mod fee_distribution;
mod fees;
mod init;
mod libraries;
mod staked_lp;
mod staking;
mod vesting;

use crate::err::AmmError;
use crate::fees::*;
use crate::init::*;
use crate::staked_lp::*;
use crate::staking::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount, Transfer};
use core as core_;
use num_integer::Roots;
use std::cmp::{max, min};
use std::mem;

declare_id!("3LpdC7WHSrw2d6mWm3Enfvpzy1u5zoHkysyH1WxdmpPB");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    declare_id!("8vBA2MzaQdt3UWimSkx1J4m2zMgp8A2iwtRKzXVurXP2");
}

#[program]
pub mod memechan_sol {
    use super::*;

    pub fn new(ctx: Context<New>) -> Result<()> {
        new_handler(ctx)
    }

    pub fn swap_x(
        ctx: Context<SwapCoinX>,
        coin_in_amount: u64,
        coin_y_min_value: u64,
    ) -> Result<()> {
        swap_x_handler(ctx, coin_in_amount, coin_y_min_value)
    }

    pub fn swap_y(
        ctx: Context<SwapCoinY>,
        coin_in_amount: u64,
        coin_x_min_value: u64,
    ) -> Result<()> {
        swap_y_handler(ctx, coin_in_amount, coin_x_min_value)
    }

    pub fn go_live<'info>(ctx: Context<'_, '_, '_, 'info, GoLive<'info>>) -> Result<()> {
        go_live_handler(ctx)
    }

    pub fn add_fees<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
        add_fees_handler(ctx)
    }

    pub fn unstake(ctx: Context<Unstake>, release_amount: u64) -> Result<()> {
        unstake_handler(ctx, release_amount)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        withdraw_fees_handler(ctx)
    }

    pub fn bound_merge_tickets(ctx: Context<BoundMergeTickets>) -> Result<()> {
        bound_merge_tickets_handler(ctx)
    }

    pub fn staking_merge_tickets(ctx: Context<StakingMergeTickets>) -> Result<()> {
        staking_merge_tickets_handler(ctx)
    }

    pub fn close_ticket(ctx: Context<CloseTicket>) -> Result<()> {
        close_ticket_handler(ctx)
    }
}

const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
const WSOL_DECIMALS: u64 = 1_000_000_000;
const MAX_TICKET_TOKENS: u64 = 900_000_000;
const MAX_MEME_TOKENS: u64 = 1_125_000_000;

const DEFAULT_PRICE_FACTOR: u64 = 2;
const DEFAULT_MAX_M_LP: u128 = 200_000_000_000_000;
const DEFAULT_MAX_M: u128 = 900_000_000_000_000;
const DEFAULT_MAX_S: u128 = 300;

const DECIMALS_ALPHA: u128 = 1_000_000; // consider increase
const DECIMALS_BETA: u128 = 1_000_000; // consider increase
const DECIMALS_S: u128 = 1_000_000_000;

#[account]
pub struct BoundPool {
    pub meme_amt: u64,
    pub meme_mint: Pubkey,
    pub sol_reserve: Reserve,
    pub admin_fees_meme: u64,
    pub admin_fees_sol: u64,
    pub admin_vault_sol: Pubkey,
    pub launch_token_vault: Pubkey,
    pub fees: Fees,
    pub config: Config,
    pub locked: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Config {
    alpha_abs: u128, // |alpha|, because alpha is negative
    beta: u128,
    price_factor: u64,
    // In sui denomination
    gamma_s: u64,
    // In raw denomination
    gamma_m: u64, // DEFAULT_MAX_M * DECIMALS_M = 900_000_000_000_000
    // In raw denomination
    omega_m: u64, // DEFAULT_MAX_M_LP * DECIMALS_M = 200_000_000_000_000
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Reserve {
    pub tokens: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
}

impl BoundPool {
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        let discriminant = 8;
        let meme_amt = 8;
        let meme_mint = 32;
        let sol_reserve = mem::size_of::<Reserve>();
        let admin_fees_meme = 8;
        let admin_fees_sol = 8;
        let admin_vault_sol = 32;
        let launch_token_vault = 32;
        let fees = mem::size_of::<Fees>();
        let config = mem::size_of::<Config>();
        let locked = 1;

        discriminant
            + meme_amt
            + meme_mint
            + sol_reserve
            + admin_fees_meme
            + admin_fees_sol
            + admin_vault_sol
            + launch_token_vault
            + fees
            + config
            + locked
    }
}
#[derive(Accounts)]
pub struct New<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = BoundPool::space()
    )]
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = meme_mint.mint_authority == COption::Some(pool_signer.key())
            @ err::acc("meme mint authority must be the pool signer"),
        constraint = meme_mint.freeze_authority == COption::None
            @ err::acc("meme mint mustn't have a freeze authority"),
    )]
    pub meme_mint: Account<'info, Mint>,
    #[account(
        constraint = sol_vault.mint == sol_mint.key()
            @ err::acc("ticket vault must be of ticket mint"),
        constraint = sol_vault.owner == pool_signer.key()
            @ err::acc("ticket vault authority must match pool pda"),
    )]
    pub sol_vault: Account<'info, TokenAccount>,
    #[account(
        constraint = sol_mint.key() == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    pub sol_mint: Account<'info, Mint>,
    #[account(
        constraint = admin_sol_vault.mint == sol_mint.key()
            @ err::acc("admin sol vault must be of sol mint"),
        constraint = admin_sol_vault.owner == crate::admin::id()
            @ err::acc("admin sol vault authority must match admin"),
    )]
    pub admin_sol_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = launch_vault.mint == meme_mint.key()
            @ err::acc("admin ticket vault must be of ticket mint"),
        constraint = launch_vault.owner == pool_signer.key()
            @ err::acc("launch vault authority must match admin"),
    )]
    pub launch_vault: Account<'info, TokenAccount>,
    /// CHECK: pool_pda
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> New<'info> {
    fn mint_meme_tokens(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_accounts = token::MintTo {
            mint: self.meme_mint.to_account_info(),
            to: self.launch_vault.to_account_info(),
            authority: self.pool_signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn set_mint_authority(
        &self,
        mint: &Account<'info, Mint>,
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: self.pool_signer.to_account_info(),
            account_or_mint: mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn new_handler(ctx: Context<New>) -> Result<()> {
    let accs = ctx.accounts;

    if accs.meme_mint.supply != 0 {
        return Err(error!(err::acc("")));
    }

    let seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[ctx.bumps.pool_signer],
    ];

    let signer_seeds = &[&seeds[..]];

    token::mint_to(
        accs.mint_meme_tokens().with_signer(signer_seeds),
        MAX_MEME_TOKENS * MEME_TOKEN_DECIMALS,
    )
    .unwrap();

    token::set_authority(
        accs.set_mint_authority(&accs.meme_mint)
            .with_signer(signer_seeds),
        MintTokens,
        None,
    )
    .unwrap();

    let pool = &mut accs.pool;
    pool.admin_vault_sol = accs.admin_sol_vault.key();
    pool.sol_reserve = Reserve {
        tokens: 0,
        mint: accs.sol_mint.key(),
        vault: accs.sol_vault.key(),
    };
    pool.fees = Fees {
        fee_in_percent: FEE,
        fee_out_percent: FEE,
    };

    let gamma_s = DEFAULT_MAX_S;
    let gamma_m = DEFAULT_MAX_M;
    let omega_m = DEFAULT_MAX_M_LP;
    let price_factor = DEFAULT_PRICE_FACTOR;

    pool.config = Config {
        alpha_abs: compute_alpha_abs(gamma_s, gamma_m, omega_m, price_factor).unwrap(),
        beta: compute_beta(gamma_s, gamma_m, omega_m, price_factor).unwrap(),
        gamma_s: gamma_s as u64,
        gamma_m: gamma_m as u64,
        omega_m: omega_m as u64,
        price_factor,
    };

    pool.meme_amt = MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS;
    pool.meme_mint = accs.meme_mint.key();
    pool.launch_token_vault = accs.launch_vault.key();
    pool.locked = false;

    Ok(())
}

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
    #[account(
        mut,
        constraint = user_sol.mint == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    pub user_sol: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = pool.sol_reserve.vault == sol_vault.key()
    )]
    pub sol_vault: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> SwapCoinX<'info> {
    fn send_tokens_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.user_sol.to_account_info(),
            authority: self.pool_signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn swap_x_handler(
    ctx: Context<SwapCoinX>,
    coin_in_amount: u64,
    coin_y_min_value: u64,
) -> Result<()> {
    let accs = ctx.accounts;

    if coin_in_amount == 0 {
        return Err(error!(AmmError::NoZeroTokens));
    }

    let user_ticket = &mut accs.meme_ticket;

    msg!(&user_ticket.until_timestamp.to_string());
    msg!(&Clock::get().unwrap().unix_timestamp.to_string());
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

    let swap_amount = swap_amounts(pool_state, coin_in_amount, coin_y_min_value, false);

    pool_state.admin_fees_meme += swap_amount.admin_fee_in;
    pool_state.admin_fees_sol += swap_amount.admin_fee_out;

    pool_state.meme_amt += swap_amount.amount_in;
    pool_state.sol_reserve.tokens -= swap_amount.amount_out + swap_amount.admin_fee_out;

    user_ticket.amount -= coin_in_amount;

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

    Ok(())
}

pub struct SwapAmount {
    amount_in: u64,
    amount_out: u64,
    admin_fee_in: u64,
    admin_fee_out: u64,
}

#[derive(Accounts)]
pub struct SwapCoinY<'info> {
    #[account(mut)]
    pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = pool.sol_reserve.vault == sol_vault.key()
    )]
    sol_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = user_sol.mint == native_mint::id()
            @ err::acc("sol mint should be native WSOL mint")
    )]
    user_sol: Account<'info, TokenAccount>,
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
            from: self.user_sol.to_account_info(),
            to: self.sol_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn swap_y_handler(
    ctx: Context<SwapCoinY>,
    coin_in_amount: u64,
    coin_x_min_value: u64,
) -> Result<()> {
    let accs = ctx.accounts;

    if coin_in_amount == 0 {
        return Err(error!(AmmError::NoZeroTokens));
    }

    if accs.pool.locked {
        return Err(error!(AmmError::PoolIsLocked));
    }

    let swap_amount = swap_amounts(&accs.pool, coin_in_amount, coin_x_min_value, true);

    token::transfer(
        accs.send_user_tokens(),
        swap_amount.amount_in + swap_amount.admin_fee_in,
    )
    .unwrap();

    let pool = &mut accs.pool;

    pool.admin_fees_sol += swap_amount.admin_fee_in;
    pool.admin_fees_meme += swap_amount.admin_fee_out;

    pool.sol_reserve.tokens += swap_amount.amount_in;
    pool.meme_amt -= swap_amount.amount_out + swap_amount.admin_fee_out;

    if pool.meme_amt == 0 {
        pool.locked = true;
    };

    let swap_amount = swap_amount.amount_out;

    let meme_ticket = &mut accs.meme_ticket;

    meme_ticket.setup(pool.key(), accs.owner.key(), swap_amount);

    return Ok(());
}

#[derive(Accounts)]
pub struct CloseTicket<'info> {
    #[account(
        mut,
        has_one = owner,
        close = owner
    )]
    pub ticket: Account<'info, MemeTicket>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn close_ticket_handler(ctx: Context<CloseTicket>) -> Result<()> {
    if ctx.accounts.ticket.amount != 0 {
        return Err(error!(AmmError::NonZeroAmountTicket));
    }

    Ok(())
}

#[derive(Accounts)]
pub struct StakingMergeTickets<'info> {
    pub staking: Account<'info, StakingPool>,
    #[account(
        mut,
        has_one = owner,
        constraint = ticket_into.pool == ticket_from.pool,
        constraint = ticket_into.pool == staking.pool,
        constraint = ticket_into.key() != ticket_from.key()
    )]
    pub ticket_into: Account<'info, MemeTicket>,
    #[account(
        mut,
        close = owner,
        has_one = owner
    )]
    pub ticket_from: Account<'info, MemeTicket>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn staking_merge_tickets_handler(ctx: Context<StakingMergeTickets>) -> Result<()> {
    let accs = ctx.accounts;
    let ticket_into = &mut accs.ticket_into;
    let ticket_from = &mut accs.ticket_from;

    ticket_into.amount += ticket_from.amount;
    ticket_into.withdraws_wsol += ticket_from.withdraws_wsol;
    ticket_into.withdraws_meme += ticket_from.withdraws_meme;
    ticket_into.vesting.notional += ticket_from.vesting.notional;
    ticket_into.vesting.released += ticket_from.vesting.released;

    Ok(())
}

#[derive(Accounts)]
pub struct BoundMergeTickets<'info> {
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        has_one = owner,
        constraint = ticket_into.pool == ticket_from.pool,
        constraint = ticket_into.pool == pool.key(),
        constraint = ticket_into.key() != ticket_from.key()
    )]
    pub ticket_into: Account<'info, MemeTicket>,
    #[account(
        mut,
        close = owner,
        has_one = owner
    )]
    pub ticket_from: Account<'info, MemeTicket>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn bound_merge_tickets_handler(ctx: Context<BoundMergeTickets>) -> Result<()> {
    let accs = ctx.accounts;
    let ticket_into = &mut accs.ticket_into;
    let ticket_from = &mut accs.ticket_from;

    ticket_into.amount += ticket_from.amount;
    ticket_into.withdraws_wsol += ticket_from.withdraws_wsol;
    ticket_into.withdraws_meme += ticket_from.withdraws_meme;
    ticket_into.vesting.notional += ticket_from.vesting.notional;
    ticket_into.vesting.released += ticket_from.vesting.released;
    ticket_into.until_timestamp = max(ticket_into.until_timestamp, ticket_from.until_timestamp);

    Ok(())
}

fn swap_amounts(
    pool_state: &Account<BoundPool>,
    coin_in_amount: u64,
    coin_out_min_value: u64,
    buy_meme: bool,
) -> SwapAmount {
    if buy_meme {
        buy_meme_swap_amounts(pool_state, coin_in_amount, coin_out_min_value).unwrap()
    } else {
        sell_meme_swap_amounts(pool_state, coin_in_amount, coin_out_min_value).unwrap()
    }
}

//
// // We keep track of how much each address ownes of coin_m
// add_from_token_acc(pool, swap_amount, sender(ctx));
// staked_lp
// }
//
// fun new_fees(
// fee_in_percent: u256,
// fee_out_percent: u256,
// ): Fees {
// fees::new(fee_in_percent, fee_out_percent)
// }
//
fn balances(state: &Account<BoundPool>) -> (u64, u64) {
    (state.meme_amt, state.sol_reserve.tokens)
}

fn mist(sui: u64) -> u64 {
    WSOL_DECIMALS * sui
}

fn gamma_s_mist(pool: &Config) -> u64 {
    mist(pool.gamma_s)
}

fn buy_meme_swap_amounts(
    pool: &Account<BoundPool>,
    delta_s: u64,
    min_delta_m: u64,
) -> Result<SwapAmount> {
    let (m_t0, s_t0) = balances(pool);

    let p = &pool.config;

    let max_delta_s = (gamma_s_mist(p)) - s_t0;

    let admin_fee_in = get_fee_in_amount(&pool.fees, delta_s).unwrap();
    let is_max = delta_s - admin_fee_in >= max_delta_s;

    let net_delta_s = min(delta_s - admin_fee_in, max_delta_s);

    let delta_m = if is_max {
        m_t0
    } else {
        compute_delta_m(pool, s_t0, s_t0 + net_delta_s)
    };

    let admin_fee_out = get_fee_out_amount(&pool.fees, delta_m).unwrap();
    let net_delta_m = delta_m - admin_fee_out;

    //assert!(net_delta_m >= min_delta_m, errors::slippage());
    if net_delta_m < min_delta_m {
        return Err(error!(AmmError::SlippageExceeded));
    }

    Ok(SwapAmount {
        amount_in: net_delta_s,
        amount_out: net_delta_m,
        admin_fee_in,
        admin_fee_out,
    })
}

fn sell_meme_swap_amounts(
    pool: &Account<BoundPool>,
    delta_m: u64,
    min_delta_s: u64,
) -> Result<SwapAmount> {
    let (m_b, s_b) = balances(pool);

    let p = &pool.config;

    let max_delta_m = p.gamma_m - m_b; // TODO: confirm

    let admin_fee_in = get_fee_in_amount(&pool.fees, delta_m).unwrap();
    let is_max = delta_m - admin_fee_in > max_delta_m; // TODO: shouldn't it be >=?

    let net_delta_m = min(delta_m - admin_fee_in, max_delta_m);

    let delta_s = if is_max {
        s_b // TODO: confirm
    } else {
        compute_delta_s(pool, s_b, net_delta_m)
    };

    let admin_fee_out = get_fee_out_amount(&pool.fees, delta_s).unwrap();
    let net_delta_s = delta_s - admin_fee_out;

    //assert!(net_delta_s >= min_delta_s, errors::slippage());
    if net_delta_s < min_delta_s {
        return Err(error!(AmmError::SlippageExceeded));
    }

    Ok(SwapAmount {
        amount_in: net_delta_m,
        amount_out: net_delta_s,
        admin_fee_in,
        admin_fee_out,
    })
}

pub fn compute_alpha_abs(
    gamma_s: u128,
    gamma_m: u128,
    omega_m: u128,
    price_factor: u64,
) -> Result<u128> {
    let left = omega_m * (price_factor as u128);
    //assert!(left < gamma_m, EBondingCurveMustBeNegativelySloped);
    if left >= gamma_m {
        return Err(error!(AmmError::BondingCurveMustBeNegativelySloped));
    }

    // We compute |alpha|, hence the subtraction is switched
    Ok((2 * (gamma_m - left) * DECIMALS_ALPHA) / (gamma_s * gamma_s))
}

pub fn compute_beta(
    gamma_s: u128,
    gamma_m: u128,
    omega_m: u128,
    price_factor: u64,
) -> Result<u128> {
    let left = 2 * gamma_m;
    let right = omega_m * (price_factor as u128);
    //assert!(left > right, EBondingCurveInterceptMustBePositive);
    if left <= gamma_m {
        return Err(error!(AmmError::BondingCurveInterceptMustBePositive));
    }

    Ok(((left - right) * DECIMALS_BETA) / gamma_s)
}

pub fn compute_delta_m(pool: &Account<BoundPool>, s_a: u64, s_b: u64) -> u64 {
    let s_a = s_a as u128;
    let s_b = s_b as u128;

    let alpha_abs = &pool.config.alpha_abs;
    let beta = &pool.config.beta;

    let left = *beta * (s_b - s_a) / (DECIMALS_BETA * DECIMALS_S);
    let pow_decimals = DECIMALS_S * DECIMALS_S;
    let right = *alpha_abs * ((s_b * s_b) / pow_decimals - (s_a * s_a) / pow_decimals)
        / (2 * DECIMALS_ALPHA);

    (left - right) as u64
}

pub fn compute_delta_s(
    pool: &Account<BoundPool>,
    // s_a: u64,
    s_b: u64,
    delta_m: u64,
) -> u64 {
    let s_b = s_b as u128;
    let delta_m = delta_m as u128;

    let alpha_abs = pool.config.alpha_abs;
    let beta = pool.config.beta;

    let b_hat_abs = ((2 * beta * DECIMALS_ALPHA * DECIMALS_S)
        - (2 * alpha_abs * s_b * DECIMALS_BETA))
        / (DECIMALS_ALPHA * DECIMALS_BETA * DECIMALS_S);

    // SQRT
    let sqrt_term = ((((b_hat_abs * b_hat_abs) * DECIMALS_ALPHA) + (8 * delta_m * alpha_abs))
        / DECIMALS_ALPHA)
        .sqrt();

    let num = sqrt_term - b_hat_abs;

    ((num * DECIMALS_ALPHA * DECIMALS_S) / (2 * alpha_abs)) as u64
}
