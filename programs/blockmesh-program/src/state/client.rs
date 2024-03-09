use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Client {
    pub bump: u8,
    pub owner: Pubkey,
}

impl Client {
    pub const PREFIX: &'static str = "CLIENT";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* owner */
        64; /* padding */
}
