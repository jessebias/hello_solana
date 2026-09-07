use anchor_lang::prelude::*;

declare_id!("BzbkBrAU7R4BJtJfrk1ddtzcTHDxPueSTsQdJyV7VLnq");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
