use anchor_lang::prelude::*;

#[derive(Default, Debug, AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum DisputeStatus {
    #[default]
    NoDispute,
    Dispute,
}

#[account]
#[derive(Default, Debug)]
pub struct ApiToken {
    pub bump: u8,
    pub owner: Pubkey,
    pub client: Pubkey,
    pub provider_node: Pubkey,
    pub api_token: Pubkey,
    pub bandwidth_paid: u64,
    pub bandwidth_used: u64,
    pub dispute_status: DisputeStatus,
    pub latest_client_report: u64,
    pub latest_provider_node_report: u64,
}

impl ApiToken {
    pub const PREFIX: &'static str = "API_TOKEN";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        std::mem::size_of::<Pubkey>() + /* owner */
        std::mem::size_of::<Pubkey>() + /* client */
        std::mem::size_of::<Pubkey>() + /* provider_node */
        std::mem::size_of::<Pubkey>() + /* api_token */
        std::mem::size_of::<u64>() + /* bandwidth_paid */
        std::mem::size_of::<u64>() + /* bandwidth_used */
        std::mem::size_of::<DisputeStatus>() + /* dispute_status */
        std::mem::size_of::<u64>() + /* latest_client_report */
        std::mem::size_of::<u64>() + /* latest_provider_node_report */
        64; /* padding */
}
