use anchor_lang::prelude::*;
// use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

/// Holds whether or not a claimant has claimed tokens.
#[account]
#[derive(Default)]
pub struct ClaimStatus {
    pub bump: u8,
    /// If true, the tokens have been claimed.
    pub is_claimed: bool,
    /// Authority that claimed the tokens.
    pub claimant: Pubkey,
    /// When the tokens were claimed.
    pub claimed_at: i64,
    /// Amount of tokens claimed.
    pub amount: u64,
}

impl ClaimStatus {
    pub const LEN: usize = 500;
}

/// Emitted when tokens are claimed.
#[event]
pub struct ClaimedEvent {
    /// Index of the claim.
    pub index: u64,
    /// User that claimed.
    pub claimant: Pubkey,
    /// Amount of tokens to distribute.
    pub amount: u64,
}
