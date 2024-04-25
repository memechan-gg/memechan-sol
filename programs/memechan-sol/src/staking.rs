use crate::err::AmmError;
use crate::fee_distribution::{calc_withdraw, update_stake};
use crate::staked_lp::MemeTicket;
use crate::vesting::VestingConfig;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};
use std::mem;

#[account]
pub struct StakingPool {
    pub pool: Pubkey,
    pub meme_vault: Pubkey,
    pub wsol_vault: Pubkey,
    pub vesting_config: VestingConfig,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
}

impl StakingPool {
    pub const SIGNER_PDA_PREFIX: &'static [u8; 7] = b"staking";

    pub fn space() -> usize {
        let discriminant = 8;
        let pool = 32;
        let meme_vault = 32;
        let wsol_vault = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let stakes_total = 8;
        let fees_x_total = 8;
        let fees_y_total = 8;

        discriminant
            + pool
            + meme_vault
            + wsol_vault
            + vesting_config
            + stakes_total
            + fees_x_total
            + fees_y_total
    }
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    staking: Account<'info, StakingPool>,
    meme_ticket: Account<'info, MemeTicket>,
    user_meme: Account<'info, TokenAccount>,
    user_wsol: Account<'info, TokenAccount>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
    signer: Signer<'info>,
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pool_signer_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    fn send_wsol_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.wsol_vault.to_account_info(),
            to: self.user_wsol.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_meme_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.meme_vault.to_account_info(),
            to: self.user_meme.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn unstake_handler(ctx: Context<Unstake>, release_amount: u64) -> Result<()> {
    let accs = ctx.accounts;

    let vesting_data = accs.meme_ticket.vesting;
    let vesting_config = accs.staking.vesting_config;

    let amount_available_to_release =
        vesting_data.to_release(&vesting_config, Clock::get()?.unix_timestamp);

    // let release_amount = token::value(&coin_x);
    // assert!(release_amount <= amount_available_to_release, 0);
    if release_amount > amount_available_to_release {
        return Err(error!(AmmError::NotEnoughTokensToRelease));
    }

    // let vesting_data = table::borrow_mut(&mut staking_pool.vesting_data, sender(ctx));
    //
    // let vesting_old = vesting::current_stake(vesting_data);
    //
    // let (balance_meme, balance_sui) = fee_distribution::update_stake(vesting_old, release_amount, &mut staking_pool.fee_state, ctx);

    let withdrawal = update_stake(
        &mut accs.staking,
        &mut accs.meme_ticket,
        vesting_data.current_stake(),
        release_amount,
    )?;

    // vesting::release(vesting_data, release_amount);
    //

    accs.meme_ticket.vesting.release(release_amount);

    // balance::join(&mut staking_pool.balance_x, token_ir::into_balance(policy, coin_x, ctx));
    //
    // balance::join(&mut balance_meme, balance::split(&mut staking_pool.balance_meme, release_amount));

    // (
    // coin::from_balance(balance_meme, ctx),
    // coin::from_balance(balance_sui, ctx)
    // )

    token::transfer(
        accs.send_meme_to_user(),
        withdrawal.max_withdrawal_meme + release_amount,
    )?;

    token::transfer(accs.send_wsol_to_user(), withdrawal.max_withdrawal_wsol)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    pub staking: Account<'info, StakingPool>,
    pub lp_ticket: Account<'info, MemeTicket>,
    pub user_meme_acc: Account<'info, TokenAccount>,
    pub user_wsol_acc: Account<'info, TokenAccount>,
    pub meme_vault: Account<'info, TokenAccount>,
    pub wsol_vault: Account<'info, TokenAccount>,
    /// CHECK: pda signer
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub pool_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFees<'info> {
    fn send_wsol_fees_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.wsol_vault.to_account_info(),
            to: self.user_wsol_acc.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_meme_fees_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.meme_vault.to_account_info(),
            to: self.user_meme_acc.to_account_info(),
            authority: self.pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn withdraw_fees_handler(ctx: Context<WithdrawFees>) -> Result<()> {
    let accs = ctx.accounts;
    let staking = &mut accs.staking;
    let lp_ticket = &mut accs.lp_ticket;

    let withdrawal = calc_withdraw(staking, lp_ticket).unwrap();

    lp_ticket.withdraws_meme += withdrawal.max_withdrawal_meme;
    lp_ticket.withdraws_wsol += withdrawal.max_withdrawal_wsol;

    token::transfer(
        accs.send_meme_fees_to_user(),
        withdrawal.max_withdrawal_meme,
    )
    .unwrap();

    token::transfer(
        accs.send_wsol_fees_to_user(),
        withdrawal.max_withdrawal_wsol,
    )
    .unwrap();
    // (
    // coin::from_balance(balance_meme, ctx),
    // coin::from_balance(balance_sui, ctx)
    // )
    Ok(())
}

#[derive(Accounts)]
pub struct AddFees<'info> {
    pub staking: Account<'info, StakingPool>,
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

pub fn add_fees_handler(ctx: Context<AddFees>) -> Result<()> {
    let accs = ctx.accounts;
    let state = &mut accs.staking;
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

    Ok(())
}
