use crate::{Withdraw, POOL_STATE};
use anchor_lang::prelude::*;
use std::ops::DerefMut;

pub(crate) fn withdraw(ctx: Context<Withdraw>, lp_to_withdraw: u64) -> Result<()> {
    let depositor_state = ctx.accounts.depositor_state.deref_mut();

    assert!(depositor_state.depositor_lp >= lp_to_withdraw);

    let token_prog = ctx.accounts.token_program.to_account_info();
    let staking_token = ctx.accounts.staking_token.to_account_info();
    let pool_token_ata = ctx.accounts.pool_token_ata.to_account_info();
    let depositor_token_ata = ctx.accounts.depositor_token_ata.to_account_info();
    let decimal = ctx.accounts.staking_token.decimals;

    let pool_state = ctx.accounts.pool_state.deref_mut();
    let total_lp = pool_state.total_lp as u128;
    let pool_token_amount = ctx.accounts.pool_token_ata.amount as u128;
    let token_to_withdraw = lp_to_withdraw as u128 * pool_token_amount / total_lp;

    let pool_seed: &[&[&[u8]]] = &[&[
        POOL_STATE.as_bytes(),
        staking_token.key.as_ref(),
        &[ctx.bumps.pool_state],
    ]];

    depositor_state.depositor_lp -= lp_to_withdraw;
    let context = CpiContext::new_with_signer(
        token_prog,
        anchor_spl::token_2022::TransferChecked {
            from: pool_token_ata,
            mint: staking_token.clone(),
            to: depositor_token_ata,
            authority: ctx.accounts.pool_state.to_account_info(),
        },
        pool_seed,
    );

    anchor_spl::token_2022::transfer_checked(context, token_to_withdraw as u64, decimal)?;

    Ok(())
}
