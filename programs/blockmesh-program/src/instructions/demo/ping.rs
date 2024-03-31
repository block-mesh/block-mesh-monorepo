use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PingContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn ping(ctx: Context<PingContext>) -> Result<()> {
    let _signer = &ctx.accounts.signer;
    Ok(())
}
