#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::program_memory::sol_memset;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::spl_token;

pub fn vec_to_array(vec: &[u8]) -> [u8; 32] {
    let mut array = [0u8; 32]; // Initialize with zeros
    let len = vec.len().min(32); // Use the minimum of 32 or vector length
    array[..len].copy_from_slice(&vec[..len]);
    array
}

pub fn assert_ata(ata: &Pubkey, owner: &Pubkey, mint: &Pubkey) -> Result<()> {
    let real_ata = get_associated_token_address(owner, mint);
    require_keys_eq!(*ata, real_ata, ErrorCode::OwnerMismatch);
    Ok(())
}

pub fn transfer_sol<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(&from.key(), &to.key(), amount),
        &[from, to, system_program],
    )?;
    Ok(())
}

pub fn transfer_token<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            &token_program.key(),
            &from.key(),
            &to.key(),
            &owner.key(),
            &[],
            amount,
        )?,
        &[from, to, token_program, owner],
    )?;
    Ok(())
}

pub fn transfer_token_pda<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    amount: u64,
    seeds: &[&[&[u8]]],
) -> Result<()> {
    solana_program::program::invoke_signed(
        &spl_token::instruction::transfer(
            &token_program.key(),
            &from.key(),
            &to.key(),
            &owner.key(),
            &[],
            amount,
        )?,
        &[from, to, token_program, owner],
        seeds,
    )?;
    Ok(())
}

pub fn close_account(from: &mut AccountInfo, to: &mut AccountInfo) -> Result<()> {
    let amount = from.lamports();
    let size = from.try_data_len()?;
    transfer_sol_from_pda(from, to, amount)?;
    sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
    Ok(())
}

pub fn close_token_account<'a>(
    account: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    seeds: &[&[&[u8]]],
) -> Result<()> {
    let ix = spl_token::instruction::close_account(
        &spl_token::ID,
        account.key,
        destination.key,
        authority.key,
        &[],
    )?;
    solana_program::program::invoke_signed(&ix, &[account, destination, authority], seeds)?;
    Ok(())
}

pub fn transfer_sol_from_pda(
    from: &mut AccountInfo,
    to: &mut AccountInfo,
    amount: u64,
) -> Result<()> {
    let post_from = from
        .lamports()
        .checked_sub(amount)
        .ok_or(ErrorCode::NumericalOverflow)?;
    let post_to = to
        .lamports()
        .checked_add(amount)
        .ok_or(ErrorCode::NumericalOverflow)?;

    **from.try_borrow_mut_lamports().unwrap() = post_from;
    **to.try_borrow_mut_lamports().unwrap() = post_to;
    Ok(())
}
