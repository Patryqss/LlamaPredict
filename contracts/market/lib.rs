#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::market::MarketRef;


use ink::primitives::{AccountId, Hash};
pub mod errors;
pub struct MarketState {}

#[ink::trait_definition]
pub trait Predicter {
    #[ink(message)]
    fn add_collateral(&self, collateral: AccountId, amount: u128);
}

#[ink::contract]
mod market {
    use ink::contract_ref;
    use primitive_types::{U128, U256};
    use crate::{errors::MarketError, Predicter};
    use conditional_psp22::ConditionalPSP22Ref;
    use traits::PSP22Extras;

    fn scale(a: u128, scaler: u16) -> u128 {
        let result = U128::from(a).full_mul(U128::from(scaler));
        (result >> u16::BITS).low_u128()
    }

    #[ink(storage)]
    pub struct Market {
        predicter: AccountId,
        collateral: AccountId,
        hash: Hash,
        token_a: ConditionalPSP22Ref,
        token_b: ConditionalPSP22Ref,
        collateral_rate: u16,
        total_minted: u128,
        total_collateral: u128,
    }

    impl Market {
        #[ink(constructor)]
        pub fn new(
            token_hash: Hash,
            router: AccountId,
            collateral: AccountId,
            hash: Hash,
            collateral_rate: u16,
        ) -> Self {
            let token_a = ConditionalPSP22Ref::new(router)
                .code_hash(token_hash)
                .endowment(0)
                .salt_bytes([0x00])
                .instantiate();
            let token_b = ConditionalPSP22Ref::new(router)
                .code_hash(token_hash)
                .endowment(0)
                .salt_bytes([0x01])
                .instantiate();
            let predicter = Self::env().caller();
            Self { 
                predicter,
                collateral,
                hash,
                token_a,
                token_b,
                collateral_rate,
                total_minted: 0,
                total_collateral: 0,
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, amount: u128) -> Result<(), MarketError>  {
            let collateral = self.collateral;
            let predicter = self.predicter;

            let predicter: contract_ref!(Predicter) = predicter.into();
            predicter.add_collateral(collateral, amount);

            let caller = self.env().caller();
            let collateral_rate = self.collateral_rate;
            let total_collateral = self.total_collateral;
            let total_minted = self.total_minted;

            let new_total_collateral = {
                let r = total_collateral.checked_add(amount);
                r.ok_or(MarketError::MintOverflow)?
            };
            let minted = amount - scale(amount, collateral_rate);
            let new_minted = total_minted + minted;

            self.total_collateral = new_total_collateral;
            self.total_minted = new_minted;

            self.token_a.mint_to(caller, minted).ok().ok_or(MarketError::MintPSP20Error)?;
            self.token_b.mint_to(caller, minted).ok().ok_or(MarketError::MintPSP20Error)?;

            Ok(())
        }


        #[ink(message)]
        pub fn account_id(&self) -> AccountId {
            self.env().account_id()
        }
    }
}
