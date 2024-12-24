use crate::state::air_dropper::AirDropper;
use crate::state::claim_marker::ClaimMarker;
use crate::utils::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMarkerArgs {
    pub amount: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateMarkerArgs)]
pub struct CreateMarkerContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: can be anyone
    pub claimant: UncheckedAccount<'info>,
    #[account(
    seeds = [b"AirDropper".as_ref()],
    bump=air_dropper.bump
    )]
    pub air_dropper: Box<Account<'info, AirDropper>>,
    #[account(init,
    payer = signer,
    seeds = [b"ClaimMarker".as_ref(), claimant.key().as_ref()],
    space = 100,
    bump
    )]
    pub claim_marker: Box<Account<'info, ClaimMarker>>,
    #[account(
    init,
    payer = signer,
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

pub fn create_marker(ctx: Context<CreateMarkerContext>, args: CreateMarkerArgs) -> Result<()> {
    let token_program = &ctx.accounts.token_program;
    let signer = &mut ctx.accounts.signer;
    let signer_token_account = &ctx.accounts.signer_token_account;
    let claim_marker_token_account = &ctx.accounts.claim_marker_token_account;
    let claim_marker = &mut ctx.accounts.claim_marker;
    claim_marker.bump = ctx.bumps.claim_marker;
    claim_marker.is_claimed = false;
    claim_marker.claimant = ctx.accounts.claimant.key();
    claim_marker.amount = args.amount;
    transfer_token(
        signer_token_account.to_account_info(),
        claim_marker_token_account.to_account_info(),
        token_program.to_account_info(),
        signer.to_account_info(),
        args.amount,
    )?;
    Ok(())
}
