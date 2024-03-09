use anchor_lang::prelude::*;

declare_id!("GzscdwWG2FwpA6iqB6yYKEESvvw773c1iAzmJatXLcve");

#[program]
pub mod blockmesh_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
