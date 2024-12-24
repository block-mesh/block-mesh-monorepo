#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod merkle_proof;
pub mod state;
pub mod utils;

pub use instructions::*;

declare_id!("AZMc26abaSP7si1wtLaV5yPxTxpWd895M8YpJFFdQ8Qw");

#[program]
pub mod merkle_distributor {
    use super::*;

    pub fn create_distributor(
        ctx: Context<CreateDistributor>,
        args: CreateDistributorArgs,
    ) -> Result<()> {
        create_distributor::create_distributor(ctx, args)
    }

    pub fn claim(ctx: Context<Claim>, args: ClaimArgs) -> Result<()> {
        claim::claim(ctx, args)
    }
}
