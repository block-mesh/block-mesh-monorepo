use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct EndpointNode {
    pub bump: u8,
    pub owner: Pubkey,
}

impl EndpointNode {
    pub const PREFIX: &'static str = "ENDPOINT";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* owner */
        64; /* padding */
}
