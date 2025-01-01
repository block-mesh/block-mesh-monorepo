use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct FeeCollector {
    /// Bump seed
    pub bump: u8,
    /// Controller
    pub owner: Pubkey,
    /// [Fee] 10_000 basis points
    pub fee: u64,
}

impl FeeCollector {
    pub const LEN: usize = 500;

    pub fn fee_per(&self) -> f64 {
        self.fee as f64 / 10_000f64
    }
}
