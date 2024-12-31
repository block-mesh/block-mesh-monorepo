use crate::error::ErrorCode;
// use crate::merkle_proof;
use crate::state::claim_status::{ClaimStatus, ClaimedEvent};
use crate::state::merkle_distributor::MerkleDistributor;
use crate::state::off_chain::Claimant;
use crate::utils::{transfer_token_pda, vec_to_array};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use rs_merkle::algorithms::Sha256;
use rs_merkle::{Hasher, MerkleProof};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimArgs {
    index: u64,
    amount: u64,
    proof: Vec<u8>,
    leaves_to_prove: Vec<Vec<u8>>,
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
            distributor.key().as_ref(),
            claimant.key().as_ref(),
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
    let clock = Clock::get()?;
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
    let merkle_root = distributor.root;
    let proof_bytes = args.proof;
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes.clone())
        .map_err(|_| ErrorCode::InvalidProof)?;
    let indices_to_prove = [args.index as usize];
    let leaves_to_prove = args.leaves_to_prove;
    let leaves_to_prove = leaves_to_prove
        .iter()
        .map(|i| vec_to_array(i))
        .collect::<Vec<[u8; 32]>>();
    let leaves_to_prove = leaves_to_prove.as_slice();

    require!(
        proof.verify(
            merkle_root,
            &indices_to_prove,
            leaves_to_prove,
            distributor.leaves_len as usize,
        ),
        ErrorCode::InvalidProof
    );

    require_eq!(1, leaves_to_prove.len(), ErrorCode::InvalidProofLength);

    let leaf = Claimant {
        claimant: claimant.key(),
        amount: args.amount,
    };

    let leaf = Sha256::hash(&*leaf.as_bytes());
    let inner_leaf = leaves_to_prove[0];
    require!(leaf == inner_leaf, ErrorCode::CannotValidateProof);

    // Mark it claimed and send the tokens.
    claim_status.bump = ctx.bumps.claim_status;
    claim_status.amount = args.amount;
    claim_status.is_claimed = true;
    claim_status.mint = mint.key();
    claim_status.claimed_at = clock.unix_timestamp;
    claim_status.claimant = claimant.key();

    let mint_key = mint.key();
    let seeds = &[
        b"MerkleDistributor".as_ref(),
        mint_key.as_ref(),
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
        mint: mint.key(),
        amount: args.amount
    });
    Ok(())
}
