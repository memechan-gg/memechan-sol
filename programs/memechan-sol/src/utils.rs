use solana_program::pubkey::Pubkey;

pub fn check_slerf_mint(mint: Pubkey) -> bool {
    #[cfg(feature = "dev")]
    return true;
    #[cfg(not(feature = "dev"))]
    return mint == SLERF_MINT;
}
