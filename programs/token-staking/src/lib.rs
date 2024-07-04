use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use solana_program::{
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::program_pack::Pack;

use std::str::FromStr;
// use lazy_static::lazy_static;



declare_id!("74zrM9avnd5Dw3e9VxyZKqkgVRv63bxGhg9kjPpP8FjW");



#[program]
pub mod token_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let staking_account = &mut ctx.accounts.staking_account;
        staking_account.owner = owner;
        staking_account.token_mint = ctx.accounts.token_mint.key();
        Ok(())
    }

    pub fn stake24_m(ctx: Context<Stake>, amount: u64) -> Result<()> {
        private::stake(ctx, amount)
    }

    pub fn stake36_m(ctx: Context<Stake>, amount: u64) -> Result<()> {
        private::stake(ctx, amount)
    }

    pub fn stake60_m(ctx: Context<Stake>, amount: u64) -> Result<()> {
        private::stake(ctx, amount)
    }

    pub fn unstake30(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        private::unstake(ctx, amount)
    }

    pub fn unstake40(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        private::unstake(ctx, amount)
    }

    mod private {
        use super::*;

        pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
            let staking_account = &ctx.accounts.staking_account;

            if ctx.accounts.from.mint != staking_account.token_mint {
                msg!("Invalid token mint for the from account");
                return Err(ProgramError::InvalidArgument.into());
            }
            if ctx.accounts.to.mint != staking_account.token_mint {
                msg!("Invalid token mint for the to account");
                return Err(ProgramError::InvalidArgument.into());
            }

            let cpi_accounts = Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;
            Ok(())
        }

        pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
            let staking_account = &ctx.accounts.staking_account;

            if ctx.accounts.from.mint != staking_account.token_mint {
                msg!("Invalid token mint for the from account");
                return Err(ProgramError::InvalidArgument.into());
            }
            if ctx.accounts.to.mint != staking_account.token_mint {
                msg!("Invalid token mint for the to account");
                return Err(ProgramError::InvalidArgument.into());
            }

            if ctx.accounts.owner.key() != staking_account.owner {
                msg!("Account owner mismatch");
                return Err(ProgramError::IllegalOwner.into());
            }

            let cpi_accounts = Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            token::transfer(cpi_ctx, amount)?;
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32)]
    pub staking_account: Account<'info, StakingAccount>,
    pub token_mint: Account<'info, token::Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub user: Signer<'info>,
    #[account(address = Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap())]
    pub token_program: AccountInfo<'info>,
    #[account(mut)]
    pub staking_account: Account<'info, StakingAccount>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut, has_one = owner)]
    pub staking_account: Account<'info, StakingAccount>,
    #[account(signer, address = staking_account.owner)]
    pub owner: AccountInfo<'info>,
    #[account(address = Pubkey::from_str("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb").unwrap())]
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub token_mint: Pubkey,
    
}
