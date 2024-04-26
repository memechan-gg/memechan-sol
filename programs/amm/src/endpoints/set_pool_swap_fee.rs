//! Admin of a pool can change the swap fee to a maximum of
//! [`consts::MAX_SWAP_FEE`].

use crate::prelude::*;

#[derive(Accounts)]
pub struct SetPoolSwapFee<'info> {
    pub admin: Signer<'info>,
    #[account(
        mut,
        constraint = pool.admin.key() == admin.key()
            @ err::acc("The signer must match pool's admin"),
    )]
    pub pool: Account<'info, Pool>,
}
