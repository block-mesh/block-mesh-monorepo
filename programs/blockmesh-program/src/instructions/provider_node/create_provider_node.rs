use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
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
pub fn create_provider_node(ctx: Context<CreateProviderNodeContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let provider_node = &mut ctx.accounts.provider_node;
    provider_node.bump = ctx.bumps.provider_node;
    provider_node.owner = signer.key();
    Ok(())
}
