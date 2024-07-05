use crate::consts::SWAP_AUTH_KEY;
use crate::err;
use crate::models::staking::StakingPool;
use anchor_lang::context::{Context, CpiContext};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use solana_program::account_info::AccountInfo;

#[derive(Accounts)]
pub struct SendAirdropFunds<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut, constraint = staking.to_airdrop != 0)]
    pub staking: Box<Account<'info, StakingPool>>,
    //
    /// Staking Pool Signer
    /// CHECK: live phase pda signer
    #[account(mut, seeds = [StakingPool::SIGNER_PDA_PREFIX, staking.key().as_ref()], bump)]
    pub staking_pool_signer_pda: AccountInfo<'info>,
    #[account(mut)]
    pub staking_meme_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = meme_mint.key() == staking_meme_vault.mint
            @ err::acc("Invalid meme mint")
    )]
    pub meme_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = sender,
        associated_token::mint = meme_mint,
        associated_token::authority = airdrop_owner
    )]
    pub airdrop_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(constraint = airdrop_owner.key() == SWAP_AUTH_KEY)]
    /// CHECK: constraint
    pub airdrop_owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> SendAirdropFunds<'info> {
    fn transfer_airdrop_meme_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.staking_meme_vault.to_account_info(),
            to: self.airdrop_token_vault.to_account_info(),
            authority: self.staking_pool_signer_pda.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle(ctx: Context<SendAirdropFunds>) -> Result<()> {
    let accs = ctx.accounts;

    let staking_seeds = &[
        StakingPool::SIGNER_PDA_PREFIX,
        &accs.staking.key().to_bytes()[..],
        &[ctx.bumps.staking_pool_signer_pda],
    ];

    let staking_signer_seeds = &[&staking_seeds[..]];

    accs.staking.to_airdrop = 0;

    token::transfer(
        accs.transfer_airdrop_meme_ctx()
            .with_signer(staking_signer_seeds),
        accs.staking.to_airdrop,
    )
    .unwrap();

    Ok(())
}
