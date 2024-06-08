use crate::models::target_config::TargetConfig;
use crate::raydium::Accounts;

use crate::consts::ADMIN_KEY;
use anchor_lang::context::Context;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeTargetConfig<'info> {
    #[account(constraint = sender.key() == ADMIN_KEY.key())]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub target_config: Account<'info, TargetConfig>,
}

pub fn handle<'info>(
    ctx: Context<ChangeTargetConfig<'info>>,
    new_target_amount: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.target_config;

    config.token_target_amount = new_target_amount;

    Ok(())
}
