#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::spl_token;

pub fn vec_to_array(vec: &Vec<u8>) -> [u8; 32] {
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
