use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct ProviderNode {
    pub bump: u8,
    pub owner: Pubkey,
}

impl ProviderNode {
    pub const PREFIX: &'static str = "PROVIDER_NODE";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* owner */
        64; /* padding */
}
