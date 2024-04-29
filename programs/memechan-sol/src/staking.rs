use crate::err::AmmError;
use crate::fee_distribution::{calc_withdraw, update_stake};
use crate::staked_lp::MemeTicket;
use crate::vesting::VestingConfig;
use amm::cpi::accounts::RedeemLiquidity;
use amm::models::{TokenAmount, TokenLimit};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use std::mem;

#[account]
pub struct StakingPool {
    pub pool: Pubkey,
    pub meme_vault: Pubkey,
    pub meme_mint: Pubkey,
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
        let meme_mint = 32;
        let wsol_vault = 32;
        let vesting_config = mem::size_of::<VestingConfig>();
        let stakes_total = 8;
        let fees_x_total = 8;
        let fees_y_total = 8;

        discriminant
            + pool
            + meme_vault
            + meme_mint
            + wsol_vault
            + vesting_config
            + stakes_total
            + fees_x_total
            + fees_y_total
    }
}

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

pub fn unstake_handler(ctx: Context<Unstake>, release_amount: u64) -> Result<()> {
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

pub fn withdraw_fees_handler(ctx: Context<WithdrawFees>) -> Result<()> {
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

#[derive(Accounts)]
pub struct AddFees<'info> {
    #[account(
        mut,
        has_one = meme_vault,
        has_one = wsol_vault
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(mut)]
    pub meme_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub wsol_vault: Account<'info, TokenAccount>,
    /// CHECK: pda
    #[account(seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    /// CHECK: done by inner call
    #[account(mut)]
    pub aldrin_pool_acc: AccountInfo<'info>,
    #[account(mut)]
    pub aldrin_lp_mint: Account<'info, Mint>,
    /// CHECK: done by inner call
    pub aldrin_pool_signer: AccountInfo<'info>,
    #[account(mut)]
    pub aldrin_pool_lp_wallet: Account<'info, TokenAccount>,
    /// CHECK: comparing with dep ID
    #[account(
        constraint = aldrin_amm_program.key() == amm::id()
    )]
    pub aldrin_amm_program: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> AddFees<'info> {
    pub fn withdraw_fees_ctx(&self) -> CpiContext<'_, '_, '_, 'info, RedeemLiquidity<'info>> {
        let cpi_program = self.aldrin_amm_program.to_account_info();
        let cpi_accounts = RedeemLiquidity {
            user: self.staking_signer_pda.to_account_info(),
            pool: self.aldrin_pool_acc.to_account_info(),
            pool_signer: self.aldrin_pool_signer.to_account_info(),
            lp_mint: self.aldrin_lp_mint.to_account_info(),
            lp_token_wallet: self.aldrin_pool_lp_wallet.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn add_fees_handler<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    let meme_vault_initial_amt = accs.meme_vault.amount;
    let wsol_vault_initial_amt = accs.wsol_vault.amount;

    amm::cpi::redeem_liquidity(
        accs.withdraw_fees_ctx()
            .with_signer(staking_signer_seeds)
            .with_remaining_accounts(vec![
                ctx.remaining_accounts[0].to_account_info(),
                accs.meme_vault.to_account_info(),
                ctx.remaining_accounts[1].to_account_info(),
                accs.wsol_vault.to_account_info(),
            ]),
        TokenAmount {
            amount: accs.aldrin_pool_lp_wallet.amount,
        },
        vec![
            TokenLimit {
                mint: accs.meme_vault.mint,
                tokens: TokenAmount { amount: 1 },
            },
            TokenLimit {
                mint: accs.wsol_vault.mint,
                tokens: TokenAmount { amount: 1 },
            },
        ],
    )
    .unwrap();

    accs.meme_vault.reload().unwrap();
    accs.wsol_vault.reload().unwrap();

    let state = &mut accs.staking;
    state.fees_x_total += accs.meme_vault.amount - meme_vault_initial_amt;
    state.fees_y_total += accs.wsol_vault.amount - wsol_vault_initial_amt;

    Ok(())
}
