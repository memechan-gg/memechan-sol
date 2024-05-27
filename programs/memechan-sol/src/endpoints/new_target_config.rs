use crate::err;
use crate::models::target_config::TargetConfig;

use crate::consts::ADMIN_KEY;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct NewTargetConfig<'info> {
    #[account(
        mut,
        constraint = sender.key() == ADMIN_KEY.key()
            @ err::acc("Sender must be admin"),
    )]
    pub sender: Signer<'info>,
    #[account(
        init,
        payer = sender,
        space = TargetConfig::space(),
        seeds = [TargetConfig::CONFIG_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub target_config: Account<'info, TargetConfig>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handle<'info>(ctx: Context<NewTargetConfig<'info>>, target_amount: u64) -> Result<()> {
    let accs = ctx.accounts;

    let config = &mut accs.target_config;
    config.token_mint = accs.mint.key();
    config.token_target_amount = target_amount;

    Ok(())
}
