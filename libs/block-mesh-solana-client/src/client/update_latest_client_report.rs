use anchor_lang::InstructionData;
use anchor_lang::ToAccountMetas;
use blockmesh_program::instruction as blockmesh_program_instruction;
use blockmesh_program::{accounts as blockmesh_program_account, UpdateLatestClientReportArgs};
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_program, sysvar};

pub fn update_latest_client_report_instruction(
    program_id: Pubkey,
    signer: Pubkey,
    client: Pubkey,
    api_token: Pubkey,
    provider_node: Pubkey,
    latest_client_report: u64,
) -> Instruction {
    let accounts = blockmesh_program_account::UpdateLatestClientReportContext {
        signer,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
        client,
        api_token,
        provider_node,
    };
    let accounts = accounts.to_account_metas(None);
    let args = blockmesh_program_instruction::UpdateLatestClientReport {
        args: UpdateLatestClientReportArgs {
            latest_client_report,
        },
    };
    Instruction {
        program_id,
        accounts,
        data: args.data(),
    }
}
