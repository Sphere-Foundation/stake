use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_signer::Signer;
use spherenet_validator_whitelist_interface::state::{
    account::ValidatorWhitelistAccount, whitelist_entry::ValidatorWhitelistEntry, AccountState,
    SizeOf,
};

use solana_account::AccountSharedData;
use solana_genesis_config::GenesisConfig;

pub fn add_validator_whitelist_account_to_genesis_config(
    genesis_config: &mut GenesisConfig,
    authority: &Pubkey,
    vote_accounts_count: u32,
) {
    let data_size = ValidatorWhitelistAccount::SIZE_OF;

    let non_zero_rent = match genesis_config.rent.lamports_per_byte_year {
        0 => &Rent::default(),
        _ => &genesis_config.rent,
    };

    let lamports = Rent::minimum_balance(non_zero_rent, data_size);

    let mut validator_whitelist_account = AccountSharedData::new(
        lamports,
        data_size,
        &spherenet_validator_whitelist_interface::program_solana::id(),
    );

    let validator_whitelist_account_data = ValidatorWhitelistAccount {
        authority: authority.to_bytes(),
        pending_authority: Pubkey::default().to_bytes(),
        validator_amount: vote_accounts_count.to_le_bytes(),
        state: AccountState::Initialized as u8,
    }
    .pack();

    validator_whitelist_account.set_data_from_slice(&validator_whitelist_account_data);

    genesis_config.add_account(
        spherenet_validator_whitelist_interface::account_solana::id(),
        validator_whitelist_account,
    );
}

pub fn add_validator_whitelist_entry_to_genesis_config(
    genesis_config: &mut GenesisConfig,
    vote_keypair: &Keypair,
) -> Pubkey {
    let whitelist_entry_pubkey = Pubkey::find_program_address(
        &[vote_keypair.pubkey().as_ref()],
        &spherenet_validator_whitelist_interface::program_solana::id(),
    )
    .0;

    let whitelist_entry_data = ValidatorWhitelistEntry {
        pubkey: vote_keypair.pubkey().to_bytes(),
        start_epoch: 0u64.to_le_bytes(),
        end_epoch: u64::MAX.to_le_bytes(),
        state: AccountState::Initialized as u8,
    }
    .pack();

    let non_zero_rent = match genesis_config.rent.lamports_per_byte_year {
        0 => &Rent::default(),
        _ => &genesis_config.rent,
    };

    let mut whitelist_entry_account = AccountSharedData::new(
        Rent::minimum_balance(non_zero_rent, ValidatorWhitelistEntry::SIZE_OF),
        ValidatorWhitelistEntry::SIZE_OF,
        &spherenet_validator_whitelist_interface::program_solana::ID,
    );
    whitelist_entry_account.set_data_from_slice(&whitelist_entry_data);

    // Insert the whitelist entry into the genesis config
    genesis_config.add_account(whitelist_entry_pubkey, whitelist_entry_account);

    whitelist_entry_pubkey
}
