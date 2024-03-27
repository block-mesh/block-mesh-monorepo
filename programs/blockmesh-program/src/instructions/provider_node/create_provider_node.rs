use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateProviderNodeArgs {
    pub ipv4: [u8; 4],
    pub proxy_port: u16,
    pub client_port: u16,
    pub report_bandwidth_limit: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateProviderNodeArgs)]
pub struct CreateProviderNodeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = ProviderNode::SIZE,
    seeds = [ProviderNode::PREFIX.as_bytes(), signer.key().as_ref()],
    bump
    )]
    pub provider_node: Box<Account<'info, ProviderNode>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn create_provider_node(
    ctx: Context<CreateProviderNodeContext>,
    args: CreateProviderNodeArgs,
) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let provider_node = &mut ctx.accounts.provider_node;
    provider_node.bump = ctx.bumps.provider_node;
    provider_node.owner = signer.key();
    provider_node.ipv4 = args.ipv4;
    provider_node.proxy_port = args.proxy_port;
    provider_node.client_port = args.client_port;
    provider_node.report_bandwidth_limit = args.report_bandwidth_limit;
    provider_node.active = true;
    Ok(())
}
