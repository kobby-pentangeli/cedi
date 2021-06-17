use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, metadata, near_bindgen, setup_alloc};

use std::collections::HashMap;

setup_alloc!();

metadata! {
    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct Cedi {
        balances: HashMap<Vec<u8>, u64>,
        allowances: HashMap<Vec<u8>, u64>,
        pub owner: Vec<u8>,
        pub ticker: String,
        pub max_supply: u64,
    }
}

#[near_bindgen]
impl Cedi {
    pub fn transfer(&mut self, to: &[u8], amount: u64) -> bool {
        let from_id = env::signer_account_pk();
        let to_id = to.to_vec();
        let from_bal = self.balances.get(&from_id).unwrap_or(&0);
        let to_bal = self.balances.get(&to_id).unwrap_or(&0);

        if from_bal < &amount {
            return false;
        }

        let new_from_bal = from_bal - amount;
        let new_to_bal = to_bal + amount;
        self.balances.insert(from_id, new_from_bal);
        self.balances.insert(to_id, new_to_bal);
        true
    }

    pub fn transfer_from(&mut self, from: &[u8], to: &[u8], amount: u64) -> bool {
        let from_id = from.to_vec();
        let spender_id = env::signer_account_pk();
        let to_id = to.to_vec();
        let id = [from_id.clone(), spender_id].concat();
        let from_bal = self.get_balance_of(from);
        let to_bal = self.get_balance_of(to);
        let spender_allowance = self.allowances.get(&id).unwrap_or(&0);

        if from_bal < &amount {
            return false;
        } else if spender_allowance < &amount {
            return false;
        }

        let new_allowance = spender_allowance - amount;
        let new_from_bal = from_bal - amount;
        let new_to_bal = to_bal + amount;
        self.allowances.insert(id, new_allowance);
        self.balances.insert(from_id, new_from_bal);
        self.balances.insert(to_id, new_to_bal);
        true
    }

    pub fn set_allowance(&mut self, spender: &[u8], allowance: u64) {
        let id = [env::signer_account_pk(), spender.to_vec()].concat();
        self.allowances.insert(id, allowance);
    }

    pub fn get_allowance_of(&self, owner: &[u8], spender: &[u8]) -> &u64 {
        let id = [owner.to_vec(), spender.to_vec()].concat();
        self.allowances.get(&id).unwrap_or(&0)
    }

    pub fn get_balance_of(&self, owner: &[u8]) -> &u64 {
        let owner = owner.to_vec();
        self.balances.get(&owner).unwrap_or(&0)
    }
}

impl Default for Cedi {
    fn default() -> Self {
        let mut balances = HashMap::new();
        let max_supply: u64 = 500_000__000;
        balances.insert(env::signer_account_pk(), max_supply);
        Self {
            balances,
            allowances: HashMap::new(),
            owner: env::signer_account_pk(),
            ticker: String::from("Cedi"),
            max_supply,
        }
    }
}
