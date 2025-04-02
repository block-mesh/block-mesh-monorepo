#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("AZMc26abaSP7si1wtLaV5yPxTxpWd895M8YpJFFdQ8Qw");

#[program]
pub mod merkle_distributor {
    use super::*;

    pub fn claim_marker(ctx: Context<ClaimMarkerContext>) -> Result<()> {
        claim_marker::claim_marker(ctx)
    }

    pub fn create_air_dropper(ctx: Context<CreateAirDropperContext>) -> Result<()> {
        create_air_dropper::create_air_dropper(ctx)
    }

    pub fn create_marker(ctx: Context<CreateMarkerContext>, args: CreateMarkerArgs) -> Result<()> {
        create_marker::create_marker(ctx, args)
    }

    pub fn create_distributor(
        ctx: Context<CreateDistributor>,
        args: CreateDistributorArgs,
    ) -> Result<()> {
        create_distributor::create_distributor(ctx, args)
    }

    pub fn claim(ctx: Context<Claim>, args: ClaimArgs) -> Result<()> {
        claim::claim(ctx, args)
    }

    pub fn reclaim_marker(ctx: Context<ReclaimMarkerContext>) -> Result<()> {
        reclaim_marker::reclaim_marker(ctx)
    }

    pub fn create_fee_collector(
        ctx: Context<CreateFeeCollectorContext>,
        args: CreateFeeCollectorArgs,
    ) -> Result<()> {
        create_fee_collector::create_fee_collector(ctx, args)
    }

    pub fn close_token_account_with_fee(
        ctx: Context<CloseTokenAccountWithFeeContext>,
    ) -> Result<()> {
        close_token_account_with_fee::close_token_account_with_fee(ctx)
    }
}
