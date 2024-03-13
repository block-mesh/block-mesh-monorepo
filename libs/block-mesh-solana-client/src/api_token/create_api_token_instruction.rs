use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use blockmesh_program::accounts as blockmesh_program_account;
use blockmesh_program::instruction as blockmesh_program_instruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_program, sysvar};

pub fn create_api_token_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    client: Pubkey,
    api_token: Pubkey,
    provider_node: Pubkey,
) -> Instruction {
    let accounts = blockmesh_program_account::CreateApiTokenContext {
        signer,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
        client,
        api_token,
        provider_node,
    };
    let accounts = accounts.to_account_metas(None);
    let args = blockmesh_program_instruction::CreateApiToken {};
    Instruction {
        program_id,
        accounts,
        data: args.data(),
    }
}
