use anchor_lang::prelude::*;

#[account]
pub struct UserPoints {
    pub points: u64,
    pub points_received: u64,
    pub referrer: Pubkey,
    pub padding: [u8; 8],
}

impl UserPoints {
    pub const USER_POINTS_PREFIX: &'static [u8; 11] = b"user_points";

    pub fn space() -> usize {
        let discriminant = 8;
        let points = 8;
        let points_received = 8;
        let referrer = 32;
        let padding = 8;

        discriminant + points + points_received + referrer + padding
    }
}
