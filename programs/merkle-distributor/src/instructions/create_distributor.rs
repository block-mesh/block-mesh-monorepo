use crate::state::merkle_distributor::MerkleDistributor;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateDistributorArgs {
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub max_num_nodes: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateDistributorArgs)]
pub struct CreateDistributor<'info> {
    /// Base key of the distributor.
    pub base: Signer<'info>,
    /// [MerkleDistributor].
    #[account(
        init,
        seeds = [
            b"MerkleDistributor".as_ref(),
            base.key().to_bytes().as_ref()
        ],
        bump,
        space = 8 + MerkleDistributor::LEN,
        payer = payer
    )]
    pub distributor: Account<'info, MerkleDistributor>,
    /// The mint to distribute.
    pub mint: Account<'info, Mint>,
    /// Payer to create the distributor.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// The [System] program.
    pub system_program: Program<'info, System>,
}

pub fn create_distributor(
    ctx: Context<CreateDistributor>,
    args: CreateDistributorArgs,
) -> Result<()> {
    let distributor = &mut ctx.accounts.distributor;
    distributor.base = ctx.accounts.base.key();
    distributor.bump = ctx.bumps.distributor;
    distributor.root = args.root;
    distributor.mint = ctx.accounts.mint.key();
    distributor.max_total_claim = args.max_total_claim;
    distributor.max_num_nodes = args.max_num_nodes;
    distributor.total_amount_claimed = 0;
    distributor.num_nodes_claimed = 0;
    Ok(())
}
