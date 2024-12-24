use crate::state::air_dropper::AirDropper;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct CreateAirDropperContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,
    payer = signer,
    seeds = [b"AirDropper".as_ref()],
    space = 100,
    bump
    )]
    pub air_dropper: Box<Account<'info, AirDropper>>,
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_air_dropper(ctx: Context<CreateAirDropperContext>) -> Result<()> {
    let air_dropper = &mut ctx.accounts.air_dropper;
    air_dropper.bump = ctx.bumps.air_dropper;
    air_dropper.owner = ctx.accounts.signer.key();
    Ok(())
}
