use anchor_lang::prelude::*;

declare_id!("FpEDYosAg83J3YkM4NxxR1hrkEG682ysLzKbxqhGVyJ6");

#[program]
pub mod token_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
