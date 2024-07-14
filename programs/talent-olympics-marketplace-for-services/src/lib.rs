use anchor_lang::prelude::*;

declare_id!("C19PZG14VjxjmqGQqARnC3nvydYM8TcmydANZr3Zc8Uq");

#[program]
pub mod talent_olympics_marketplace_for_services {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
