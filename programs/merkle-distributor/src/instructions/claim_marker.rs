use crate::state::air_dropper::AirDropper;
use crate::state::claim_marker::ClaimMarker;
use crate::utils::transfer_token_pda;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct ClaimMarkerContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init_if_needed,
    associated_token::mint = mint,
    associated_token::authority = signer,
    payer = signer
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    seeds = [b"AirDropper".as_ref()],
    bump=air_dropper.bump
    )]
    pub air_dropper: Box<Account<'info, AirDropper>>,
    #[account(mut,
    seeds = [b"ClaimMarker".as_ref(), signer.key().as_ref()],
    constraint = claim_marker.claimant == signer.key(),
    bump=claim_marker.bump
    )]
    pub claim_marker: Box<Account<'info, ClaimMarker>>,
    #[account(
    init,
    payer = signer,
    seeds = [b"ClaimMarker2".as_ref(), signer.key().as_ref()],
    space = 600,
    bump
    )]
    pub claim_marker2: Box<Account<'info, ClaimMarker>>,
    #[account(
    mut,
    token::mint = mint,
    token::authority = air_dropper,
    seeds = [b"ClaimMarker".as_ref(), mint.key().as_ref(), signer.key().as_ref()],
    bump
    )]
    pub claim_marker_token_account: Box<Account<'info, TokenAccount>>,
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn claim_marker(ctx: Context<ClaimMarkerContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let token_program = &ctx.accounts.token_program;
    let signer_token_account = &ctx.accounts.signer_token_account;
    let claim_marker_token_account = &ctx.accounts.claim_marker_token_account;
    let claim_marker = &mut ctx.accounts.claim_marker;
    let claim_marker2 = &mut ctx.accounts.claim_marker2;
    claim_marker2.bump = ctx.bumps.claim_marker2;
    claim_marker2.amount = claim_marker.amount;
    claim_marker2.claimant = claim_marker.claimant;
    let air_dropper = &ctx.accounts.air_dropper;
    require_eq!(claim_marker.is_claimed, false);
    require_keys_eq!(signer.key(), claim_marker.claimant);
    let seeds = &["AirDropper".as_bytes(), &[air_dropper.bump]];
    transfer_token_pda(
        claim_marker_token_account.to_account_info(),
        signer_token_account.to_account_info(),
        token_program.to_account_info(),
        air_dropper.to_account_info(),
        claim_marker.amount,
        &[seeds],
    )?;
    claim_marker.is_claimed = true;
    claim_marker2.is_claimed = true;
    Ok(())
}
