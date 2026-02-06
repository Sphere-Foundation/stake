//! Instruction builders with validator whitelist support
//!
//! These helpers automatically include the whitelist entry PDA in delegation instructions.
//! Only available when the `helpers` feature is enabled.

// Remove the following `allow` when the `Redelegate` variant is renamed to
// `Unused` starting from v3.
// Required to avoid warnings from uses of deprecated types during trait derivations.
#![allow(deprecated)]

use {
    solana_instruction::{AccountMeta, Instruction},
    solana_stake_interface::{
        instruction::{create_account, create_account_with_seed, StakeInstruction},
        program::ID,
        state::{Authorized, Lockup},
    },
    spherenet_validator_whitelist_interface::program_solana::ID as WHITELIST_PROGRAM_ID,
};
use solana_pubkey::Pubkey;

// Inline some constants to avoid dependencies.
//
// Note: replace these inline IDs with the corresponding value from
// `solana_sdk_ids` once the version is updated to 2.2.0.

const CLOCK_ID: Pubkey = Pubkey::from_str_const("SysvarC1ock11111111111111111111111111111111");

const STAKE_HISTORY_ID: Pubkey =
    Pubkey::from_str_const("SysvarStakeHistory1111111111111111111111111");

pub fn create_account_and_delegate_stake(
    from_pubkey: &Pubkey,
    stake_pubkey: &Pubkey,
    vote_pubkey: &Pubkey,
    authorized: &Authorized,
    lockup: &Lockup,
    lamports: u64,
) -> Vec<Instruction> {
    let mut instructions = create_account(from_pubkey, stake_pubkey, authorized, lockup, lamports);
    instructions.push(delegate_stake(
        stake_pubkey,
        &authorized.staker,
        vote_pubkey,
    ));
    instructions
}

#[allow(clippy::too_many_arguments)]
pub fn create_account_with_seed_and_delegate_stake(
    from_pubkey: &Pubkey,
    stake_pubkey: &Pubkey,
    base: &Pubkey,
    seed: &str,
    vote_pubkey: &Pubkey,
    authorized: &Authorized,
    lockup: &Lockup,
    lamports: u64,
) -> Vec<Instruction> {
    let mut instructions = create_account_with_seed(
        from_pubkey,
        stake_pubkey,
        base,
        seed,
        authorized,
        lockup,
        lamports,
    );
    instructions.push(delegate_stake(
        stake_pubkey,
        &authorized.staker,
        vote_pubkey,
    ));
    instructions
}

pub fn delegate_stake(
    stake_pubkey: &Pubkey,
    authorized_pubkey: &Pubkey,
    vote_pubkey: &Pubkey,
) -> Instruction {
    let account_metas = vec![
        AccountMeta::new(*stake_pubkey, false),
        AccountMeta::new_readonly(*vote_pubkey, false),
        AccountMeta::new_readonly(CLOCK_ID, false),
        AccountMeta::new_readonly(STAKE_HISTORY_ID, false),
        // Whitelist entry pubkey for stake delegation
        AccountMeta::new_readonly(
            Pubkey::find_program_address(&[&vote_pubkey.to_bytes()], &WHITELIST_PROGRAM_ID).0,
            false,
        ),
        AccountMeta::new_readonly(*authorized_pubkey, true),
    ];
    Instruction::new_with_bincode(ID, &StakeInstruction::DelegateStake, account_metas)
}
