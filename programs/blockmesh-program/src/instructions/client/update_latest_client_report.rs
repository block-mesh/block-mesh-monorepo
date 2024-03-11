use crate::error::ErrorCode;
use crate::state::api_token::ApiToken;
use crate::state::client::Client;
use crate::state::provider_node::ProviderNode;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateLatestClientReportArgs {
    pub latest_client_report: u64,
}

#[derive(Accounts)]
#[instruction(args: UpdateLatestClientReportArgs)]
pub struct UpdateLatestClientReportContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    constraint = signer.key() == client.owner @ ErrorCode::ClientNotProviderNode,
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
pub fn update_latest_client_report(
    ctx: Context<UpdateLatestClientReportContext>,
    args: UpdateLatestClientReportArgs,
) -> Result<()> {
    let api_token = &mut ctx.accounts.api_token;
    require_gte!(
        args.latest_client_report,
        api_token.latest_client_report,
        ErrorCode::LatestClientReportCannotBeLowerThanPreviousReport
    );
    api_token.latest_client_report = args.latest_client_report;
    Ok(())
}
