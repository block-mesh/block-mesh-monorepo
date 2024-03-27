use crate::state::endpoint_node::EndpointNode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateEndpointContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = EndpointNode::SIZE,
    seeds = [EndpointNode::PREFIX.as_bytes(), signer.key().as_ref()],
    bump
    )]
    pub endpoint: Box<Account<'info, EndpointNode>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn create_endpoint_node(ctx: Context<CreateEndpointContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let endpoint = &mut ctx.accounts.endpoint;
    endpoint.bump = ctx.bumps.endpoint;
    endpoint.owner = signer.key();
    Ok(())
}
