use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use blockmesh_program::accounts as blockmesh_program_account;
use blockmesh_program::instruction as blockmesh_program_instruction;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_program, sysvar};

pub fn create_client_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    client: Pubkey,
) -> Instruction {
    let accounts = blockmesh_program_account::CreateClientContext {
        signer,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
        client,
    };
    let accounts = accounts.to_account_metas(None);
    let args = blockmesh_program_instruction::CreateClient {};
    Instruction {
        program_id,
        accounts,
        data: args.data(),
    }
}
