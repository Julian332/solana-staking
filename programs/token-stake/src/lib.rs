pub mod ixs;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::ixs::init_a_stake::StakingState;
pub const STATE: &str = "state";
declare_id!("8QmoUoRsRFnJLedRhqTUZWxyLAMuWidM8S4TAj7JQACA");

#[program]
pub mod token_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ixs::init_a_stake::initialize(ctx)
    }
}

#[derive(Accounts)] 
pub struct Initialize<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        seeds = [STATE.as_bytes()],
        bump,
        payer = creator,
        space = ixs::init_a_stake::STAKING_STATE_SPACE
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
