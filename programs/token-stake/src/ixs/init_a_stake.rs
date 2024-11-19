use crate::Initialize;
use anchor_lang::prelude::*;
use std::ops::DerefMut;

//todo

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    // msg!("Greetings from: {:?}", ctx.program_id);
    let state = ctx.accounts.pool_state.deref_mut();
    state.staking_token_ata = ctx.accounts.staking_token_ata.key();
    state.staking_token = ctx.accounts.staking_token.key();
    state.creator = ctx.accounts.creator.key();
    Ok(())
}
