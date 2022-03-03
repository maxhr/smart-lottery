use anchor_lang::prelude::*;
use std::collections::{HashMap};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_smart_lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let smart_lottery = &mut ctx.accounts.smart_lottery;
        smart_lottery.authority = ctx.accounts.authtority.key();
        smart_lottery.prize_pool = 0;
        smart_lottery.participants = HashMap::new();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let smart_lottery = &mut ctx.accounts.smart_lottery;
        smart_lottery.prize_pool += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = smart_lottery, space = 48)]
    pub smart_lottery: Account<'info, SmartLottery>,
    pub authtority: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = authority)]
    pub smart_lottery: Account<'info, SmartLottery>,
    pub authority: Signer<'info>,
}

#[account]
pub struct SmartLottery {
    pub authority: Pubkey,
    pub participants: HashMap<Pubkey, u64>,
    pub prize_pool: u64,
}
