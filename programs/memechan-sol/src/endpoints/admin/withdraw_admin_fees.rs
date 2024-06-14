use crate::consts::ADMIN_KEY;
use crate::models::bound::BoundPool;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawAdminFee<'info> {
    #[account(
        mut,
        constraint = sender.key() == ADMIN_KEY.key()
    )]
    pub sender: Signer<'info>,
    #[account(
        mut,
        has_one = fee_vault_quote
    )]
    pub pool: Box<Account<'info, BoundPool>>,
    /// CHECK: bound-curve phase pda signer
    #[account(seeds = [BoundPool::SIGNER_PDA_PREFIX, pool.key().as_ref()], bump)]
    pub bound_pool_signer_pda: AccountInfo<'info>,
    #[account(
        mut,
        constraint = pool.quote_reserve.vault == pool_quote_vault.key()
    )]
    /// Bonding Pool Quote Vault
    pub pool_quote_vault: Box<Account<'info, TokenAccount>>,
    /// Bonding Pool Fee Vault
    #[account(
        mut,
        constraint = pool.fee_vault_quote == fee_vault_quote.key()
    )]
    pub fee_vault_quote: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawAdminFee<'info> {
    fn send_admin_fee_sol(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.pool_quote_vault.to_account_info(),
            to: self.fee_vault_quote.to_account_info(),
            authority: self.bound_pool_signer_pda.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn handle<'info>(ctx: Context<WithdrawAdminFee<'info>>) -> Result<()> {
    let accs = ctx.accounts;

    let bp_seeds = &[
        BoundPool::SIGNER_PDA_PREFIX,
        &accs.pool.key().to_bytes()[..],
        &[*ctx.bumps.get("bound_pool_signer_pda").unwrap()],
    ];

    let bp_signer_seeds = &[&bp_seeds[..]];

    if accs.pool.admin_fees_quote != 0 {
        token::transfer(
            accs.send_admin_fee_sol().with_signer(bp_signer_seeds),
            accs.pool.admin_fees_quote,
        )
        .unwrap();
    };

    accs.pool.admin_fees_quote = 0;

    Ok(())
}
