use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use blockmesh_program::instruction as blockmesh_program_instruction;
use blockmesh_program::{accounts as blockmesh_program_account, UpdateProviderNodeArgs};
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_program, sysvar};

pub fn update_provider_node_instruction(
    program_id: Pubkey,
    ipv4: [u8; 4],
    port: u16,
    report_bandwidth_limit: u64,
    signer: Pubkey,
    provider_node: Pubkey,
) -> Instruction {
    let accounts = blockmesh_program_account::UpdateProviderNodeContext {
        signer,
        provider_node,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
    };
    let accounts = accounts.to_account_metas(None);
    let args = blockmesh_program_instruction::UpdateProviderNode {
        args: UpdateProviderNodeArgs {
            ipv4,
            port,
            report_bandwidth_limit,
        },
    };
    Instruction {
        program_id,
        accounts,
        data: args.data(),
    }
}
