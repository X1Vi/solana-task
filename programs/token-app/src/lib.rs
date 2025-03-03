use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("FpEDYosAg83J3YkM4NxxR1hrkEG682ysLzKbxqhGVyJ6");

#[program]
pub mod solana_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.stake_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        ctx.accounts.stake_data.amount += amount;
        Ok(())
    }
    
    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        require!(ctx.accounts.stake_data.amount >= amount, CustomError::InsufficientStake);
        let cpi_accounts = Transfer {
            from: ctx.accounts.stake_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.stake_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        ctx.accounts.stake_data.amount -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_data: Account<'info, StakeData>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stake_data: Account<'info, StakeData>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StakeData {
    pub amount: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Insufficient staked tokens")] 
    InsufficientStake,
}