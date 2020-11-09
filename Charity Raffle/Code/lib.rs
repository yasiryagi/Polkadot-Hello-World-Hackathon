#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod charity_raffle_burgerking {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::HashMap as StorageHashMap,
    };

    use ink_env::{
        hash::{
            Keccak256, // this  made ma day
        }
    };
    
    const MIN_ENTRIES: u64 = 5; // minimo numero
    const MAX_DRAWS: u64 = 2; // maxi drawer
    const RUN_TIME: u64 = 900000; //60k * 15
    const MIN_PRICE: u128 = 10000000000000; //10 (12)  // 0.01
    const MAX_PRICE: u128 = 100000000000000;//100  (12) // 0.1

    #[ink(storage)]
    pub struct CharityRaffleBurgerking {
        charity_pot: AccountId,
        entries: StorageHashMap<u64, AccountId>,
        entrants: StorageHashMap<AccountId, u64>,
        winners: [Option<AccountId>; MAX_DRAWS as usize],
        funds: u128,
        draw_tickets: u64,
        draws: u64,
        start_time: Timestamp,
        end_time: Timestamp
        
    }

    impl CharityRaffleBurgerking {
        #[ink(constructor)]
        pub fn new(charity_pot: AccountId) -> Self {
            Self { 
                charity_pot: charity_pot,
                funds: 0,
                draw_tickets: 0,
                draws: 0,
                start_time: 0,
                end_time: 0,
                entrants: StorageHashMap::new(),
                entries: StorageHashMap::new(),
                winners: [None, None]
            }
        }
        fn now(&self) -> Timestamp {
            self.env().block_timestamp()
        }

        fn do_random(&self) -> u64 {
            let mut output: u64 = 0;
            let encodable = [self.now(), self.start_time, self.end_time, self.draw_tickets, self.draws];
            let encoded = self.env().hash_encoded::<Keccak256, _>(&encodable);
            let mut hashed = self.env().random(&encoded);
            let random = hashed.as_mut();
            for do_random in random.iter() {
                output += *do_random as u64;
            }
            output
        }
        #[ink(message)]
        #[ink(payable)]
        pub fn join_the_charity(&mut self) {
            // use std::time::Instant;
            let now = self.now();
            let caller = self.env().caller();
            let amount = self.env().transferred_balance();
            // Checking timestamp
            assert!(
                self.end_time == 0 || self.draw_tickets < MIN_ENTRIES || now < self.end_time,
                "Closed for new entants"
            );
            // Checking ammount
            assert!(
                amount >= MIN_PRICE && amount <= MAX_PRICE,
                "Wrong amount paid"
            );
            // Checking entry /
            assert!(
                self.entrants.contains_key(&caller) == false,
                "Must only enter once"
            );

            self.draw_tickets += 1;
            self.entrants.insert(caller, self.draw_tickets);
            self.entries.insert(self.draw_tickets, caller);

            if self.draw_tickets == MIN_ENTRIES {
                self.start_time = self.now();
                self.end_time = self.start_time + RUN_TIME;
            }
            self.funds += amount;
        }
        #[ink(message)]
        pub fn draw_your_luck(&mut self) {
            assert!(
                self.end_time > 0 && self.draw_tickets >= MIN_ENTRIES && self.now() >= self.end_time,
                "Not ready to draw yet"
            );

            assert!(
                self.draws < MAX_DRAWS,
                "Winners already decided"
            );
            
            let winner = self.do_random() % self.draw_tickets + 1;
            let winning_account = self.entries[&winner];
            self.winners[self.draws as usize] = Some(winning_account);
            self.draws += 1;
            if self.draws == MAX_DRAWS {
                let _ = self.env().transfer(self.charity_pot, self.funds);
            }
        }

        #[ink(message)]
        pub fn get_start(&self) -> u64 {
            self.start_time
        }
        #[ink(message)]
        pub fn get_end(&self) -> u64 {
            self.end_time
        }
        #[ink(message)]
        pub fn get_tickets_drawn(&self) -> u64 {
            self.draws
        }
        #[ink(message)]
        pub fn get_tickets_sold(&self) -> u64 {
            self.draw_tickets
        }
        #[ink(message)]
        pub fn show_the_winner(&self) -> [Option<AccountId>; MAX_DRAWS as usize] {
            self.winners
        }
        
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {

        // use super::*;
        
        // const DEFAULT_CALLEE_HASH: [u8; 32] = [0x07; 32];
        // const DEFAULT_GAS_LIMIT: Balance = 1000000;
        // const MINIMUM_PLAYERS: u32 = 5;
        // const MINIMUM_RAFFLE_DURATION_IN_MS: u64 = 900000;
        // const MAX_WINNERS: usize = 2;
        // const MINIMUM_BET: u128 = 10_000_000_000_000;
        // const MAXIMUM_BET: u128 = 100_000_000_000_000;
        // const MIN_ENTRIES: u64 = 5; // minimo numero
        // const MAX_DRAWS: u64 = 2; // maxi drawer / winners
        // const RUN_TIME: u64 = 900000; //60k * 15
        // const MIN_PRICE: u128 = 10000000000000; //10 (12)  // 0.01
        // const MAX_PRICE: u128 = 100000000000000;//100  (12) // 0.1

        // fn default_accounts(
        // ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
        //     ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        //         .expect("off-chain environment should have been initialized already")
        // }

        // fn set_next_caller(caller: AccountId, endowment: Option<Balance>) {
        //     ink_env::test::push_execution_context::<ink_env::DefaultEnvironment>(
        //         caller,
        //         AccountId::from(DEFAULT_CALLEE_HASH),
        //         DEFAULT_GAS_LIMIT,
        //         endowment.unwrap_or(MAX_PRICE),
        //         ink_env::test::CallData::new(ink_env::call::Selector::new([0x00; 4])),
        //     )
        // }
    }
}
