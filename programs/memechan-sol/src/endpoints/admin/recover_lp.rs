use crate::consts::ADMIN_KEY;
use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct RecoverLP<'info> {
    /// Signer
    #[account(constraint = signer.key() == ADMIN_KEY.key())]
    pub signer: Signer<'info>,
    /// Staking Pool Account
    #[account(
        seeds = [StakingPool::POOL_PREFIX, meme_mint.key().as_ref()],
        bump
    )]
    pub staking: Box<Account<'info, StakingPool>>,
    /// Staking Pool Signer
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub pool_lp_token_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_destination_lp_token_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> RecoverLP<'info> {
    fn recover_liquidity(&self, amount: u64, seeds: &[&[&[u8]]]) -> Result<()> {
        let instruction = anchor_spl::token::Transfer {
            from: self.pool_lp_token_ata.to_account_info(),
            to: self.user_destination_lp_token_ata.to_account_info(),
            authority: self.staking_pool_signer_pda.to_account_info(),
        };
        let ctx = CpiContext {
            accounts: instruction,
            program: self.token_program.to_account_info(),
            signer_seeds: seeds,
            remaining_accounts: vec![],
        };
        anchor_spl::token::transfer(ctx, amount)?;

        Ok(())
    }
}

pub fn handle<'info>(ctx: Context<RecoverLP<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_pool_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    msg!("1");

    accs.recover_liquidity(accs.pool_lp_token_ata.amount, staking_signer_seeds)?;

    msg!("2");

    Ok(())
}
