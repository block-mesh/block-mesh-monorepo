use crate::state::token_usage::TokenUsage;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateTokenUsageContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = TokenUsage::SIZE,
    seeds = [TokenUsage::PREFIX.as_bytes(), signer.key().as_ref()],
    bump
    )]
    pub token_usage: Box<Account<'info, TokenUsage>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn create_token_usage(ctx: Context<CreateTokenUsageContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let token_usage = &mut ctx.accounts.token_usage;
    token_usage.bump = ctx.bumps.token_usage;
    token_usage.owner = signer.key();
    Ok(())
}
