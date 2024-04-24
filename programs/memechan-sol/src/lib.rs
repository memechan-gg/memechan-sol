mod curve;
mod err;
mod fee_distribution;
mod fees;
mod init;
mod staked_lp;
mod staking;
mod vesting;

use anchor_lang::prelude::*;
use std::cmp::min;

use crate::err::AmmError;
use crate::fees::*;
use crate::init::*;
use crate::staked_lp::*;
use crate::staking::*;
use anchor_lang::solana_program::program_option::COption;
use anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
use anchor_spl::token::spl_token::native_mint;
use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount, Transfer};
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

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        unstake_handler(ctx)
    }

    pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
        withdraw_fees_handler(ctx)
    }
}

const TOKEN_DECIMALS: u64 = 1_000_000;
const WSOL_DECIMALS: u64 = 1_000_000_000;
const MAX_TICKET_TOKENS: u64 = 900_000_000;
const MAX_MEME_TOKENS: u64 = 1_125_000_000;

const MAX_WSOL: u64 = 300;

const FEE: u128 = 1_000_000;

#[account]
pub struct PoolState {
    pub meme_vault: Reserve,
    pub sol_vault: Reserve,
    pub admin_fees_ticket: u64,
    pub admin_fees_sol: u64,
    pub admin_vault_ticket: Pubkey,
    pub admin_vault_sol: Pubkey,
    pub launch_token_vault: Pubkey,
    pub fees: Fees,
    pub locked: bool,
}

impl PoolState {
    pub const SIGNER_PDA_PREFIX: &'static [u8; 6] = b"signer";

