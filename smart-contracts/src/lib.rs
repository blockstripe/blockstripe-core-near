use std::str::FromStr;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, Balance, Promise, env, require, log, ONE_NEAR};
use near_sdk::collections::{Vector, LookupMap};

type BlockStripeTenantId = String;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlockStripeTenant {
    pub email: String,
    pub account_id: String,
    pub tenant_id: BlockStripeTenantId
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlockStripeTenantExecutable {
    pub tenant_executable_id: BlockStripeTenantId,
    pub tenant_id: BlockStripeTenantId,
    /// Amount of transaction executed.
    pub executable_amount: u128,
    /// Counts left for executable.
    pub executable_counts_current: u128,
    /// Receiver of transaction.
    pub executable_receiver_account: String
}

/// BlockStripe interface defined.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BlockStripe {
    /// Tenants created by users.
    pub tenants: LookupMap<AccountId, BlockStripeTenant>,
    /// Tenant executables.
    pub tenant_executables: LookupMap<AccountId, Vector<BlockStripeTenantExecutable>>,
}

/// BlockStripe defaults implementation.
impl Default for BlockStripe {
    fn default() -> Self {
        Self {
            tenants: LookupMap::new(b"r"),
            tenant_executables: LookupMap::new(b"r"),
        }
    }
}

mod utils {
    // Returns random ID for entity using pattern `<account_id>_<block_id> (marianna.near_123456789)`.
    pub fn generate_random_id(account_id: &str, random_seed_from_block: Vec<u8>) -> String {
        let block_id_as_str = String::from_utf8(random_seed_from_block).unwrap_or("".to_owned());

        return account_id.to_owned() + "_" + &block_id_as_str.to_owned();
    }
}

/// BlockStripe contract.
#[near_bindgen]
impl BlockStripe {
    #[payable]
    pub fn add_tenant_executable(&mut self, executable_counts: u128, executable_amount: u128, executable_recepient: String) {
        let account_id: AccountId = env::signer_account_id();
        let tx_deposit_amount: Balance = env::attached_deposit();
        let executable_amount_in_yocto = (executable_amount * executable_counts) * ONE_NEAR;

        log!("add_tenant_executable(): account id {}", account_id.as_str());
        require!(tx_deposit_amount >= executable_amount_in_yocto, "Deposit amount should be higher or equal to executable amount");

        if !env::is_valid_account_id(executable_recepient.as_bytes()) {
            env::panic_str("Account ID (recepient) is not valid");
        }

        match self.tenants.get(&account_id) {
            Some(_t) => {
                let executables_for_tenant = &mut self.tenant_executables.get(&account_id).unwrap_or(Vector::new(vec![]));

                executables_for_tenant.push(&BlockStripeTenantExecutable {
                    tenant_executable_id: utils::generate_random_id(&account_id.as_str(), env::random_seed()),
                    tenant_id: _t.tenant_id,
                    executable_amount: executable_amount_in_yocto,
                    executable_counts_current: executable_counts,
                    executable_receiver_account: executable_recepient
                });

                self.tenant_executables.insert(&account_id, &executables_for_tenant);
            },
            None => {
                env::panic_str("Account ID do not exists - should be added first before add a new executable for it");
            }, 
        }
    }

    pub fn add_tenant(mut self, email: String) {
        let account_id: AccountId = env::signer_account_id();

        log!("add_tenant(): account id {}", account_id.as_str());

        match self.tenants.get(&account_id) {
            Some(_t) => {
                env::panic_str("Account ID already has a tenant");
            },
            None => {
                self.tenants.insert(&account_id, &BlockStripeTenant {
                    email,
                    account_id: account_id.as_str().to_string(),
                    tenant_id: utils::generate_random_id(&account_id.as_str(), env::random_seed())
                });
            },
        }
    }

    pub fn trigger_tenant_executable(&mut self, tenant_executable_id: String, account_id_as_str: String) -> Promise {
        log!("trigger_executable(): for executable {}", tenant_executable_id);

        let account_id = AccountId::from_str(account_id_as_str.as_str()).unwrap();

        match self.tenant_executables.get(&account_id) {
            Some(ex_collection) => {
                if ex_collection.len() > 0 {
                    let boxed_ex =
                        ex_collection.iter().find(|i| i.tenant_executable_id == tenant_executable_id);
                    
                    match boxed_ex {
                        Some(ex) => {
                            if (ex.executable_counts_current - 1) == 0 {
                                self.tenant_executables.remove(&account_id);
                                // TODO: Attach flag to destory a executable cron.
                            }

                            Promise::new(AccountId::from_str(ex.executable_receiver_account.as_str()).unwrap())
                                .transfer(ex.executable_amount)
                        }
                        None => {
                            env::panic_str("Nothing to execute (no execution with ID present)")
                        }
                    }
                } else {
                    env::panic_str("Nothing to execute (empty executions)");
                }
            },
            None => {
                env::panic_str("Nothing to execute (no executions for tenant)");
            },
        }
    }
}

// BlockStripe unit tests.
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_tenant_flow() {
        //
    }

    #[test]
    fn add_tenant_executable_flow() {
        //
    }

    #[test]
    fn execute_tenant_executable_flow() {
        //
    }
}
