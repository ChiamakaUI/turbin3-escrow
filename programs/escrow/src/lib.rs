use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("4Phc1E7fyrYwRrVdzdYDAw8P1ZWRcjotkKYTtCdg4Zwn");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seeds: u64, receive_amount: u64, deposit_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow_state(seeds, receive_amount, ctx.bumps)?;
        ctx.accounts.deposit(deposit_amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>, seeds: u64, amount: u64,) -> Result<()> {
        ctx.accounts.transfer(amount)?;
        ctx.accounts.close_vault(seeds)?;
        Ok(())
    }
    pub fn refund(ctx: Context<Refund>, seeds: u64,)-> Result<()>  {
        ctx.accounts.close_vault(seeds)?;
        Ok(())
    }
}
