#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod smart_lottery {
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct SmartLottery {
        participants: HashMap<AccountId, (i32, Balance)>,
        prize_pool: Balance,
    }

    #[ink(event)]
    pub struct Lottery {
        prize: Balance,
    }

    impl SmartLottery {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self { 
                participants: Default::default(),
                prize_pool: 0,
            }
        }

        #[ink(message)]
        pub fn play(&mut self, number: i32, amount: Balance) {
            let caller = self.env().caller();
            if self.participants.contains_key(&caller) {
                return
            }
            self.prize_pool += amount;
            self.participants.insert(caller, (number, amount));
        }

        #[ink(message)]
        pub fn get_my_play(&self) -> (i32, Balance) {
            let caller = self.env().caller();
            let caller_play = self.participants.get(&caller).unwrap_or(&(0, 0u128));
            *caller_play
        }

        #[ink(message)]
        pub fn get_caller(&self) -> AccountId {
            self.env().caller()
        }

        #[ink(message)]
        pub fn get_pool(&self) -> Balance {
            self.prize_pool
        }

        #[ink(message)]
        pub fn do_lottery(&self) {
            self.env().emit_event(Lottery {
                prize: self.prize_pool,
            });
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
