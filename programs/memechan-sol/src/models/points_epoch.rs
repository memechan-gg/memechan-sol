use anchor_lang::prelude::*;

#[account]
pub struct PointsEpoch {
    pub epoch_number: u64,
    pub points_per_sol_num: u64,
    pub points_per_sol_denom: u64,
    pub padding: [u8; 8],
}

impl PointsEpoch {
    pub const POINTS_EPOCH_PREFIX: &'static [u8; 12] = b"points_epoch";

    pub fn space() -> usize {
        let discriminant = 8;
        let epoch_number = 8;
        let points_per_sol_num = 8;
        let points_per_sol_denom = 8;
        let padding = 64;

        discriminant + epoch_number + points_per_sol_num + points_per_sol_denom + padding
    }
}
