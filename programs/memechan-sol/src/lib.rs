mod err;

use spl_math::uint::*;

use anchor_lang::prelude::*;

use crate::err::acc;
use anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens;
use anchor_spl::token::{self, Mint, SetAuthority, Token, TokenAccount};

declare_id!("Bos3FKqnNf725J46YBRvxCribD22RLkCreJvjB9WLdgq");

#[program]
pub mod memechan_sol {
    use super::*;

    pub fn new(ctx: Context<New>) -> Result<()> {
        new_handler(ctx)
    }
}

const TOKEN_DECIMALS: u64 = 1_000_000;
const MAX_TICKET_TOKENS: u64 = 900_000_000 * TOKEN_DECIMALS;
const MAX_MEME_TOKENS: u64 = 1_125_000_000 * TOKEN_DECIMALS;

const PRECISION: u128 = 1_000_000;
const FEE: u128 = 1_000_000;

pub fn new_handler(ctx: Context<New>) -> Result<()> {
    let accs = ctx.accounts;

    let pool = &mut accs.pool;

    if accs.meme_mint.supply != 0 || accs.ticket_mint.supply != 0 {
        return Err(error!(err::acc("")));
    }

    let res = token::mint_to(accs.mint_ticket_tokens(), MAX_TICKET_TOKENS);
    let res = token::mint_to(accs.mint_meme_tokens(), MAX_MEME_TOKENS);

    let res = token::set_authority(accs.set_mint_authority(&accs.meme_mint), MintTokens, None);
    let res = token::set_authority(accs.set_mint_authority(&accs.ticket_mint), MintTokens, None);

    pool.admin_vault_ticket = accs.admin_ticket_vault.key();
    pool.admin_vault_sol = accs.admin_sol_vault.key();
    pool.meme = Reserve {
        tokens: 0,
        mint: accs.ticket_mint.key(),
        vault: accs.ticket_vault.key(),
    };
    pool.sol = Reserve {
        tokens: 0,
        mint: accs.sol_mint.key(),
        vault: accs.sol_vault.key(),
    };
    pool.fees = Fees {
        fee_in_percent: 0,
        fee_out_percent: 0,
    };
    pool.launch_token_vault = accs.meme_launch_vault.key();
    pool.locked = false;

    Ok(())
}

