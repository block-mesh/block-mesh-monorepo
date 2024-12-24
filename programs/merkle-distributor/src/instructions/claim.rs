use crate::error::ErrorCode;
use crate::merkle_proof;
use crate::state::claim_status::{ClaimStatus, ClaimedEvent};
use crate::state::merkle_distributor::MerkleDistributor;
use crate::utils::assert_ata;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimArgs {
    index: u64,
    amount: u64,
    proof: Vec<[u8; 32]>,
}

#[derive(Accounts)]
#[instruction(args: ClaimArgs)]
pub struct Claim<'info> {
    /// The [MerkleDistributor].
    #[account(
        mut,
        address = from.owner
    )]
    pub distributor: Account<'info, MerkleDistributor>,

    /// Status of the claim.
    #[account(
        init,
        seeds = [
            b"ClaimStatus".as_ref(),
            args.index.to_le_bytes().as_ref(),
            distributor.key().to_bytes().as_ref()
        ],
        bump,
        space = 8 + ClaimStatus::LEN,
        payer = payer
    )]
    pub claim_status: Account<'info, ClaimStatus>,

    /// Distributor ATA containing the tokens to distribute.
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    /// Account to send the claimed tokens to.
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    /// Who is claiming the tokens.
    #[account(address = to.owner @ ErrorCode::OwnerMismatch)]
    pub claimant: Signer<'info>,

    /// Payer of the claim.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The [System] program.
    pub system_program: Program<'info, System>,

    /// SPL [Token] program.
    pub token_program: Program<'info, Token>,
}

pub fn claim(ctx: Context<Claim>, args: ClaimArgs) -> Result<()> {
    require_keys_neq!(ctx.accounts.from.key(), ctx.accounts.to.key());
    let claim_status = &mut ctx.accounts.claim_status;
    require!(
        // This check is redundant, we should not be able to initialize a claim status account at the same key.
        !claim_status.is_claimed && claim_status.claimed_at == 0,
        ErrorCode::DropAlreadyClaimed
    );

    let claimant_account = &ctx.accounts.claimant;
    let distributor = &ctx.accounts.distributor;
    require!(claimant_account.is_signer, ErrorCode::Unauthorized);

    // Verify the merkle proof.
    let node = anchor_lang::solana_program::keccak::hashv(&[
        &args.index.to_le_bytes(),
        &claimant_account.key().to_bytes(),
        &args.amount.to_le_bytes(),
    ]);
    require!(
        merkle_proof::verify(args.proof, distributor.root, node.0),
        ErrorCode::InvalidProof
    );

    // Mark it claimed and send the tokens.
    claim_status.amount = args.amount;
    claim_status.is_claimed = true;
    let clock = Clock::get()?;
    claim_status.claimed_at = clock.unix_timestamp;
    claim_status.claimant = claimant_account.key();

    let seeds = [
        b"MerkleDistributor".as_ref(),
        &distributor.base.to_bytes(),
        &[ctx.accounts.distributor.bump],
    ];

    // #[allow(deprecated)]
    // {
    //     vipers::assert_ata!(
    //             ctx.accounts.from,
    //             ctx.accounts.distributor,
    //             distributor.mint
    //         );
    // }
    assert_ata(
        &ctx.accounts.from.key(),
        &ctx.accounts.distributor.key(),
        &distributor.mint.key(),
    )?;
    require_keys_eq!(
        ctx.accounts.to.owner,
        claimant_account.key(),
        ErrorCode::OwnerMismatch
    );
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.distributor.to_account_info(),
            },
        )
        .with_signer(&[&seeds[..]]),
        args.amount,
    )?;

    let distributor = &mut ctx.accounts.distributor;
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
        claimant: claimant_account.key(),
        amount: args.amount
    });
    Ok(())
}
