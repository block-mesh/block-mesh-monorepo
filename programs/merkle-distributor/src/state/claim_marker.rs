use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ClaimMarker {
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
