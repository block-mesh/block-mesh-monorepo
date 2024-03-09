use crate::state::client::Client;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateClientContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = Client::SIZE,
    seeds = [Client::PREFIX.as_bytes(), signer.key().as_ref()],
    bump
    )]
    pub client: Box<Account<'info, Client>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn create_client(ctx: Context<CreateClientContext>) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let client = &mut ctx.accounts.client;
    client.bump = ctx.bumps.client;
    client.owner = signer.key();
    Ok(())
}
