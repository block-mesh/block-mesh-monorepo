#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("GzscdwWG2FwpA6iqB6yYKEESvvw773c1iAzmJatXLcve");

#[program]
pub mod blockmesh_program {
    use super::*;

    pub fn create_client(ctx: Context<CreateClientContext>) -> Result<()> {
        create_client::create_client(ctx)
    }

    pub fn create_provider_node(
        ctx: Context<CreateProviderNodeContext>,
        args: CreateProviderNodeArgs,
    ) -> Result<()> {
        create_provider_node::create_provider_node(ctx, args)
    }

    pub fn create_api_token(ctx: Context<CreateApiTokenContext>) -> Result<()> {
        create_api_token::create_api_token(ctx)
    }
    pub fn update_latest_client_report(
        ctx: Context<UpdateLatestClientReportContext>,
        args: UpdateLatestClientReportArgs,
    ) -> Result<()> {
        update_latest_client_report::update_latest_client_report(ctx, args)
    }

    pub fn update_latest_provider_node_report(
        ctx: Context<UpdateLatestProviderNodeReportContext>,
        args: UpdateLatestProviderNodeReportArgs,
    ) -> Result<()> {
        update_latest_provider_node_report::update_latest_provider_node_report(ctx, args)
    }

    pub fn sync_token_usage(ctx: Context<SyncTokenUsageContext>) -> Result<()> {
        sync_token_usage::sync_token_usage(ctx)
    }
}
