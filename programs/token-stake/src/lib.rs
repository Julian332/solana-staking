pub mod ixs;

use crate::ixs::init_a_stake::StakingState;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::ixs::deposit::DepositState;

pub const POOL_STATE: &str = "pool_state";
pub const DEPOSITOR_STATE: &str = "depositor_state";
declare_id!("8QmoUoRsRFnJLedRhqTUZWxyLAMuWidM8S4TAj7JQACA");

#[program]
pub mod token_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ixs::init_a_stake::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, staking_amount: u64) -> Result<()> {
        ixs::deposit::deposit(ctx,staking_amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = ixs::init_a_stake::POOL_STATE_SPACE,
        seeds = [POOL_STATE.as_bytes(),staking_token.key().as_ref()],
        bump,

    )]
    pub pool_state: Account<'info, StakingState>,

    #[account(
        init,
        payer = creator,
        token::mint = staking_token,
        token::authority = pool_state,
    )]
    pub staking_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program,
    )]
    pub staking_token: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [POOL_STATE.as_bytes(),staking_token.key().as_ref()],
        bump,
    )]
    pub pool_state: Account<'info, StakingState>,

    #[account(
        init_if_needed,
        payer = depositor,
        space = ixs::deposit::DEPOSITOR_STATE_SPACE,
        seeds = [DEPOSITOR_STATE.as_bytes(),staking_token.key().as_ref(),depositor.key().as_ref()],
        bump,
    )]
    pub depositor_state: Account<'info, DepositState>,

    #[account(
        token::mint = staking_token,
        token::authority = depositor,
    )]
    pub depositor_token_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        token::mint = staking_token,
        token::authority = pool_state,
    )]
    pub pool_token_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mint::token_program = token_program,
    )]
    pub staking_token: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
