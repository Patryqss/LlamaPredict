#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::market::MarketRef;
pub mod errors;


#[ink::contract]
mod market {
    use ink::contract_ref;
    use ink::prelude::vec;
    use primitive_types::{U128, U256};
    use crate::errors::MarketError;
    use conditional_psp22::ConditionalPSP22Ref;
    use traits::{PSP22Extras, Predictor};
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
        predictor: AccountId,
        underlying_token: AccountId,
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
            underlying_token: AccountId,
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
            let predictor = Self::env().caller();
            let resolved_at = expired_at.saturating_sub(resolution_time);
            Self { 
                predictor,
                underlying_token,
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
            let underlying_token = self.underlying_token;
            let caller = self.env().caller();
            let predictor = self.predictor;

            let predictor: contract_ref!(Predictor) = predictor.into();
            {
                let r = predictor.add_tokens(underlying_token, caller, amount);
                r.map_err(|e| MarketError::MintPredictorError(e))?;
            }

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

            let mut underlying_token: contract_ref!(PSP22) = self.underlying_token.into();
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

            {
                let r = underlying_token.transfer(caller, to_withdraw, vec![]);
                r.ok().ok_or(MarketError::BurnPSP22Error)?;
            }

            Ok(())

        }

        #[ink(message)]
        pub fn account_id(&self) -> AccountId {
            self.env().account_id()
        }
    }
}
