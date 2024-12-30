use crate::state::merkle_distributor::MerkleDistributor;
use crate::utils::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateDistributorArgs {
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub max_num_nodes: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateDistributorArgs)]
pub struct CreateDistributor<'info> {
    /// Distributor.
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    /// [MerkleDistributor].
    #[account(
        init,
        seeds = [
            b"MerkleDistributor".as_ref(),
            mint.key().as_ref()
        ],
        bump,
        space = 8 + MerkleDistributor::LEN,
        payer = payer
    )]
    pub distributor: Account<'info, MerkleDistributor>,
    #[account(
    init,
    payer = signer,
    token::mint = mint,
    token::authority = distributor,
    seeds = [b"MerkleDistributor".as_ref(), distributor.key().as_ref()],
    bump
    )]
    pub distributor_token_account: Box<Account<'info, TokenAccount>>,
    /// The mint to distribute.
    pub mint: Account<'info, Mint>,
    /// Payer to create the distributor.
    #[account(mut)]
    pub payer: Signer<'info>,
    /// The [System] program.
    pub system_program: Program<'info, System>,
    /// The [Token] program.
    pub token_program: Program<'info, Token>,
    /// The [Associated Token] program.
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// The [Rent] sysvar.
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_distributor(
    ctx: Context<CreateDistributor>,
    args: CreateDistributorArgs,
) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let signer_token_account = &ctx.accounts.signer_token_account;
    let distributor = &mut ctx.accounts.distributor;
    let distributor_token_account = &ctx.accounts.distributor_token_account;
    let token_program = &ctx.accounts.token_program;
    distributor.bump = ctx.bumps.distributor;
    distributor.signer = ctx.accounts.signer.key();
    distributor.root = args.root;
    distributor.mint = ctx.accounts.mint.key();
    distributor.max_total_claim = args.max_total_claim;
    distributor.max_num_nodes = args.max_num_nodes;
    distributor.total_amount_claimed = 0;
    distributor.num_nodes_claimed = 0;
    distributor.token_account = distributor_token_account.key();
    transfer_token(
        signer_token_account.to_account_info(),
        distributor_token_account.to_account_info(),
        token_program.to_account_info(),
        signer.to_account_info(),
        args.max_total_claim,
    )?;
    Ok(())
}
