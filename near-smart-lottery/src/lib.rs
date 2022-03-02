use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance,
    collections::{ UnorderedMap },
};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SmartLottery {
    participants: UnorderedMap<AccountId, Balance>,
    prize_pool: Balance,
}

#[near_bindgen]
impl SmartLottery {

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(env::is_valid_account_id(owner_id.as_bytes()), "Invalid owner account");
        assert!(!env::state_exists(), "Already initialized");
        Self {
            prize_pool: 0,
            participants: UnorderedMap::new(b"credits".to_vec()),
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();
        let mut credits = self.participants.get(&account_id).unwrap_or(0);
        credits = credits + deposit;
        self.participants.insert(&account_id, &credits);
        // env::log_str(&format!("==== {:?}", self.participants));
        self.prize_pool += deposit;
    }

    pub fn get_random_seed(&mut self) -> u8 {
        let rand: u8 = *env::random_seed().get(0).unwrap();
        rand
    }

    pub fn get_grand_prize(&self) -> Balance {
        self.prize_pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, Balance, VMContext};
 
    fn get_context(signer_account_id: AccountId, attached_deposit: Balance) -> VMContext {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(signer_account_id.clone())
            .signer_account_id(signer_account_id.clone())
            .attached_deposit(attached_deposit.clone())
            .predecessor_account_id(accounts(1).clone())
            .account_balance(0)
            .build();
        builder.context
    }


    #[test]
    fn play() {
        let context1 = get_context(accounts(1), 115);
        testing_env!(context1);
        let mut contract = SmartLottery {
            participants: UnorderedMap::new(b"credits".to_vec()),
            prize_pool: 0,
        };
        contract.deposit();
        assert_eq!(115, contract.get_grand_prize().clone());
        assert_eq!(contract.participants.get(&accounts(1)), Some(115u128));

        let context2 = get_context(accounts(2), 23);
        testing_env!(context2);
        contract.deposit();
        assert_eq!(115 + 23, contract.get_grand_prize().clone());
        assert_eq!(contract.participants.get(&accounts(2)), Some(23u128));

        let context2a = get_context(accounts(2), 2);
        testing_env!(context2a);
        contract.deposit();
        assert_eq!(115 + 23 + 2, contract.get_grand_prize().clone());
        assert_eq!(contract.participants.get(&accounts(2)), Some(23u128 + 2u128));

        assert_eq!(contract.participants.len(), 2);
    }

}
