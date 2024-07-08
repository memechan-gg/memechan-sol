use crate::models::staking::StakingPool;

use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::consts::{CHAN_MINT, FEE_KEY};
use crate::err::{self, AmmError};
use crate::models::fees::{get_fee_amount, COMMS_FEE};
use dynamic_amm::program::DynamicAmm as MeteoraAmm;
use dynamic_vault::program::DynamicVault as MeteoraVault;

#[derive(Accounts)]
pub struct AddFees<'info> {
    #[account(
        mut,
        has_one = meme_vault,
        constraint = staking.quote_amm_pool == amm_pool.key() || staking.chan_amm_pool == amm_pool.key()
            @ err::acc("amm pool key must be one of the staking's amm pools"),
        constraint = staking.quote_vault == quote_vault.key() || staking.chan_vault == quote_vault.key()
            @ err::acc("quote vault key must be one of the staking's vaults"),
    )]
    pub staking: Account<'info, StakingPool>,
    #[account(mut)]
    pub meme_vault: Box<Account<'info, TokenAccount>>,
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub quote_vault: Box<Account<'info, TokenAccount>>,
    pub quote_mint: Box<Account<'info, Mint>>,
    #[account(mut, token::authority = FEE_KEY)]
    pub meme_fee_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut, token::authority = FEE_KEY)]
    pub quote_fee_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: pda
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_signer_pda: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub amm_pool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub lp_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_token_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_vault_lp: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub a_vault_lp_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_token_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_vault_lp: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub b_vault_lp_mint: AccountInfo<'info>,
    #[account(mut)]
    pub lock_escrow: Box<Account<'info, dynamic_amm::state::LockEscrow>>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub escrow_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: meteora cpi account
    pub source_tokens: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,
    // Programs
    pub token_program: Program<'info, Token>,
    pub amm_program: Program<'info, MeteoraAmm>,
    pub vault_program: Program<'info, MeteoraVault>,
    /// CHECK: Check is done by Raydium inside CPI
    pub memo_program: AccountInfo<'info>,
}

impl<'info> AddFees<'info> {
    pub fn collect_fees(&self, amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let program = self.amm_program.to_account_info();
        let cpi = dynamic_amm::cpi::accounts::ClaimFee {
            owner: self.staking_signer_pda.to_account_info(),
            lp_mint: self.lp_mint.to_account_info(),
            pool: self.amm_pool.to_account_info(),
            a_token_vault: self.a_token_vault.to_account_info(),
            a_vault: self.a_vault.to_account_info(),
            a_vault_lp: self.a_vault_lp.to_account_info(),
            a_vault_lp_mint: self.a_vault_lp_mint.to_account_info(),
            b_token_vault: self.b_token_vault.to_account_info(),
            b_vault: self.b_vault.to_account_info(),
            b_vault_lp: self.b_vault_lp.to_account_info(),
            b_vault_lp_mint: self.b_vault_lp_mint.to_account_info(),
            lock_escrow: self.lock_escrow.to_account_info(),
            escrow_vault: self.escrow_vault.to_account_info(),
            source_tokens: self.source_tokens.to_account_info(),
            user_a_token: self.meme_vault.to_account_info(),
            user_b_token: self.quote_vault.to_account_info(),

            vault_program: self.vault_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(program, cpi, signer_seeds);
        dynamic_amm::cpi::claim_fee(cpi_ctx, amount)
    }

    fn send_meme_comms(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.meme_vault.to_account_info(),
            to: self.meme_fee_vault.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }

    fn send_quote_comms(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.quote_vault.to_account_info(),
            to: self.quote_fee_vault.to_account_info(),
            authority: self.staking_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    if !accs.staking.is_active {
        return Err(error!(AmmError::StakingIsNotActive));
    }

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    let lp_tokens_total = accs.lock_escrow.total_locked_amount;

    let meme_vault_initial_amt = accs.meme_vault.amount;
    let quote_vault_initial_amt = accs.quote_vault.amount;

    accs.collect_fees(lp_tokens_total, staking_signer_seeds)?;

    accs.meme_vault.reload().unwrap();
    accs.quote_vault.reload().unwrap();

    msg!(
        "meme_vault_amt {} meme_vault_initial_amt {} quote_vault_amt {} quote_vault_initial_amt {}",
        accs.meme_vault.amount,
        meme_vault_initial_amt,
        accs.quote_vault.amount,
        quote_vault_initial_amt
    );

    // Calculate fees received and commissions
    let fees_x = accs
        .meme_vault
        .amount
        .checked_sub(meme_vault_initial_amt)
        .unwrap();
    let fees_x_comms = get_fee_amount(fees_x, COMMS_FEE).unwrap();
    let fees_x_no_comms = fees_x.checked_sub(fees_x_comms).unwrap();

    let fees_y = accs
        .quote_vault
        .amount
        .checked_sub(quote_vault_initial_amt)
        .unwrap();
    let fees_y_comms = get_fee_amount(fees_y, COMMS_FEE).unwrap();
    let fees_y_no_comms = fees_y.checked_sub(fees_y_comms).unwrap();

    // Send commissions
    token::transfer(
        accs.send_meme_comms().with_signer(staking_signer_seeds),
        fees_x_comms,
    )?;
    token::transfer(
        accs.send_quote_comms().with_signer(staking_signer_seeds),
        fees_y_comms,
    )?;

    // Mutate the staking
    let state = &mut accs.staking;

    state.fees_x_total += fees_x_no_comms;

    if accs.quote_mint.key() == CHAN_MINT {
        state.fees_z_total += fees_y_no_comms;
    } else {
        state.fees_y_total += fees_y_no_comms;
    }

    Ok(())
}
