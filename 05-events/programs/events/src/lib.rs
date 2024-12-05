use anchor_lang::prelude::*;

declare_id!("AHxUGv3gDPYhq5wST1wWwoTQ3xPutNbbDxg433phVzm6");


#![allow(unused)]
fn main() {
#[program]
pub mod my_program {
    use super::*;

    pub fn transfer(ctx: Context<TransferContext>, amount: u64) -> Result<()>  {
        // Perform transfer logic

        // Emit the TransferEvent
        emit!(TransferEvent {
            from: *ctx.accounts.from.key,
            to: *ctx.accounts.to.key,
            amount,
        });

        Ok(())
    }
}
}


#![allow(unused)]
fn main() {
#[event]
pub struct TransferEvent {
    from: Pubkey,
    to: Pubkey,
    amount: u64,
}
}


#[derive(Accounts)]
pub struct Initialize {}
