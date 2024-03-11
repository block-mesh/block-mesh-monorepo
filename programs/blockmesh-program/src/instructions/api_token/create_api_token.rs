use crate::state::api_token::{ApiToken, DisputeStatus};
use crate::state::client::Client;
use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateApiTokenContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = ApiToken::SIZE,
    seeds = [ApiToken::PREFIX.as_bytes(), signer.key().as_ref(), provider_node.owner.as_ref()],
    bump
    )]
    pub api_token: Box<Account<'info, ApiToken>>,
    #[account(
    seeds = [Client::PREFIX.as_bytes(), signer.key().as_ref()],
    bump  = client.bump
    )]
    pub client: Box<Account<'info, Client>>,
    #[account(
    seeds = [ProviderNode::PREFIX.as_bytes(), provider_node.owner.as_ref()],
    bump  = provider_node.bump
    )]
    pub provider_node: Box<Account<'info, ProviderNode>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn create_api_token(ctx: Context<CreateApiTokenContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let client = &mut ctx.accounts.client;
    let provider_node = &ctx.accounts.provider_node;
    let api_token = &mut ctx.accounts.api_token;
    api_token.bump = ctx.bumps.api_token;
    api_token.owner = signer.key();
    api_token.client = client.key();
    api_token.provider_node = provider_node.key();
    // TODO - need to make variable - for POC it's a const
    api_token.bandwidth_paid = 1_000_000;
    api_token.bandwidth_used = 0;
    api_token.dispute_status = DisputeStatus::NoDispute;
    api_token.latest_provider_node_report = 0;
    api_token.latest_client_report = 0;
    Ok(())
}
