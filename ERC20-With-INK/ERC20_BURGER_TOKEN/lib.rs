#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod burger {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    
    };

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, initial_supply);
            let instance = Self {
                total_supply: initial_supply,
                balances,
                allowances: StorageHashMap::new(),
            };
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_or_zero(&owner)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_of_or_zero(&owner, &spender)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            true
        }

        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> bool {
            let caller = self.env().caller();
            let allowance = self.allowance_of_or_zero(&from, &caller);
            if allowance < value {
                return false
            }
            self.allowances.insert((from, caller), allowance - value);
            self.transfer_from_to(from, to, value)
        }

        fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> bool {
            let from_balance = self.balance_of_or_zero(&from);
            if from_balance < value {
                return false
            }
            self.balances.insert(from, from_balance - value);
            let to_balance = self.balance_of_or_zero(&to);
            self.balances.insert(to, to_balance + value);
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            true
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(owner).unwrap_or(&0)
        }

        fn allowance_of_or_zero(
            &self,
            owner: &AccountId,
            spender: &AccountId,
        ) -> Balance {
            *self.allowances.get(&(*owner, *spender)).unwrap_or(&0)
        }
    }

    
    #[cfg(test)]
    mod tests {
        
        use super::*;
        use ink_env::{
            hash::{
                Blake2x256,
                CryptoHash,
                HashOutput,
            },
            Clear,
        };

        type Event = <Erc20 as ::ink_lang::BaseEvent>::Type;

        use ink_lang as ink;

        fn assert_transfer_event(
            event: &ink_env::test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            if let Event::Transfer(Transfer { from, to, value }) = decoded_event {
                assert_eq!(from, expected_from, "encountered invalid Transfer.from");
                assert_eq!(to, expected_to, "encountered invalid Transfer.to");
                assert_eq!(value, expected_value, "encountered invalid Trasfer.value");
            } else {
                panic!("encountered unexpected event kind: expected a Transfer event")
            }
            fn encoded_into_hash<T>(entity: &T) -> Hash
                where
                    T: scale::Encode,
            {
                let mut result = Hash::clear();
                let len_result = result.as_ref().len();
                let encoded = entity.encode();
                let len_encoded = encoded.len();
                if len_encoded <= len_result {
                    result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                    return result
                }
                let mut hash_output =
                    <<Blake2x256 as HashOutput>::Type as Default>::default();
                <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
                let copy_len = core::cmp::min(hash_output.len(), len_result);
                result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
                result
            }
            let expected_topics = vec![
                encoded_into_hash(b"Erc20::Transfer"),
                encoded_into_hash(&expected_from),
                encoded_into_hash(&expected_to),
                encoded_into_hash(&expected_value),
            ];
            for (n, (actual_topic, expected_topic)) in
            event.topics.iter().zip(expected_topics).enumerate()
            {
                let topic = actual_topic
                    .decode::<Hash>()
                    .expect("encountered invalid topic encoding");
                assert_eq!(topic, expected_topic, "encountered invalid topic at {}", n);
            }
        }

        
        #[ink::test]
        fn new_works() {
            
            let _burger = Erc20::new(100);

            
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        
        #[ink::test]
        fn total_supply_works() {
            
            let burger = Erc20::new(100);
            
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            
            assert_eq!(burger.total_supply(), 100);
        }

        
        #[ink::test]
        fn balance_of_works() {
            
            let burger = Erc20::new(100);
            
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            
            assert_eq!(burger.balance_of(accounts.alice), 100);
            
            assert_eq!(burger.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            
            let mut burger = Erc20::new(100);
            
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(burger.balance_of(accounts.bob), 0);
            
            assert_eq!(burger.transfer(accounts.bob, 10), true);
            
            assert_eq!(burger.balance_of(accounts.bob), 10);

            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 2);
            
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        #[ink::test]
        fn invalid_transfer_should_fail() {
            
            let mut burger = Erc20::new(100);
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            assert_eq!(burger.balance_of(accounts.bob), 0);
            
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); 
            data.push_arg(&accounts.bob);
            
            assert_eq!(
                ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                    accounts.bob,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );

            
            assert_eq!(burger.transfer(accounts.eve, 10), false);
            
            assert_eq!(burger.balance_of(accounts.alice), 100);
            assert_eq!(burger.balance_of(accounts.bob), 0);
            assert_eq!(burger.balance_of(accounts.eve), 0);

            
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        #[ink::test]
        fn transfer_from_works() {
            
            let mut burger = Erc20::new(100);
            
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            
            assert_eq!(burger.transfer_from(accounts.alice, accounts.eve, 10), false);
            
            assert_eq!(burger.approve(accounts.bob, 10), true);

            
            assert_eq!(ink_env::test::recorded_events().count(), 2);

            
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            
            let mut data =
                ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])); 
            data.push_arg(&accounts.bob);
            
            assert_eq!(
                ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
                    accounts.bob,
                    callee,
                    1000000,
                    1000000,
                    data
                ),
                ()
            );

            
            assert_eq!(burger.transfer_from(accounts.alice, accounts.eve, 10), true);
            
            assert_eq!(burger.balance_of(accounts.eve), 10);

            
            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }
    }
}