use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct ProviderNode {
    pub bump: u8,
    pub owner: Pubkey,
    pub ipv4: [u8; 4],
    pub port: u16,
    pub active: bool,
    pub report_bandwidth_limit: u64,
}

impl ProviderNode {
    pub const PREFIX: &'static str = "PROVIDER_NODE";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* owner */
        4 * std::mem::size_of::<u8>() + /* ipv4 */
        4 * std::mem::size_of::<u16>() + /* port */
        4 * std::mem::size_of::<bool>() + /* bool */
        4 * std::mem::size_of::<u64>() + /* report_bandwidth_limit */
        64; /* padding */
}
