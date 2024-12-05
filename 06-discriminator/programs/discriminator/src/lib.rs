use anchor_lang::prelude::*;

declare_id!("AwuoewALSVZEevDTaD61PXNxtY393SYBRpaUoBXGPfPC");

#[program]
pub mod discriminator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// Discriminator
//  A discriminator is particularly crucial for differentiating between 
//  different types of account data structures at runtime
pub trait Discriminator {
    const DISCRIMINATOR: [u8; 8];
    fn discriminator() -> [u8; 8] {
        Self::DISCRIMINATOR
    }
}

/// An account data structure capable of zero copy deserialization.
// pub trait ZeroCopy: Discriminator + Copy + Clone + Zeroable + Pod {}

/// Calculates the data for an instruction invocation, where the data is
/// `Sha256(<namespace>:<method_name>)[..8] || BorshSerialize(args)`.
/// `args` is a borsh serialized struct of named fields for each argument given
/// to an instruction.
pub trait InstructionData: Discriminator + AnchorSerialize {
    fn data(&self) -> Vec<u8> {
        let mut d = Self::discriminator().to_vec();
        d.append(&mut self.try_to_vec().expect("Should always serialize"));
        d
    }
}

/// An event that can be emitted via a Solana log. See [`emit!`](crate::prelude::emit) for an example.
pub trait Event: AnchorSerialize + AnchorDeserialize + Discriminator {
    fn data(&self) -> Vec<u8>;
}

