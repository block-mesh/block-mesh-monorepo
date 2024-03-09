mod error;
mod instructions;
mod state;
mod utils;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("GzscdwWG2FwpA6iqB6yYKEESvvw773c1iAzmJatXLcve");

#[program]
pub mod blockmesh_program {
    use super::*;

    pub fn create_client(ctx: Context<CreateClientContext>) -> Result<()> {
        create_client::create_client(ctx)
    }

    pub fn create_provider_node(ctx: Context<CreateProviderNodeContext>) -> Result<()> {
        create_provider_node::create_provider_node(ctx)
    }
}