    pub fn space() -> usize {
        let discriminant = 8;
        let meme = mem::size_of::<Reserve>();
        let sol = mem::size_of::<Reserve>();
        let admin_vault_ticket = 32;
        let admin_vault_sol = 32;
        let launch_token_vault = 32;
        let fees = mem::size_of::<Fees>();
        let locked = mem::size_of::<bool>();

        discriminant
            + meme
            + sol
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
    #[account(init, payer = sender, space = PoolState::space())]
    pub pool: Account<'info, PoolState>,
    #[account(
    constraint = ticket_vault.mint == ticket_mint.key()
    @ err::acc("ticket vault must be of ticket mint"),
    constraint = ticket_vault.owner == pool_signer_pda.key()
    @ err::acc("ticket vault authority must match pool pda"),
    )]
    pub ticket_vault: Account<'info, TokenAccount>,
    #[account(
    constraint = ticket_mint.mint_authority == COption::Some(pool_signer_pda.key())
    @ err::acc("ticket mint authority must be the pool signer"),
    constraint = ticket_mint.freeze_authority == COption::None
    @ err::acc("ticket mint mustn't have a freeze authority"),
    )]
    pub ticket_mint: Account<'info, Mint>,
    #[account(
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
    constraint = admin_ticket_vault.mint == ticket_mint.key()
    @ err::acc("admin ticket vault must be of ticket mint"),
    constraint = admin_ticket_vault.owner == crate::admin::id()
    @ err::acc("admin ticket vault authority must match admin"),
    )]
    pub admin_ticket_vault: Account<'info, TokenAccount>,
    #[account(
    constraint = admin_sol_vault.mint == sol_mint.key()
    @ err::acc("admin sol vault must be of ticket mint"),
    constraint = admin_sol_vault.owner == crate::admin::id()
    @ err::acc("admin sol vault authority must match admin"),
    )]
    pub admin_sol_vault: Account<'info, TokenAccount>,
    #[account(
    constraint = launch_vault.mint == meme_mint.key()
    @ err::acc("admin ticket vault must be of ticket mint"),
    constraint = launch_vault.owner == pool_signer_pda.key()
    @ err::acc("launch vault authority must match admin"),
    )]
    pub launch_vault: Account<'info, TokenAccount>,
    /// CHECK: pda signer
    #[account(seeds = [PoolState::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer_pda: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> New<'info> {
    fn mint_ticket_tokens(&self) -> CpiContext<'_, '_, '_, 'info, token::MintTo<'info>> {
        let cpi_accounts = token::MintTo {
            mint: self.ticket_mint.to_account_info(),
            to: self.ticket_vault.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

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

    if accs.meme_mint.supply != 0 || accs.ticket_mint.supply != 0 {
        return Err(error!(err::acc("")));
    }

    token::mint_to(
        accs.mint_ticket_tokens(),
        MAX_TICKET_TOKENS * TOKEN_DECIMALS,
    )
    .unwrap();
    token::mint_to(accs.mint_meme_tokens(), MAX_MEME_TOKENS * TOKEN_DECIMALS).unwrap();

    token::set_authority(accs.set_mint_authority(&accs.meme_mint), MintTokens, None).unwrap();
    token::set_authority(accs.set_mint_authority(&accs.ticket_mint), MintTokens, None).unwrap();

    let pool = &mut accs.pool;
    pool.admin_vault_ticket = accs.admin_ticket_vault.key();
    pool.admin_vault_sol = accs.admin_sol_vault.key();
    pool.meme_vault = Reserve {
        tokens: accs.ticket_vault.amount,
        mint: accs.ticket_mint.key(),
        vault: accs.ticket_vault.key(),
    };
    pool.sol_vault = Reserve {
        tokens: 0,
        mint: accs.sol_mint.key(),
        vault: accs.sol_vault.key(),
    };
    pool.fees = fees::Fees {
        fee_in_percent: FEE,
        fee_out_percent: FEE,
    };
    pool.launch_token_vault = accs.launch_vault.key();
    pool.locked = false;

    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Reserve {
    pub tokens: u64,
    pub mint: Pubkey,
    pub vault: Pubkey,
}

pub fn swap_x_handler(
    ctx: Context<SwapCoinX>,
    coin_in_amount: u64,
    coin_y_min_value: u64,
) -> Result<()> {
    let accs = ctx.accounts;

    assert_ne!(coin_in_amount, 0, "no_zero_coin");

    let pool_state = &mut accs.pool;
    assert!(!pool_state.locked, "pool_is_locked");

    let swap_amount = swap_amounts(pool_state, coin_in_amount, coin_y_min_value, true);

    pool_state.admin_fees_sol += swap_amount.admin_fee_out;
    pool_state.admin_fees_ticket += swap_amount.admin_fee_in;

    token::transfer(accs.send_user_tokens(), swap_amount.amount_in).unwrap();

    //balance::join(&mut pool_state.balance_x, token_ir::into_balance(policy, coin_x, ctx));

    //events::swap<CoinX, CoinY, SwapAmount>(pool_address, coin_in_amount, swap_amount);

    //let coin_y = coin::take(&mut pool_state.balance_y, swap_amount.amount_out, ctx);

    token::transfer(accs.send_tokens_to_user(), swap_amount.amount_out).unwrap();

    // We keep track of how much each address ownes of coin_x
    //subtract_from_token_acc(pool, coin_in_amount, sender(ctx));

    Ok(())
}

pub fn swap_y_handler(
    ctx: Context<SwapCoinY>,
    coin_in_amount: u64,
    coin_x_min_value: u64,
) -> Result<()> {
    let accs = ctx.accounts;
    let pool = &mut accs.pool;

    let user_wsol = &mut accs.user_wsol;

    if user_wsol.amount == 0 {
        return Err(error!(AmmError::NoZeroTokens));
    }

    if pool.locked {
        return Err(error!(AmmError::PoolIsLocked));
    }

    let swap_amount = swap_amounts(pool, coin_in_amount, coin_x_min_value, false);

    pool.admin_fees_sol += swap_amount.admin_fee_in;
    pool.admin_fees_ticket += swap_amount.admin_fee_out;

    token::transfer(accs.send_user_tokens(), swap_amount.amount_in).unwrap();

    //events::swap<CoinY, CoinX, SwapAmount>(pool_address, coin_in_amount,swap_amount);

    if accs.meme_vault.amount == 0 {
        accs.pool.locked = true;
    };

    //coin::take(&mut pool_state.balance_x, swap_amount.amount_out, ctx)
    let swap_amount = swap_amount.amount_out;

    accs.staked_lp.owner = accs.signer.key();
    accs.staked_lp.amount = swap_amount;
    accs.staked_lp.until_timestamp = Clock::get()?.unix_timestamp + staked_lp::LOCK_TIME;

    //let staked_lp = amm::staked_lp::new(balance::split(&mut pool_state.balance_x, swap_amount));

    // We keep track of how much each address ownes of coin_x
    //add_from_token_acc(pool, swap_amount, sender(ctx));
    return Ok(());
}

pub struct SwapAmount {
    amount_in: u64,
    amount_out: u64,
    admin_fee_in: u64,
    admin_fee_out: u64,
}

#[derive(Accounts)]
pub struct SwapCoinX<'info> {
    pub pool: Account<'info, PoolState>,
    #[account()]
    pub user_meme: Account<'info, TokenAccount>,
    pub meme_coin_vault: Account<'info, TokenAccount>,
    pub user_sol: Account<'info, TokenAccount>,
    pub sol_coin_vault: Account<'info, TokenAccount>,
    pub signer: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [PoolState::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub pool_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> SwapCoinX<'info> {
    fn send_user_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_meme.to_account_info(),
            to: self.meme_coin_vault.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_tokens_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.sol_coin_vault.to_account_info(),
            to: self.user_sol.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
pub struct SwapCoinY<'info> {
    pool: Account<'info, PoolState>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
    user_wsol: Account<'info, TokenAccount>,
    admin_meme_coin: Account<'info, TokenAccount>,
    admin_sol_coin: Account<'info, TokenAccount>,
    staked_lp: Account<'info, StakedLP>,
    signer: Signer<'info>,
    /// CHECK: pda signer
    #[account(seeds = [PoolState::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pool_signer_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> SwapCoinY<'info> {
    fn send_admin_fee(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_user_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.user_wsol.to_account_info(),
            to: self.wsol_vault.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

fn swap_amounts(
    pool_state: &Account<PoolState>,
    coin_in_amount: u64,
    coin_out_min_value: u64,
    is_x: bool,
) -> SwapAmount {
    let balance_x = pool_state.meme_vault.tokens;
    let balance_y = pool_state.sol_vault.tokens;
    let prev_k = curve::invariant(balance_x, balance_y).unwrap();

    let admin_fee_in = fees::get_fee_in_amount(&pool_state.fees, coin_in_amount).unwrap();

    let coin_in_amount = {
        if is_x {
            min(
                coin_in_amount - admin_fee_in,
                (MAX_TICKET_TOKENS * TOKEN_DECIMALS) - balance_x,
            )
        } else {
            min(
                coin_in_amount - admin_fee_in,
                (MAX_WSOL * WSOL_DECIMALS) - balance_y,
            )
        }
    };

    let amount_out = curve::get_amount_out(coin_in_amount, balance_x, balance_y, is_x).unwrap();

    let admin_fee_out = fees::get_fee_out_amount(&pool_state.fees, amount_out).unwrap();

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
