#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Incrementer {
        value: i32,
        my_value: ink_storage::Mapping<AccountId, i32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = init_value;
                let caller = Self::env().caller();
                contract.my_value.insert(&caller, &0);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = Default::default();
            })
        }

        #[ink(message)]
        pub fn increment(&mut self, by: i32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value.get(&self.env().caller()).unwrap_or_default()
        }

        #[ink(message)]
        pub fn increment_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            let my_value = self.get_mine();
            let new_value = my_value + by;
            self.my_value.insert(caller, &new_value);
        }

        #[ink(message)]
        pub fn remove_mine(&self) {
            self.my_value.remove(&self.env().caller())
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(5);
            assert_eq!(contract.get(), 5);

            contract.increment(1);
            assert_eq!(contract.get(), 6);

            contract.increment(20);
            assert_eq!(contract.get(), 26);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(1);
            assert_eq!(contract.get(), 1);
            assert_eq!(contract.get_mine(), 0);

            contract.increment_mine(10);
            assert_eq!(contract.get_mine(), 10);
        }

        #[ink::test]
        fn increment_mine_works() {
            let mut contract = Incrementer::new(55);
            assert_eq!(contract.get_mine(), 0);
            contract.increment_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.increment_mine(6);
            assert_eq!(contract.get_mine(), 11);
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract = Incrementer::new(55);
            assert_eq!(contract.get_mine(), 0);
            contract.increment_mine(20);
            assert_eq!(contract.get_mine(), 20);
            contract.remove_mine();
            assert_eq!(contract.get_mine(), 0);
        }
    }
}
