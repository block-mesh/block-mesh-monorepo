use crate::error::ErrorCode;
use crate::merkle_proof;
use crate::state::claim_status::{ClaimStatus, ClaimedEvent};
use crate::state::merkle_distributor::MerkleDistributor;
use crate::utils::transfer_token_pda;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimArgs {
    index: u64,
    amount: u64,
    proof: Vec<[u8; 32]>,
}

#[derive(Accounts)]
#[instruction(args: ClaimArgs)]
pub struct Claim<'info> {
    /// Who is claiming the tokens.
    #[account(mut)]
    pub claimant: Signer<'info>,
    #[account(
        init_if_needed,
        associated_token::mint = mint,
        associated_token::authority = claimant,
        payer = claimant
    )]
    pub claimant_token_account: Box<Account<'info, TokenAccount>>,
    /// The [MerkleDistributor].
    #[account(mut,
      seeds = [
            b"MerkleDistributor".as_ref(),
            mint.key().as_ref()
        ],
    bump=distributor.bump
    )]
    pub distributor: Account<'info, MerkleDistributor>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = distributor,
        seeds = [b"MerkleDistributor".as_ref(), distributor.key().as_ref()],
        bump
    )]
    pub distributor_token_account: Box<Account<'info, TokenAccount>>,
    /// Status of the claim.
    #[account(
        init,
        seeds = [
            b"ClaimStatus".as_ref(),
            distributor.key().to_bytes().as_ref(),
            claimant.key().as_ref(),
            args.index.to_le_bytes().as_ref(),
        ],
        bump,
        space = 8 + ClaimStatus::LEN,
        payer = claimant
    )]
    pub claim_status: Account<'info, ClaimStatus>,
    /// Distributor ATA containing the tokens to distribute.
    /// The [System] program.
    pub system_program: Program<'info, System>,
    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
    pub mint: Box<Account<'info, Mint>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn claim(ctx: Context<Claim>, args: ClaimArgs) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let claim_status = &mut ctx.accounts.claim_status;
    let claimant = &ctx.accounts.claimant;
    let claimant_token_account = &ctx.accounts.claimant_token_account;
    let distributor_token_account = &ctx.accounts.distributor_token_account;
    let token_program = &ctx.accounts.token_program;
    let distributor = &mut ctx.accounts.distributor;
    require!(
        // This check is redundant, we should not be able to initialize a claim status account at the same key.
        !claim_status.is_claimed && claim_status.claimed_at == 0,
        ErrorCode::DropAlreadyClaimed
    );
    require!(claimant.is_signer, ErrorCode::Unauthorized);
    // Verify the merkle proof.
    let node = anchor_lang::solana_program::keccak::hashv(&[
        &args.index.to_le_bytes(),
        &claimant.key().to_bytes(),
        &args.amount.to_le_bytes(),
    ]);
    require!(
        merkle_proof::verify(args.proof, distributor.root, node.0),
        ErrorCode::InvalidProof
    );
    // Mark it claimed and send the tokens.
    claim_status.bump = ctx.bumps.claim_status;
    claim_status.amount = args.amount;
    claim_status.is_claimed = true;
    let clock = Clock::get()?;
    claim_status.claimed_at = clock.unix_timestamp;
    claim_status.claimant = claimant.key();

    let mint_key = mint.key();
    let seeds = &[
        b"MerkleDistributor".as_ref(),
        &mint_key.as_ref(),
        &[distributor.bump],
    ];
    transfer_token_pda(
        distributor_token_account.to_account_info(),
        claimant_token_account.to_account_info(),
        token_program.to_account_info(),
        distributor.to_account_info(),
        args.amount,
        &[seeds],
    )?;
    distributor.total_amount_claimed = distributor
        .total_amount_claimed
        .checked_add(args.amount)
        .ok_or(ErrorCode::BadMath)?;
    require!(
        distributor.total_amount_claimed <= distributor.max_total_claim,
        ErrorCode::ExceededMaxClaim
    );
    distributor.num_nodes_claimed = distributor
        .num_nodes_claimed
        .checked_add(1)
        .ok_or(ErrorCode::BadMath)?;
    require!(
        distributor.num_nodes_claimed <= distributor.max_num_nodes,
        ErrorCode::ExceededMaxNumNodes
    );
    emit!(ClaimedEvent {
        index: args.index,
        claimant: claimant.key(),
        amount: args.amount
    });
    Ok(())
}
