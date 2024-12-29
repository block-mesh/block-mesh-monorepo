use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct AirDropper {
    pub bump: u8,
    pub owner: Pubkey,
}
