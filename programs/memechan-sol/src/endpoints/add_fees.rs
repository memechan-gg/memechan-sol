use crate::{models::staking::StakingPool, raydium, RAYDIUM_PROGRAM_ID};
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
    pub raydium_lp_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub pool_lp_wallet: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    // raydium
    #[account(mut)]
    pub raydium_amm: AccountInfo<'info>,
    pub raydium_amm_authority: AccountInfo<'info>,
    pub open_orders: AccountInfo<'info>,
    pub raydium_meme_vault: AccountInfo<'info>,
    pub raydium_wsol_vault: AccountInfo<'info>,
    pub target_orders: AccountInfo<'info>,
    pub amm_config: AccountInfo<'info>,
    pub fee_destination: AccountInfo<'info>,
    pub market_program_id: AccountInfo<'info>,
    pub market_account: AccountInfo<'info>,
    pub user_destination_lp_token_ata: AccountInfo<'info>,
    pub market_event_queue: AccountInfo<'info>,
    pub market_coin_vault: AccountInfo<'info>,
    pub market_pc_vault: AccountInfo<'info>,
    pub market_vault_signer: AccountInfo<'info>,
    pub market_bids: AccountInfo<'info>,
    pub market_asks: AccountInfo<'info>,
}

impl<'info> AddFees<'info> {
    pub fn redeem_liquidity(&self, amount: u64, signer_seeds: &[&[&[u8]]; 1]) -> Result<()> {
        let instruction = raydium::withdraw(
            &RAYDIUM_PROGRAM_ID,
            // params
            amount,
            // accounts
            &self.token_program.key(),
            &self.raydium_amm.key(),
            &self.raydium_amm_authority.key(),
            &self.open_orders.key(),
            &self.target_orders.key(),
            &self.raydium_lp_mint.key(),    // lp mint
            &self.raydium_meme_vault.key(), // coin_vault
            &self.raydium_wsol_vault.key(), // pc_vault
            &self.market_program_id.key(),
            &self.market_account.key(),
            &self.market_coin_vault.key(),
            &self.market_pc_vault.key(),
            &self.market_vault_signer.key(),
            &self.pool_lp_wallet.key(),
            &self.meme_vault.key(), // user wallet (pool)
            &self.wsol_vault.key(), // user wallet (pool)
            &self.signer.key(),     // user wallet
            &self.market_event_queue.key(),
            &self.market_bids.key(),
            &self.market_asks.key(),
        );

        solana_program::program::invoke_signed(
            &instruction,
            &[
                self.token_program.to_account_info().clone(),
                self.raydium_amm.to_account_info().clone(),
                self.raydium_amm_authority.to_account_info().clone(),
                self.open_orders.to_account_info().clone(),
                self.target_orders.to_account_info().clone(),
                self.raydium_lp_mint.to_account_info().clone(),
                self.raydium_meme_vault.to_account_info().clone(),
                self.raydium_wsol_vault.to_account_info().clone(),
                self.market_program_id.to_account_info().clone(),
                self.market_account.to_account_info().clone(),
                self.market_coin_vault.to_account_info().clone(),
                self.market_pc_vault.to_account_info().clone(),
                self.market_vault_signer.to_account_info().clone(),
                self.pool_lp_wallet.to_account_info().clone(),
                self.meme_vault.to_account_info().clone(),
                self.wsol_vault.to_account_info().clone(),
                self.signer.to_account_info().clone(),
                self.market_event_queue.to_account_info().clone(),
                self.market_bids.to_account_info().clone(),
                self.market_asks.to_account_info().clone(),
            ],
            signer_seeds,
        )?;

        Ok(())
    }

    // pub fn withdraw_fees_ctx(&self) -> CpiContext<'_, '_, '_, 'info, RedeemLiquidity<'info>> {
    //     let cpi_program = self.aldrin_amm_program.to_account_info();
    //     let cpi_accounts = RedeemLiquidity {
    //         user: self.staking_signer_pda.to_account_info(),
    //         pool: self.aldrin_pool_acc.to_account_info(),
    //         pool_signer: self.aldrin_pool_signer.to_account_info(),
    //         lp_mint: self.aldrin_lp_mint.to_account_info(),
    //         lp_token_wallet: self.aldrin_pool_lp_wallet.to_account_info(),
    //         token_program: self.token_program.to_account_info(),
    //     };
    //     CpiContext::new(cpi_program, cpi_accounts)
    // }
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

    // TODO: Ammount of LP to withdraw should come as params
    accs.redeem_liquidity(1, staking_signer_seeds)?;

    accs.meme_vault.reload().unwrap();
    accs.wsol_vault.reload().unwrap();

    let state = &mut accs.staking;
    state.fees_x_total += accs.meme_vault.amount - meme_vault_initial_amt;
    state.fees_y_total += accs.wsol_vault.amount - wsol_vault_initial_amt;

    Ok(())
}
