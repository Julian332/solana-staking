use crate::{ixs, Deposit};
use anchor_lang::context::Context;
use anchor_lang::prelude::*;
use std::ops::DerefMut;
pub const DEPOSITOR_STATE_SPACE: usize = 8 + 8;
pub const MIN_STAKING_AMOUNT: u64 = 10000;

pub(crate) fn deposit(ctx: Context<Deposit>, staking_amount: u64) -> Result<()> {
    assert!(staking_amount > MIN_STAKING_AMOUNT);
    let token_prog = ctx.accounts.token_program.to_account_info();
    let depositor = ctx.accounts.depositor.to_account_info();
    let staking_token = ctx.accounts.staking_token.to_account_info();
    let pool_token_ata = ctx.accounts.pool_token_ata.to_account_info();
    let depositor_token_ata = ctx.accounts.depositor_token_ata.to_account_info();
    let decimal = ctx.accounts.staking_token.decimals;
    let depositor_state = ctx.accounts.depositor_state.deref_mut();
    let pool_state = ctx.accounts.pool_state.deref_mut();

    let pool_token_amount = ctx.accounts.pool_token_ata.amount as u128;
    let total_lp = pool_state.total_lp as u128;

    let context = CpiContext::new(
        token_prog,
        anchor_spl::token_2022::TransferChecked {
            from: depositor_token_ata,
            mint: staking_token.clone(),
            to: pool_token_ata,
            authority: depositor,
        },
    );

    anchor_spl::token_2022::transfer_checked(context, staking_amount, decimal)?;
    let transfer_fee = ixs::get_transfer_fee(&staking_token, staking_amount)?;
    let actual_staking_amount = staking_amount - transfer_fee;
    let new_lp = if total_lp == 0 {
        actual_staking_amount as u128
    } else {
        actual_staking_amount as u128 * total_lp / pool_token_amount
    };
    pool_state.total_lp += new_lp as u64;
    depositor_state.depositor_lp += new_lp as u64;
    Ok(())
}
