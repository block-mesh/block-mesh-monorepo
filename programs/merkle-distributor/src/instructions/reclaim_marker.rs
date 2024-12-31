use crate::state::air_dropper::AirDropper;
use crate::state::claim_marker::ClaimMarker;
use crate::utils::{close_account, close_token_account, transfer_token_pda};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct ReclaimMarkerContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init_if_needed,
    associated_token::mint = mint,
    associated_token::authority = signer,
    payer = signer
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: can be anyone
    pub claimant: UncheckedAccount<'info>,
    #[account(
    seeds = [b"AirDropper".as_ref()],
    constraint = air_dropper.owner == signer.key(),
    bump=air_dropper.bump
    )]
    pub air_dropper: Box<Account<'info, AirDropper>>,
    #[account(mut,
    seeds = [b"ClaimMarker".as_ref(), claimant.key().as_ref()],
    constraint = claim_marker.claimant == claimant.key(),
    bump=claim_marker.bump
    )]
    pub claim_marker: Box<Account<'info, ClaimMarker>>,
    #[account(mut)]
    /// CHECK: if claimed, exists, if not, doesn't exists
    // seeds = [b"ClaimMarker2".as_ref(), claimant.key().as_ref()],
    pub claim_marker2: UncheckedAccount<'info>,
    #[account(
    mut,
    token::mint = mint,
    token::authority = air_dropper,
    seeds = [b"ClaimMarker".as_ref(), mint.key().as_ref(), claimant.key().as_ref()],
    bump
    )]
    pub claim_marker_token_account: Box<Account<'info, TokenAccount>>,
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn reclaim_marker(ctx: Context<ReclaimMarkerContext>) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let token_program = &ctx.accounts.token_program;
    let signer_token_account = &mut ctx.accounts.signer_token_account;
    let claim_marker_token_account = &mut ctx.accounts.claim_marker_token_account;
    let claim_marker = &mut ctx.accounts.claim_marker;
    let claim_marker2 = &mut ctx.accounts.claim_marker2;
    let air_dropper = &mut ctx.accounts.air_dropper;
    let seeds = &["AirDropper".as_bytes(), &[air_dropper.bump]];
    transfer_token_pda(
        claim_marker_token_account.to_account_info(),
        signer_token_account.to_account_info(),
        token_program.to_account_info(),
        air_dropper.to_account_info(),
        claim_marker_token_account.amount,
        &[seeds],
    )?;
    claim_marker_token_account.reload()?;
    air_dropper.reload()?;
    close_token_account(
        claim_marker_token_account.to_account_info(),
        signer.to_account_info(),
        air_dropper.to_account_info(),
        &[seeds],
    )?;
    if claim_marker2.lamports() > 0 {
        close_account(
            &mut claim_marker2.to_account_info(),
            &mut signer.to_account_info(),
        )?;
    }
    close_account(
        &mut claim_marker.to_account_info(),
        &mut signer.to_account_info(),
    )?;
    Ok(())
}
