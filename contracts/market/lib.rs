#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::{AccountId, Hash};
pub use self::market::MarketRef;

pub struct MarketState {}

#[ink::trait_definition]
pub trait Market {
    #[ink(message)]
    fn state(&self) -> (AccountId, Hash);
}

#[ink::contract]
mod market {
    #[ink(storage)]
    pub struct Market {
        collateral: AccountId,
        hash: Hash,
        tokan_a: AccountId,
        token_b: AccountId,
    }

    impl Market {
        #[ink(constructor)]
        pub fn new(
            collateral: AccountId,
            hash: Hash,
        ) -> Self {
            Self { 
                collateral,
                hash,
                tokan_a: collateral, //FIXME
                token_b: collateral, //FIXME
            }
        }

        #[ink(message)]
        pub fn account_id(&self) -> AccountId {
            self.env().account_id()
        }
    }
}
