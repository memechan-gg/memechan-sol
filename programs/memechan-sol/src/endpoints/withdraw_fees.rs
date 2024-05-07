use crate::models::fee_distribution::calc_withdraw;
use crate::models::staked_lp::MemeTicket;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        has_one = meme_vault,
        has_one = wsol_vault,
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(
        mut,
        constraint = meme_ticket.pool == staking.pool,
        constraint = meme_ticket.owner == signer.key()
    )]
    pub meme_ticket: Account<'info, MemeTicket>,
    #[account(
        constraint = user_meme.owner == signer.key()
    )]
    pub user_meme: Account<'info, TokenAccount>,
    #[account(
        constraint = user_wsol.owner == signer.key()
    )]
    pub user_wsol: Account<'info, TokenAccount>,
    #[account(mut)]
    pub meme_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub wsol_vault: Account<'info, TokenAccount>,
    /// CHECK: pda signer
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFees<'info> {
    fn send_wsol_fees_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.wsol_vault.to_account_info(),
            to: self.user_wsol.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_meme_fees_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.meme_vault.to_account_info(),
            to: self.user_meme.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<WithdrawFees>) -> Result<()> {
    let accs = ctx.accounts;
    let staking = &mut accs.staking;
    let lp_ticket = &mut accs.meme_ticket;

    let withdrawal = calc_withdraw(staking, lp_ticket).unwrap();

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    lp_ticket.withdraws_meme += withdrawal.max_withdrawal_meme;
    lp_ticket.withdraws_wsol += withdrawal.max_withdrawal_wsol;

    token::transfer(
        accs.send_meme_fees_to_user()
            .with_signer(staking_signer_seeds),
        withdrawal.max_withdrawal_meme,
    )?;

    token::transfer(
        accs.send_wsol_fees_to_user()
            .with_signer(staking_signer_seeds),
        withdrawal.max_withdrawal_wsol,
    )?;

    Ok(())
}
