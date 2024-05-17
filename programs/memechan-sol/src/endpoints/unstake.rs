use crate::err::AmmError;
use crate::models::fee_distribution::update_stake;
use crate::models::staked_lp::MemeTicket;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
        mut,
        has_one = meme_vault,
        has_one = wsol_vault,
    )]
    staking: Account<'info, StakingPool>,
    #[account(
        mut,
        constraint = meme_ticket.pool == staking.pool,
        constraint = meme_ticket.owner == signer.key()
    )]
    meme_ticket: Account<'info, MemeTicket>,
    #[account(
        mut,
        constraint = user_meme.owner == signer.key()
    )]
    user_meme: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = user_wsol.owner == signer.key()
    )]
    user_wsol: Account<'info, TokenAccount>,
    #[account(mut)]
    meme_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    wsol_vault: Account<'info, TokenAccount>,
    signer: Signer<'info>,
    /// CHECK: checked by AMM
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    staking_signer_pda: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    fn send_wsol_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.wsol_vault.to_account_info(),
            to: self.user_wsol.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_meme_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.meme_vault.to_account_info(),
            to: self.user_meme.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<Unstake>, release_amount: u64) -> Result<()> {
    let accs = ctx.accounts;

    let vesting_data = accs.meme_ticket.vesting;
    let vesting_config = accs.staking.vesting_config;

    let amount_available_to_release =
        vesting_data.to_release(&vesting_config, Clock::get()?.unix_timestamp);

    if release_amount > amount_available_to_release {
        return Err(error!(AmmError::NotEnoughTokensToRelease));
    }

    let withdrawal = update_stake(
        &mut accs.staking,
        &mut accs.meme_ticket,
        vesting_data.current_stake(),
        release_amount,
    )?;

    accs.meme_ticket.vesting.release(release_amount);

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    token::transfer(
        accs.send_meme_to_user().with_signer(staking_signer_seeds),
        withdrawal.max_withdrawal_meme + release_amount,
    )?;

    token::transfer(
        accs.send_wsol_to_user().with_signer(staking_signer_seeds),
        withdrawal.max_withdrawal_wsol,
    )?;

    Ok(())
}
