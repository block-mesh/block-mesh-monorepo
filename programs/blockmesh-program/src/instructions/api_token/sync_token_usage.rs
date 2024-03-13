use crate::error::ErrorCode;
use crate::state::api_token::ApiToken;
use crate::state::client::Client;
use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SyncTokenUsageContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    seeds = [ApiToken::PREFIX.as_bytes(), client.owner.as_ref(), provider_node.owner.as_ref()],
    bump = api_token.bump
    )]
    pub api_token: Box<Account<'info, ApiToken>>,
    #[account(
    seeds = [Client::PREFIX.as_bytes(), client.owner.as_ref()],
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
pub fn sync_token_usage(ctx: Context<SyncTokenUsageContext>) -> Result<()> {
    let api_token = &mut ctx.accounts.api_token;
    require_eq!(
        api_token.latest_client_report,
        api_token.latest_provider_node_report,
        ErrorCode::MismatchOnReportedUsage
    );
    Ok(())
}
