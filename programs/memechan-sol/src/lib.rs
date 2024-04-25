mod curve;
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
use std::cmp::min;
use std::mem;

declare_id!("Bos3FKqnNf725J46YBRvxCribD22RLkCreJvjB9WLdgq");

pub mod admin {
    use anchor_lang::prelude::declare_id;
    #[cfg(not(feature = "devnet"))]
    declare_id!("Bos3FKqnNf725J46YBRvxCribD22RLkCreJvjB9WLdgq");
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

    pub fn go_live(ctx: Context<GoLive>) -> Result<()> {
        go_live_handler(ctx)
    }

    pub fn add_fees(ctx: Context<AddFees>) -> Result<()> {
        add_fees_handler(ctx)
    }

    pub fn unstake(ctx: Context<Unstake>, release_amount: u64) -> Result<()> {
        unstake_handler(ctx, release_amount)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        withdraw_fees_handler(ctx)
    }

    pub fn merge_tickets(ctx: Context<MergeTickets>) -> Result<()> {
        merge_tickets_handler(ctx)
    }
}

const MEME_TOKEN_DECIMALS: u64 = 1_000_000;
const WSOL_DECIMALS: u64 = 1_000_000_000;
const MAX_TICKET_TOKENS: u64 = 900_000_000;
const MAX_MEME_TOKENS: u64 = 1_125_000_000;

const MAX_WSOL: u64 = 300;

#[account]
pub struct BoundPool {
    pub meme_amt: u64,
    pub sol_reserve: Reserve,
    pub admin_fees_meme: u64,
    pub admin_fees_sol: u64,
    pub admin_vault_sol: Pubkey,
    pub launch_token_vault: Pubkey,
    pub fees: Fees,
    pub locked: bool,
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
        let meme = mem::size_of::<Reserve>();
        let sol = mem::size_of::<Reserve>();
        let admin_fees_ticket = 8;
        let admin_fees_sol = 8;
        let admin_vault_ticket = 32;
        let admin_vault_sol = 32;
        let launch_token_vault = 32;
        let fees = mem::size_of::<Fees>();
        let locked = mem::size_of::<bool>();

        discriminant
            + meme
            + sol
            + admin_fees_ticket
            + admin_fees_sol
            + admin_vault_ticket
            + admin_vault_sol
            + launch_token_vault
            + fees
            + locked
    }
}
#[derive(Accounts)]
pub struct New<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(init, payer = sender, space = BoundPool::space())]
    pub pool: Account<'info, BoundPool>,
    #[account(
        mut,
        constraint = meme_mint.mint_authority == COption::Some(pool_signer_pda.key())
            @ err::acc("meme mint authority must be the pool signer"),
        constraint = meme_mint.freeze_authority == COption::None
            @ err::acc("meme mint mustn't have a freeze authority"),
    )]
    pub meme_mint: Account<'info, Mint>,
    #[account(
        constraint = sol_vault.mint == sol_mint.key()
            @ err::acc("ticket vault must be of ticket mint"),
        constraint = sol_vault.owner == pool_signer_pda.key()
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
        constraint = launch_vault.owner == pool_signer_pda.key()
            @ err::acc("launch vault authority must match admin"),
    )]
    pub launch_vault: Account<'info, TokenAccount>,
    /// CHECK: pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> New<'info> {
    fn mint_meme_tokens(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_accounts = token::MintTo {
            mint: self.meme_mint.to_account_info(),
            to: self.launch_vault.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn set_mint_authority(
        &self,
        mint: &Account<'info, Mint>,
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            current_authority: self.pool_signer_pda.to_account_info(),
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

    token::mint_to(
        accs.mint_meme_tokens(),
        MAX_MEME_TOKENS * MEME_TOKEN_DECIMALS,
    )
    .unwrap();

    token::set_authority(accs.set_mint_authority(&accs.meme_mint), MintTokens, None).unwrap();

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
    pub user_meme_ticket: Account<'info, MemeTicket>,
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
    pub pool_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> SwapCoinX<'info> {
    fn send_tokens_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.user_sol.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
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

    let user_ticket = &mut accs.user_meme_ticket;

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

    let swap_amount = swap_amounts(pool_state, coin_in_amount, coin_y_min_value, true);

    pool_state.admin_fees_meme += swap_amount.admin_fee_in;
    pool_state.admin_fees_sol += swap_amount.admin_fee_out;

    user_ticket.amount -= coin_in_amount;
    pool_state.meme_amt += swap_amount.amount_in;

    token::transfer(accs.send_tokens_to_user(), swap_amount.amount_out).unwrap();

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

    let swap_amount = swap_amounts(&accs.pool, coin_in_amount, coin_x_min_value, false);

    token::transfer(accs.send_user_tokens(), swap_amount.amount_in).unwrap();

    let pool = &mut accs.pool;

    pool.admin_fees_sol += swap_amount.admin_fee_in;
    pool.admin_fees_meme += swap_amount.admin_fee_out;

    //events::swap<CoinY, CoinX, SwapAmount>(pool_address, coin_in_amount,swap_amount);

    if pool.meme_amt == 0 {
        pool.locked = true;
    };

    //coin::take(&mut pool_state.balance_x, swap_amount.amount_out, ctx)
    let swap_amount = swap_amount.amount_out;

    let meme_ticket = &mut accs.meme_ticket;

    meme_ticket.setup(pool.key(), accs.owner.key(), swap_amount);

    return Ok(());
}

#[derive(Accounts)]
pub struct CloseTicket<'info> {
    pub staking: Account<'info, StakingPool>,
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
pub struct MergeTickets<'info> {
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

pub fn merge_tickets_handler(ctx: Context<MergeTickets>) -> Result<()> {
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

fn swap_amounts(
    pool_state: &Account<BoundPool>,
    coin_in_amount: u64,
    coin_out_min_value: u64,
    is_x: bool,
) -> SwapAmount {
    let balance_x = pool_state.meme_amt;
    let balance_y = pool_state.sol_reserve.tokens;
    let prev_k = curve::invariant(balance_x, balance_y).unwrap();

    let admin_fee_in = get_fee_in_amount(&pool_state.fees, coin_in_amount).unwrap();

    let coin_in_amount = {
        if is_x {
            min(
                coin_in_amount - admin_fee_in,
                (MAX_TICKET_TOKENS * MEME_TOKEN_DECIMALS) - balance_x,
            )
        } else {
            min(
                coin_in_amount - admin_fee_in,
                (MAX_WSOL * WSOL_DECIMALS) - balance_y,
            )
        }
    };

    let amount_out = curve::get_amount_out(coin_in_amount, balance_x, balance_y, is_x).unwrap();

    let admin_fee_out = get_fee_out_amount(&pool_state.fees, amount_out).unwrap();

    let amount_out = amount_out - admin_fee_out;

    assert!(amount_out >= coin_out_min_value, "slippage");

    let new_k = {
        if is_x {
            curve::invariant(
                balance_x + coin_in_amount + admin_fee_in,
                balance_y - amount_out,
            )
        } else {
            curve::invariant(
                balance_x - amount_out,
                balance_y + coin_in_amount + admin_fee_in,
            )
        }
    }
    .unwrap();

    assert!(new_k >= prev_k, "invalid_invariant");

    SwapAmount {
        amount_in: coin_in_amount,
        amount_out,
        admin_fee_in,
        admin_fee_out,
    }
}
