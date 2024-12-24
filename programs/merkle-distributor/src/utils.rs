use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;

pub fn assert_ata(ata: &Pubkey, owner: &Pubkey, mint: &Pubkey) -> Result<()> {
    let real_ata = get_associated_token_address(owner, mint);
    require_keys_eq!(*ata, real_ata, ErrorCode::OwnerMismatch);
    Ok(())
}
