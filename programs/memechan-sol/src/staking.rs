use crate::fee_distribution::calc_withdraw;
use crate::staked_lp::StakedLP;
use crate::vesting::{self, VestingConfig};
use crate::{MAX_TICKET_TOKENS, TOKEN_DECIMALS};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[account]
pub struct FeeState {
    pub meme_vault: Pubkey,
    pub wsol_vault: Pubkey,
    pub vesting_config: VestingConfig,
    pub stakes_total: u64,
    pub fees_x_total: u64,
    pub fees_y_total: u64,
}

#[derive(Accounts)]
struct InitFeeState<'info> {
    fee_state: Account<'info, FeeState>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
}

pub fn init_fee_state(ctx: Context<InitFeeState>) -> Result<()> {
    let accs = ctx.accounts;
    let fs = &mut accs.fee_state;

    fs.meme_vault = accs.meme_vault.key();
    fs.wsol_vault = accs.wsol_vault.key();
    fs.stakes_total = MAX_TICKET_TOKENS * TOKEN_DECIMALS;
    fs.vesting_config = vesting::default_config();
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
    signer: AccountInfo<'info>,
}

pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    let vesting_data = ctx.accounts.lp_ticket.vesting;
    let vesting_config = &ctx.accounts.fee_state.vesting_config;

    let amount_available_to_release =
        vesting_data.to_release(vesting_config, Clock::get().unwrap().unix_timestamp);

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
    user_meme_acc: Account<'info, TokenAccount>,
    user_wsol_acc: Account<'info, TokenAccount>,
    meme_vault: Account<'info, TokenAccount>,
    wsol_vault: Account<'info, TokenAccount>,
    pool_signer_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
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

pub fn withdraw_fees(ctx: Context<WithdrawFees>) -> Result<()> {
    let accs = ctx.accounts;
    let fee_state = &mut accs.fee_state;
    let lp_ticket = &mut accs.lp_ticket;

    let withdrawal = calc_withdraw(fee_state, lp_ticket).unwrap();

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
struct AddFees<'info> {
    fee_state: Account<'info, FeeState>,
}

pub fn add_fees(ctx: Context<AddFees>) -> Result<()> {
    Ok(())
}
