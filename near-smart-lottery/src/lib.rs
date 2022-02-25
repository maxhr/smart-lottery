use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct SmartLottery {
    participants: HashMap<AccountId, Balance>,
    prize_pool: Balance,
}

#[near_bindgen]
impl SmartLottery {

    #[payable]
    pub fn play(&mut self) {
        let amount = env::attached_deposit();
        let existing_amount = self.get_existing_amount_for_account(env::signer_account_id());
        self.participants.insert(env::signer_account_id(), amount + existing_amount);
        self.prize_pool += amount;
    }

    fn get_existing_amount_for_account(&self, account_id: AccountId) -> Balance {
        let default: Balance = 0;
        let existing_amount = self.participants.get(&account_id).unwrap_or(&default);
        *existing_amount
    }

    pub fn get_prize(&self) -> Balance {
        self.prize_pool
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts};
    use near_sdk::{testing_env, AccountId, Balance, VMContext};
 
    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(signer_account_id: AccountId, attached_deposit: Balance) -> VMContext {
        // let mut builder = VMContextBuilder::new();
        VMContext {
            current_account_id: signer_account_id.to_string(),
            signer_account_id: signer_account_id.to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: accounts(1).to_string(),
            input: Vec::new(),
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }


    #[test]
    fn play() {
        let context1 = get_context(accounts(1), 115);
        testing_env!(context1);
        let mut contract = SmartLottery {
            participants: HashMap::new(),
            prize_pool: 0,
        };
        contract.play();
        // env::log_str(&contract.get_prize().to_string());
        assert_eq!(115, contract.get_prize().clone());
        assert_eq!(contract.participants.contains_key(&accounts(1)), true);
        assert_eq!(*contract.participants.get(&accounts(1)).unwrap(), 115u128);

        let context2 = get_context(accounts(2), 23);
        testing_env!(context2);
        contract.play();
        assert_eq!(115 + 23, contract.get_prize().clone());
        assert_eq!(contract.participants.contains_key(&accounts(2)), true);
        assert_eq!(*contract.participants.get(&accounts(2)).unwrap(), 23u128);

        let context2a = get_context(accounts(2), 2);
        testing_env!(context2a);
        contract.play();
        assert_eq!(115 + 23 + 2, contract.get_prize().clone());
        assert_eq!(*contract.participants.get(&accounts(2)).unwrap(), 23u128 + 2u128);

        assert_eq!(contract.participants.len(), 2);
    }

}
// near deploy --wasmFile target/wasm32-unknown-unknown/release/smart_lottery.wasm --accountId mx2122.testnet