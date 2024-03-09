#![allow(dead_code)]
#![allow(unused_variables)]

use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::program_memory::sol_memset;
use anchor_spl::token::spl_token;
use std::collections::HashSet;
use std::iter::FromIterator;

pub const DENOM: f64 = 10_000.0;

#[inline(never)]
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

#[inline(never)]
pub fn close_account(from: &mut AccountInfo, to: &mut AccountInfo) -> Result<()> {
    let amount = from.lamports();
    let size = from.try_data_len()?;
    transfer_sol_from_pda(from, to, amount)?;
    sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
    Ok(())
}

#[inline(never)]
pub fn is_native(token_mint: &AccountInfo) -> bool {
    token_mint.key() == spl_token::native_mint::id()
}

#[inline(never)]
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

#[inline(never)]
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

#[inline(never)]
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

pub fn vec_to_set<T>(data: &[T]) -> HashSet<T>
where
    T: Clone + Eq + std::hash::Hash,
{
    HashSet::from_iter(data.iter().cloned())
}

pub fn set_to_vec<T>(data: &HashSet<T>) -> Vec<T>
where
    T: Clone,
{
    Vec::from_iter(data.iter().cloned())
}

pub fn u64_to_f64(value: u64) -> f64 {
    value as f64 / DENOM
}

pub fn f64_to_u64(value: f64) -> u64 {
    (value * DENOM) as u64
}
