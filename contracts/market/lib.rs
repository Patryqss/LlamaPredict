#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::market::MarketRef;


use ink::primitives::AccountId;
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
    use ink::prelude::vec;
    use primitive_types::{U128, U256};
    use crate::{errors::MarketError, Predicter};
    use conditional_psp22::ConditionalPSP22Ref;
    use traits::PSP22Extras;
    use psp22::PSP22;

    fn scale(a: u128, scaler: u16) -> u128 {
        let result = U128::from(a).full_mul(U128::from(scaler));
        (result >> u16::BITS).low_u128()
    }

    fn ratio(a: u128, b: u128, c: u128) -> u128 {
        let denominator = U128::from(a).full_mul(U128::from(b));
        let result = denominator / U256::from(c);
        result.low_u128()
    }

    #[ink(storage)]
    pub struct Market {
        predicter: AccountId,
        collateral: AccountId,
        hash: Hash,
        token_a: ConditionalPSP22Ref,
        token_b: ConditionalPSP22Ref,
        collateral_rate: u16,
        expired_at: Timestamp,
        resolved_at: Timestamp,
        total_minted: u128,
        total_tokens: u128,
    }

    impl Market {
        #[ink(constructor)]
        pub fn new(
            token_hash: Hash,
            router: AccountId,
            collateral: AccountId,
            hash: Hash,
            collateral_rate: u16,
            expired_at: Timestamp,
            resolution_time: u64,
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
            let resolved_at = expired_at.saturating_sub(resolution_time);
            Self { 
                predicter,
                collateral,
                hash,
                token_a,
                token_b,
                collateral_rate,
                //no need to validate it
                expired_at,
                resolved_at,
                total_minted: 0,
                total_tokens: 0,
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
            let total_minted = self.total_minted;
            let total_tokens = self.total_tokens;

            let new_total_tokens = {
                let r = total_tokens.checked_add(amount);
                r.ok_or(MarketError::MintOverflow)?
            };
            let collateral = scale(amount, collateral_rate);
            let minted = amount - collateral;
            let new_total_minted = total_minted + minted;

            self.total_minted = new_total_minted;
            self.total_tokens = new_total_tokens;

            self.token_a.mint_to(caller, minted).ok().ok_or(MarketError::MintPSP22Error)?;
            self.token_b.mint_to(caller, minted).ok().ok_or(MarketError::MintPSP22Error)?;

            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, amount: u128) -> Result<(), MarketError> {
            let caller = self.env().caller();
            let token_a = &mut self.token_a;
            let token_b = &mut self.token_b;

            token_a.burn_from(caller, amount).ok().ok_or(MarketError::BurnPSP22Error)?;
            token_b.burn_from(caller, amount).ok().ok_or(MarketError::BurnPSP22Error)?;

            let total_tokens = self.total_tokens;
            let total_minted = self.total_minted;

            let new_total_minted = {
                let r = total_minted.checked_sub(amount);
                r.ok_or(MarketError::BurnOverflow)?
            };

            let to_withdraw = if total_minted == 0 {
                0
            } else {
                ratio(amount, total_tokens, total_minted)
            };
            let new_total_tokens = total_tokens - amount;

            self.total_minted = new_total_minted;
            self.total_tokens = new_total_tokens;

            let mut collateral_token: contract_ref!(PSP22) = self.collateral.into();
            collateral_token.transfer(caller, to_withdraw, vec![]).ok().ok_or(MarketError::BurnPSP22Error)?;

            Ok(())

        }


        #[ink(message)]
        pub fn account_id(&self) -> AccountId {
            self.env().account_id()
        }
    }
}
