use crate::err::AmmError;
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
        has_one = quote_vault,
    )]
    pub staking: Box<Account<'info, StakingPool>>,
    #[account(
        mut,
        constraint = meme_ticket.pool == staking.pool,
        constraint = meme_ticket.owner == signer.key()
    )]
    pub meme_ticket: Box<Account<'info, MemeTicket>>,
    #[account(
        mut,
        constraint = user_meme.owner == signer.key()
    )]
    pub user_meme: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_quote.owner == signer.key()
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub quote_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: pda signer
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFees<'info> {
    fn send_quote_fees_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.quote_vault.to_account_info(),
            to: self.user_quote.to_account_info(),
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

    if withdrawal.max_withdrawal_meme == 0 && withdrawal.max_withdrawal_quote == 0 {
        return Err(error!(AmmError::NoTokensToWithdraw));
    }

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    lp_ticket.withdraws_meme += withdrawal.max_withdrawal_meme;
    lp_ticket.withdraws_quote += withdrawal.max_withdrawal_quote;

    msg!(
        "fees_meme: {} fees_quote: {}",
        withdrawal.max_withdrawal_meme,
        withdrawal.max_withdrawal_quote,
    );

    if withdrawal.max_withdrawal_meme > 0 {
        token::transfer(
            accs.send_meme_fees_to_user()
                .with_signer(staking_signer_seeds),
            withdrawal.max_withdrawal_meme,
        )?;
    }

    if withdrawal.max_withdrawal_quote > 0 {
        token::transfer(
            accs.send_quote_fees_to_user()
                .with_signer(staking_signer_seeds),
            withdrawal.max_withdrawal_quote,
        )?;
    }

    Ok(())
}
