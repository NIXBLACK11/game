use anchor_lang::prelude::*;

declare_id!("5tufr7LfcbwofMCV8iJSB15K9vYD1XteQzK3GGmygsax");

#[program]
pub mod game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
