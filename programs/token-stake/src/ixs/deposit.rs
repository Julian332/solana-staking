use anchor_lang::account;
use crate::Deposit;
use anchor_lang::context::Context;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
pub const DEPOSITOR_STATE_SPACE: usize = 8 + 8;
pub const MIN_STAKING_AMOUNT: u64 = 10000;

#[account]
pub struct DepositState {
    pub depositor_lp: u64,
}
pub(crate) fn deposit(ctx: Context<Deposit>, staking_amount: u64) -> anchor_lang::Result<()> {
    assert!(staking_amount > MIN_STAKING_AMOUNT);
    let pool_token_amount = ctx.accounts.pool_token_ata.amount as u128;
    let total_lp = ctx.accounts.pool_state.total_lp as u128;
    let new_lp = staking_amount as u128 * total_lp / pool_token_amount;
    
    todo!()
}
