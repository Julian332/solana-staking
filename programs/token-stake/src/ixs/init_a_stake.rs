use std::ops::DerefMut;
use anchor_lang::prelude::*;
use crate::Initialize;

//todo
pub const STAKING_STATE_SPACE: usize = 8 + 32 * 2 + 8;
#[account]
pub struct StakingState {
    pub staking_token_ata: Pubkey,
    pub staking_token: Pubkey,
    pub total_lp: u64,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // msg!("Greetings from: {:?}", ctx.program_id);
    let state = ctx.accounts.pool_state.deref_mut();
    state.staking_token_ata = ctx.accounts.staking_token_ata.key();
    state.staking_token = ctx.accounts.staking_token.key();
    state.total_lp = 0;
    Ok(())
}