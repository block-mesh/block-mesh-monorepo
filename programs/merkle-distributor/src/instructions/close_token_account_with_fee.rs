use crate::state::fee_collector::FeeCollector;
use crate::utils::{close_token_account, transfer_sol};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CloseTokenAccountWithFeeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init_if_needed,
    associated_token::mint = mint,
    associated_token::authority = signer,
    payer = signer
    )]
    pub signer_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut,
    seeds = [b"FeeCollector".as_ref()],
    bump = fee_collector.bump
    )]
    pub fee_collector: Box<Account<'info, FeeCollector>>,
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn close_token_account_with_fee(ctx: Context<CloseTokenAccountWithFeeContext>) -> Result<()> {
    let signer = &mut ctx.accounts.signer;
    let fee_collector = &mut ctx.accounts.fee_collector;
    let system_program = &ctx.accounts.system_program;
    let signer_token_account = &mut ctx.accounts.signer_token_account;
    let before = signer.lamports();
    let x = signer_token_account.to_account_info().lamports();
    sol_log(&format!("before = {} | x = {}", before, x));
    close_token_account(
        signer_token_account.to_account_info(),
        signer.to_account_info(),
        signer.to_account_info(),
        &[],
    )?;
    let after = signer.lamports();
    let diff = after - before;

    let fee = ((diff as f64) * fee_collector.fee_per()).floor() as u64;

    sol_log(&format!(
        "X fee = {} | fee = {} | fee = {}",
        fee_collector.fee,
        fee_collector.fee_per(),
        fee
    ));
    sol_log(&format!("after = {} | diff = {}", after, diff));

    transfer_sol(
        signer.to_account_info(),
        fee_collector.to_account_info(),
        system_program.to_account_info(),
        fee,
    )?;
    Ok(())
}
