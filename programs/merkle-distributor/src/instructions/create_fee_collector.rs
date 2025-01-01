use crate::state::fee_collector::FeeCollector;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateFeeCollectorArgs {
    pub fee: u64,
}

#[derive(Accounts)]
#[instruction(args: CreateFeeCollectorArgs)]
pub struct CreateFeeCollectorContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init,
    payer = signer,
    seeds = [b"FeeCollector".as_ref()],
    space = FeeCollector::LEN,
    bump
    )]
    pub fee_collector: Box<Account<'info, FeeCollector>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_fee_collector(
    ctx: Context<CreateFeeCollectorContext>,
    args: CreateFeeCollectorArgs,
) -> Result<()> {
    let fee_collector = &mut ctx.accounts.fee_collector;
    fee_collector.bump = ctx.bumps.fee_collector;
    fee_collector.owner = ctx.accounts.signer.key();
    fee_collector.fee = args.fee;
    Ok(())
}
