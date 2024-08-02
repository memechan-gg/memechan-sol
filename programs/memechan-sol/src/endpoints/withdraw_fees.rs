use crate::consts::{BE_AUTH_KEY, LP_FEE_KEY};
use crate::err::AmmError;
use crate::models::fee_distribution::calc_withdraw;
use crate::models::meme_ticket::MemeTicket;
use crate::models::staking::StakingPool;
use crate::models::user_stats::UserStats;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    /// CHECK: checked by comparing to ticket's field
    pub owner: AccountInfo<'info>,
    #[account(
        has_one = meme_vault,
        has_one = quote_vault,
        has_one = chan_vault
    )]
    pub staking: Box<Account<'info, StakingPool>>,
    #[account(
        mut,
        has_one = owner,
        constraint = meme_ticket.pool == staking.pool,
    )]
    pub meme_ticket: Box<Account<'info, MemeTicket>>,
    #[account(
        mut,
        constraint = user_stats.pool == staking.pool
    )]
    pub user_stats: Option<Box<Account<'info, UserStats>>>,
    #[account(
        mut,
        constraint = user_meme.owner == owner.key()
    )]
    pub user_meme: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_quote.owner == owner.key()
    )]
    pub user_quote: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = user_chan.owner == owner.key()
    )]
    pub user_chan: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub quote_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub chan_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = be_meme.owner == BE_AUTH_KEY.key(),
    )]
    pub be_meme: Option<Box<Account<'info, TokenAccount>>>,
    /// CHECK: pda signer
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFees<'info> {
    fn send_chan_fees_to_user(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.chan_vault.to_account_info(),
            to: self.user_chan.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
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

    fn send_meme_fees_to_backend(
        &self,
        be_meme: AccountInfo<'info>,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.meme_vault.to_account_info(),
            to: be_meme,
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

    if withdrawal.max_withdrawal_meme == 0
        && withdrawal.max_withdrawal_quote == 0
        && withdrawal.max_withdrawal_chan == 0
    {
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
    lp_ticket.withdraws_chan += withdrawal.max_withdrawal_chan;

    msg!(
        "fees_meme: {} fees_quote: {} fees_chan: {}",
        withdrawal.max_withdrawal_meme,
        withdrawal.max_withdrawal_quote,
        withdrawal.max_withdrawal_chan,
    );

    if let Some(user_stats) = &mut accs.user_stats {
        user_stats.meme_received += withdrawal.max_withdrawal_meme;
        user_stats.quote_received += withdrawal.max_withdrawal_quote;
        user_stats.chan_received += withdrawal.max_withdrawal_chan;
    }

    if withdrawal.max_withdrawal_meme > 0 {
        if accs.owner.key().eq(&LP_FEE_KEY.key()) {
            if let Some(optional_account) = &accs.be_meme {
                token::transfer(
                    accs.send_meme_fees_to_backend(optional_account.to_account_info())
                        .with_signer(staking_signer_seeds),
                    withdrawal.max_withdrawal_meme,
                )?;
            } else {
                return Err(error!(AmmError::ShouldProvideBackendVault));
            }
        } else {
            token::transfer(
                accs.send_meme_fees_to_user()
                    .with_signer(staking_signer_seeds),
                withdrawal.max_withdrawal_meme,
            )?;
        }
    }

    if withdrawal.max_withdrawal_quote > 0 {
        token::transfer(
            accs.send_quote_fees_to_user()
                .with_signer(staking_signer_seeds),
            withdrawal.max_withdrawal_quote,
        )?;
    }

    if withdrawal.max_withdrawal_chan > 0 {
        token::transfer(
            accs.send_chan_fees_to_user()
                .with_signer(staking_signer_seeds),
            withdrawal.max_withdrawal_chan,
        )?;
    }

    Ok(())
}
