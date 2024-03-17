use crate::error::ErrorCode;
use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateProviderNodeArgs {
    pub ipv4: [u8; 4],
    pub port: u16,
    pub report_bandwidth_limit: u64,
}

#[derive(Accounts)]
#[instruction(args: UpdateProviderNodeArgs)]
pub struct UpdateProviderNodeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    constraint = signer.key() == provider_node.owner @ ErrorCode::SignerNotProviderNode,
    seeds      = [ProviderNode::PREFIX.as_bytes(), signer.key().as_ref()],
    bump       = provider_node.bump
    )]
    pub provider_node: Box<Account<'info, ProviderNode>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn update_provider_node(
    ctx: Context<UpdateProviderNodeContext>,
    args: UpdateProviderNodeArgs,
) -> Result<()> {
    let provider_node = &mut ctx.accounts.provider_node;
    provider_node.ipv4 = args.ipv4;
    provider_node.port = args.port;
    provider_node.report_bandwidth_limit = args.report_bandwidth_limit;
    provider_node.active = true;
    Ok(())
}
