use crate::error::ErrorCode;
use crate::state::api_token::ApiToken;
use crate::state::client::Client;
use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateLatestProviderNodeReportArgs {
    pub latest_provider_node_report: u64,
}

#[derive(Accounts)]
#[instruction(args: UpdateLatestProviderNodeReportArgs)]
pub struct UpdateLatestProviderNodeReportContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    constraint = signer.key() == provider_node.owner @ ErrorCode::SignerNotProviderNode,
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
pub fn update_latest_provider_node_report(
    ctx: Context<UpdateLatestProviderNodeReportContext>,
    args: UpdateLatestProviderNodeReportArgs,
) -> Result<()> {
    let api_token = &mut ctx.accounts.api_token;
    require_gte!(
        args.latest_provider_node_report,
        api_token.latest_provider_node_report,
        ErrorCode::LatestProviderNodeReportCannotBeLowerThanPreviousReport
    );
    api_token.latest_provider_node_report = args.latest_provider_node_report;
    Ok(())
}
