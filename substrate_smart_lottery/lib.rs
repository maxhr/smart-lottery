#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;

#[ink::chain_extension]
pub trait FetchRandom {
    type ErrorCode = RandomReadErr;

    /// Note: this gives the operation a corresponding `func_id` (1101 in this case),
    /// and the chain-side chain extension will get the `func_id` to do further operations.
    #[ink(extension = 1101, returns_result = false)]
    fn fetch_random(subject: [u8; 32]) -> [u8; 32];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomReadErr {
    FailGetRandomSource,
}

impl ink_env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchRandom;
}

#[ink::contract(env = crate::CustomEnvironment)]
pub mod smart_lottery {
    use super::RandomReadErr;

    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };


    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct SmartLottery {
        participants: Mapping<AccountId, (i32, Balance)>,
        prize_pool: Balance,
    }

    #[ink(event)]
    pub struct Lottery {
        #[ink(topic)]
        winner: AccountId,
        prize: Balance,
    }

    impl SmartLottery {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract)
            })
        }
        
        fn new_init(&mut self) {}

        #[ink(message)]
        pub fn play(&mut self, number: i32, amount: Balance) {
            let caller = self.env().caller();
            if self.participants.get(&caller) != None {
                return
            }
            self.prize_pool += amount;
            self.participants.insert(caller, &(number, amount));
        }

        #[ink(message)]
        pub fn get_my_play(&self) -> (i32, Balance) {
            let caller = self.env().caller();
            let caller_play = self.participants.get(&caller).unwrap_or((0, 0u128));
            caller_play
        }

        #[ink(message)]
        pub fn get_caller(&self) -> AccountId {
            self.env().caller()
        }

        #[ink(message)]
        pub fn get_pool(&self) -> Balance {
            self.prize_pool
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        #[ink::test]
        fn basic_acceptance() {
            let mut smart_lottery = SmartLottery::default();
            assert_eq!(smart_lottery.get_pool(), 0);
            smart_lottery.play(55, 1000);
            assert_eq!(smart_lottery.get_pool(), 1000);
            assert_eq!(smart_lottery.get_my_play(), (55, 1000));
            smart_lottery.play(407, 55);
            assert_eq!(smart_lottery.get_my_play(), (55, 1000));
            
        }


    }
}
