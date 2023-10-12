use std::str::FromStr;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, Promise, env, require, ONE_NEAR};
use near_sdk::collections::{LookupMap, LazyOption};

type BlockStripeTenantId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlockStripeTenant {
    // Tenant email.
    pub email: String,
    // Tenant account ID.
    pub account_id: String,
    // Tenant ID.
    pub tenant_id: BlockStripeTenantId
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlockStripeTenantExecutable {
    // Tenant executable ID.
    pub tenant_executable_id: BlockStripeTenantId,
    // Tenant ID.
    pub tenant_id: BlockStripeTenantId,
    // Amount of transaction executed.
    pub executable_amount: u128,
    // Counts left for executable.
    pub executable_counts_current: u128,
    // Receiver of transaction email.
    pub executable_receiver_email: String,
    // Receiver of transaction.
    pub executable_receiver_account: String
}

/// BlockStripe interface defined.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlockStripe {
    /// Tenants created by users.
    pub tenants: LookupMap<AccountId, BlockStripeTenant>,
    /// Tenant executables.
    pub tenant_executables: LookupMap<BlockStripeTenantId, BlockStripeTenantExecutable>,
    /// Trusted executable.
    trusted_executable: LazyOption<AccountId>,
}

/// BlockStripe defaults implementation.
impl Default for BlockStripe {
    fn default() -> Self {
        Self {
            tenants: LookupMap::new(b"r"),
            tenant_executables: LookupMap::new(b"r"),
            trusted_executable: LazyOption::new(b"l", None),
        }
    }
}

mod utils {
    use near_sdk::BlockHeight;

    // Returns random ID for entity using pattern `<account_id>_<block_id> (marianna.near_123456789)`.
    pub fn generate_random_id(account_id: &str, block_height: BlockHeight) -> String {
        return account_id.to_owned() + "_" + &block_height.to_string();
    }
}

const TENANT_HAS_BEEN_ALREADY_ADDED: &str  = "Tenant for this Account ID has been already added";
const NO_TENANT_FOUND_TO_EXECUTE: &str  = "No tenant found to execute";
static INCORRECT_DEPOSIT_AMOUNT: &str = "Incorrect deposit amount";
static RECIPIENT_IS_NOT_VALID: &str = "Recipient is not valid";
static TENANT_IS_NOT_EXISTS: &str = "Tenant is not exists";
static NO_TENANT_FOUND_TO_REMOVE: &str = "No tenant found to remove";
static CALLER_IS_NOT_AN_OWNER: &str = "Caller is not a owner";
static AMOUNT_LARGER_THEN_CURRENT_BALANCE: &str = "Amount to pay exceeded current contract balance";

/// BlockStripe contract.
#[near_bindgen]
impl BlockStripe {
    #[init]
    pub fn new(trusted_account: String) -> Self {
        Self {
            tenants: LookupMap::new(b"r"),
            tenant_executables: LookupMap::new(b"r"),
            trusted_executable: LazyOption::new(0, Some(
                &AccountId::from_str(&trusted_account.as_str()).unwrap()
            )),
        }
    }

    pub fn add_tenant(mut self, email: String) -> String {
        let account_id: AccountId = env::signer_account_id();

        match self.tenants.get(&account_id) {
            Some(_t) => {
                env::panic_str(&TENANT_HAS_BEEN_ALREADY_ADDED);
            },
            None => {
                let new_account_id = utils::generate_random_id(&account_id.as_str(), env::block_height());
                self.tenants.insert(&account_id, &BlockStripeTenant {
                    email,
                    account_id: account_id.as_str().to_string(),
                    tenant_id: new_account_id.to_string(),
                });

                new_account_id
            },
        }
    }

    #[payable]
    pub fn add_tenant_executable(&mut self, executable_counts: u128, executable_amount: u128, executable_recepient_account: String, executable_recepient_email: String) -> String {
        let account_id: AccountId = env::signer_account_id();
        let tx_deposit_amount: Balance = env::attached_deposit();
        let executable_total_amount_in_yocto = (executable_amount * ONE_NEAR) * executable_counts;

        require!(tx_deposit_amount >= executable_total_amount_in_yocto, &INCORRECT_DEPOSIT_AMOUNT);

        if !env::is_valid_account_id(executable_recepient_account.as_bytes()) {
            env::panic_str(&RECIPIENT_IS_NOT_VALID);
        }

        match self.tenants.get(&account_id) {
            Some(tenant_matched) => {
                let new_tenant_executable_id = utils::generate_random_id(&account_id.as_str(), env::block_height());

                self.tenant_executables.insert(&new_tenant_executable_id, &BlockStripeTenantExecutable {
                    tenant_executable_id: new_tenant_executable_id.to_string(),
                    tenant_id: tenant_matched.tenant_id,
                    executable_amount: executable_amount * ONE_NEAR,
                    executable_counts_current: executable_counts,
                    executable_receiver_account: executable_recepient_account,
                    executable_receiver_email: executable_recepient_email,
                });

                new_tenant_executable_id
            },
            None => {
                env::panic_str(&TENANT_IS_NOT_EXISTS);
            }, 
        }
    }

    pub fn cancel_executable_early(&mut self, tenant_executable_id: String) {
        match self.tenant_executables.get(&tenant_executable_id) {
            Some(_) => {
                self.tenant_executables.remove(&tenant_executable_id);
            }
            None => {
                env::panic_str(&NO_TENANT_FOUND_TO_REMOVE);
            }
        }
    }

    pub fn trigger_tenant_executable(&mut self, tenant_executable_id: String) -> Promise {
        if env::signer_account_id() == self.trusted_executable.get().unwrap() {
            match self.tenant_executables.get(&tenant_executable_id) {
                Some(mut executable_matched) => {
                    if (executable_matched.executable_counts_current - 1) == 0 {
                        self.tenant_executables.remove(&tenant_executable_id);
                    } else {
                        executable_matched.executable_counts_current = executable_matched.executable_counts_current - 1;
                        self.tenant_executables.insert(&tenant_executable_id, &executable_matched);
                    }

                    if env::account_balance() > executable_matched.executable_amount {
                        Promise::new(AccountId::from_str(executable_matched.executable_receiver_account.as_str()).unwrap())
                            .transfer(executable_matched.executable_amount)
                    } else {
                        env::panic_str(&AMOUNT_LARGER_THEN_CURRENT_BALANCE);
                    }
                },
                None => {
                    env::panic_str(&NO_TENANT_FOUND_TO_EXECUTE);
                },
            }
        } else {
            env::panic_str(&CALLER_IS_NOT_AN_OWNER);
        }
    }

    pub fn get_email_for_account(&self, tenant_account_id: String) -> String {
        let account_id = AccountId::from_str(&tenant_account_id.as_str()).unwrap();

        return self.tenants.get(&account_id).unwrap().email
    }

    pub fn get_tenant_id_for_account(&self, tenant_account_id: String) -> String {
        let account_id = AccountId::from_str(&tenant_account_id.as_str()).unwrap();

        return self.tenants.get(&account_id).unwrap().tenant_id
    }
}
