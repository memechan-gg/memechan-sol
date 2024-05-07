use crate::models::staking::StakingPool;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

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

pub fn handle<'info>(ctx: Context<'_, '_, '_, 'info, AddFees<'info>>) -> Result<()> {
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
