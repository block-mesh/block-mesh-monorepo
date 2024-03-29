use crate::error::ErrorCode;
use crate::state::provider_node::ProviderNode;
use crate::utils::close_account;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CloseProviderNodeArgs {
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(args: CloseProviderNodeArgs)]
pub struct CloseProviderNodeContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    /// CHECK: inside
    pub provider_node: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[inline(never)]
pub fn close_provider_node(
    ctx: Context<CloseProviderNodeContext>,
    _args: CloseProviderNodeArgs,
) -> Result<()> {
    let signer = &ctx.accounts.signer;
    let provider_node = &mut ctx.accounts.provider_node;
    let (provider_node_address, _provider_node_bump) = Pubkey::find_program_address(
        &[ProviderNode::PREFIX.as_bytes(), signer.key().as_ref()],
        ctx.program_id,
    );
    let data: Vec<u8> = provider_node.data.borrow().to_vec();
    // let data= provider_node.data;
    // let binding = provider_node.data.clone();
    // let data = binding.borrow();
    let offset = 8 + std::mem::size_of::<u8>();
    let owner = &data[offset..offset + std::mem::size_of::<Pubkey>()];
    let owner = Pubkey::try_from(owner).map_err(|_| ErrorCode::InvalidData)?;
    require_keys_eq!(owner, signer.key(), ErrorCode::SignerMismatch);
    msg!("owner = {:?}", owner);
    msg!("signer = {:?}", signer.key());
    require_keys_eq!(
        provider_node_address,
        provider_node.key(),
        ErrorCode::AddressMismatch
    );
    msg!("provider_node_address = {:?}", provider_node_address);
    msg!("provider_node.key() = {:?}", provider_node.key());
    close_account(
        &mut provider_node.to_account_info(),
        &mut signer.to_account_info(),
    )?;
    Ok(())
}
