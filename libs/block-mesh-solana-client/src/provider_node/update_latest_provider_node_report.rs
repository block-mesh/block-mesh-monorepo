use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use blockmesh_program::instruction as blockmesh_program_instruction;
use blockmesh_program::{
    accounts as blockmesh_program_account, UpdateLatestProviderNodeReportArgs,
};
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_program, sysvar};

pub fn update_latest_provider_node_report(
    program_id: Pubkey,
    latest_provider_node_report: u64,
    signer: Pubkey,
    provider_node: Pubkey,
    api_token: Pubkey,
    client: Pubkey,
) -> Instruction {
    let accounts = blockmesh_program_account::UpdateLatestProviderNodeReportContext {
        signer,
        api_token,
        client,
        provider_node,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
    };
    let accounts = accounts.to_account_metas(None);
    let args = blockmesh_program_instruction::UpdateLatestProviderNodeReport {
        args: UpdateLatestProviderNodeReportArgs {
            latest_provider_node_report,
        },
    };
    Instruction {
        program_id,
        accounts,
        data: args.data(),
    }
}
