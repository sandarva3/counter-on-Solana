use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut counter: i64 = 0;

        for i in 0..10 {
            counter += 1;
            msg!("Counter: {}", counter);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