pub fn swap_x_handler(ctx: Context<SwapCoinX>, coin_y_min_value: u64) -> Result<()> {
    assert!(token::value(&coin_x) != 0, "no_zero_coin");

    let accs = ctx.accounts;

    let pool_address = object::uid_to_address(&pool.id);
    let pool_state = &accs.pool;
    assert!(!pool_state.locked, "pool_is_locked");

    let coin_in_amount = token::value(&coin_x);

    let swap_amount = swap_amounts(pool_state, coin_in_amount, coin_y_min_value, true);

    if swap_amount.admin_fee_in != 0 {
        //balance::join(&mut pool_state.admin_balance_x, token_ir::into_balance(policy, token::split(&mut coin_x, swap_amount.admin_fee_in, ctx), ctx));
        let res = token::transfer(
            accs.send_admin_fee(&accs.meme_coin, &accs.admin_meme_coin),
            swap_amount.admin_fee_in,
        );
    };

    if (swap_amount.admin_fee_out != 0) {
        //balance::join(&mut pool_state.admin_balance_y, balance::split(&mut pool_state.balance_y, swap_amount.admin_fee_out));
        let res = token::transfer(
            accs.send_admin_fee(&accs.sol_coin_vault, &accs.admin_sol_coin),
            swap_amount.admin_fee_out,
        );
    };

    let res = token::transfer(accs.send_user_tokens(), swap_amount.amount_in);

    //balance::join(&mut pool_state.balance_x, token_ir::into_balance(policy, coin_x, ctx));

    //events::swap<CoinX, CoinY, SwapAmount>(pool_address, coin_in_amount, swap_amount);

    //let coin_y = coin::take(&mut pool_state.balance_y, swap_amount.amount_out, ctx);

    let res = token::transfer(accs.send_tokens_to_user(), swap_amount.amount_out);

    // We keep track of how much each address ownes of coin_x
    subtract_from_token_acc(pool, coin_in_amount, sender(ctx));

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct PoolState {
    pub meme: Reserve,
    pub sol: Reserve,
    pub admin_vault_ticket: Pubkey,
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

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Fees {
    fee_in_percent: u128,
    fee_out_percent: u128,
}

pub struct SwapAmount {
    amount_in: u64,
    amount_out: u64,
    admin_fee_in: u64,
    admin_fee_out: u64,
}

#[derive(Accounts)]
pub struct New<'info> {
    pub pool: Account<'info, PoolState>,
    pub ticket_vault: Account<'info, TokenAccount>,
    pub ticket_mint: Account<'info, Mint>,
    pub meme_mint: Account<'info, Mint>,
    pub sol_vault: Account<'info, TokenAccount>,
    pub sol_mint: Account<'info, Mint>,
    pub admin_ticket_vault: Account<'info, TokenAccount>,
    pub admin_sol_vault: Account<'info, TokenAccount>,
    pub meme_launch_vault: Account<'info, TokenAccount>,
    pub pool_signer_pda: AccountInfo<'info>,
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
            to: self.meme_launch_vault.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn set_mint_authority(
        &self,
        mint: &Account<'info, Mint>,
    ) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = token::SetAuthority {
            current_authority: self.pool_signer_pda.to_account_info(),
            account_or_mint: mint.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

// === Private Functions ===

pub struct SwapCoinX<'info> {
    pub pool: Account<'info, PoolState>,
    pub meme_coin: Account<'info, TokenAccount>,
    pub meme_coin_vault: Account<'info, TokenAccount>,
    pub sol_coin: Account<'info, TokenAccount>,
    pub sol_coin_vault: Account<'info, TokenAccount>,
    pub admin_meme_coin: Account<'info, TokenAccount>,
    pub admin_sol_coin: Account<'info, TokenAccount>,
    pub signer: AccountInfo<'info>,
    pub pool_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> SwapCoinX<'info> {
    fn send_admin_fee(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_user_tokens(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self.meme_coin.to_account_info(),
            to: self.meme_coin_vault.to_account_info(),
            authority: signer,
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_tokens_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = token::Transfer {
            from: self.sol_coin_vault.to_account_info(),
            to: self.sol_coin.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub struct SwapCoinY<'info> {
    pool: Account<'info, PoolState>,
    coin_y: Account<'info, TokenAccount>,
    coin_x_min_value: u64,
}

fn swap_amounts(
    pool_state: &Account<PoolState>,
    coin_in_amount: u64,
    coin_out_min_value: u64,
    is_x: bool,
) -> SwapAmount {
    let balance_x = pool_state.meme.tokens;
    let balance_y = pool_state.sol.tokens;
    let prev_k = bound::invariant_(balance_x, balance_y);

    let admin_fee_in = fees::get_fee_in_amount(&pool_state.fees, coin_in_amount);

    let coin_in_amount = {
        if is_x {
            math::min(
                coin_in_amount - admin_fee_in,
                (MAX_X * DECIMALS_X as u64) - balance_x,
            )
        } else {
            math::min(
                coin_in_amount - admin_fee_in,
                (MAX_Y * DECIMALS_Y as u64) - balance_y,
            )
        }
    };

    let amount_out = bound::get_amount_out(coin_in_amount, balance_x, balance_y, is_x);

    let admin_fee_out = fees::get_fee_out_amount(&pool_state.fees, amount_out);

    let amount_out = amount_out - admin_fee_out;

    assert!(amount_out >= coin_out_min_value, "slippage");

    let new_k = {
        if is_x {
            bound::invariant_(
                balance_x + coin_in_amount + admin_fee_in,
                balance_y - amount_out,
            )
        } else {
            bound::invariant_(
                balance_x - amount_out,
                balance_y + coin_in_amount + admin_fee_in,
            )
        }
    };

    assert!(new_k >= prev_k, "invalid_invariant");

    SwapAmount {
        amount_in: coin_in_amount,
        amount_out,
        admin_fee_in,
        admin_fee_out,
    }
}
