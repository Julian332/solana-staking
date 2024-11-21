use anchor_lang::account;
use anchor_lang::prelude::*;
use anchor_lang::prelude::{AccountInfo, Clock};
use anchor_lang::solana_program::sysvar::Sysvar;
use anchor_spl::token_2022::spl_token_2022;
use anchor_spl::token_2022::spl_token_2022::extension::transfer_fee::TransferFeeConfig;
use anchor_spl::token_2022::spl_token_2022::extension::{
    BaseStateWithExtensions, StateWithExtensions,
};

pub mod deposit;
pub mod init_a_stake;
pub mod withdraw;

pub fn get_transfer_fee(token: &AccountInfo, pre_fee_amount: u64) -> anchor_lang::Result<u64> {
    if *token.owner == anchor_spl::token::ID {
        return Ok(0);
    }
    let mint_data = token.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        transfer_fee_config
            .calculate_epoch_fee(Clock::get()?.epoch, pre_fee_amount)
            .unwrap()
    } else {
        0
    };
    Ok(fee)
}

#[account]
pub struct DepositorState {
    pub depositor_lp: u64,
    pub total_staked: u64,
    pub total_withdrew: u64,
}
impl DepositorState {
    pub const DEPOSITOR_STATE_SPACE: usize = 8 + 8*3;
}
#[account]
pub struct StakingState {
    pub staking_token_ata: Pubkey,
    pub staking_token: Pubkey,
    pub creator: Pubkey,
    pub total_lp: u64,
}
impl StakingState {
    pub const POOL_STATE_SPACE: usize = 8 + 8 + 32 * 3;
}
