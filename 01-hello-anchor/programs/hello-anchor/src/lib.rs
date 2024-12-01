use anchor_lang::prelude::*;

declare_id!("J8oMGn6H5v4U4QncofK43EAudAb7NnscGSEUgR89vexj");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
