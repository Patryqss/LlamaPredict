#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;

#[ink::contract]
mod predictor {
    use conditional_psp22::ConditionalPSP22Ref;
    use ink::contract_ref;
    use ink::storage::Mapping;
    use primitive_types::{U128, U256};
    use psp22::PSP22;
    use ink::prelude::vec;
    use crate::errors::PredictorError;
    use ink::storage::traits::StorageLayout;

    fn scale(a: u128, scaler: u16) -> u128 {
        let result = U128::from(a).full_mul(U128::from(scaler));
        (result >> u16::BITS).low_u128()
    }

    fn ratio(a: u128, b: u128, c: u128) -> u128 {
        let denominator = U128::from(a).full_mul(U128::from(b));
        let result = denominator / U256::from(c);
        result.low_u128()
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo), derive(StorageLayout))]
    pub struct Market {
        hash: Hash,
        token_a: ConditionalPSP22Ref,
        token_b: ConditionalPSP22Ref,
        expired_at: Timestamp,
        resolved_at: Timestamp,
        total_minted: u128,
        total_tokens: u128,
        abandoned_a: u128,
        abandoned_b: u128,
        outcome_a: u128,
        outcome_b: u128,
    }

    #[ink(storage)]
    pub struct PredictorContract {
        admin: AccountId,
        token_hash: Hash,
        router: AccountId,
        underlying_token: AccountId,
        markets: Mapping<u64, Market>,
        count: u64,
        collateral_rate: u16,
    }

    impl PredictorContract {
        #[ink(constructor)]
        pub fn new(
            token_hash: Hash,
            router: AccountId,
            underlying_token: AccountId,
        ) -> Self {
            let admin = Self::env().caller();
            Self { 
                admin,
                token_hash,
                router,
                underlying_token,
                markets: Default::default(),
                count: 0,
                collateral_rate: 0,
            }
        }


        #[ink(message)]
        pub fn add_market(&mut self, 
            hash: Hash,
            expired_at: Timestamp,
            resolution_time: u64,
        ) -> Result<(), PredictorError>{
            let caller = self.env().caller();
            let admin = self.admin;
            if caller != admin {
                return Err(PredictorError::CallerIsNotAdmin);
            }

            let market_id = self.count;
            let router = self.router;
            let token_hash = self.token_hash;
            let new_count = market_id + 1;
            let token_a_salt = market_id * 2;
            let token_b_salt = token_a_salt + 1;
            let token_a = ConditionalPSP22Ref::new(router)
                .code_hash(token_hash)
                .endowment(0)
                .salt_bytes(token_a_salt.to_be_bytes())
                .instantiate();
            let token_b = ConditionalPSP22Ref::new(router)
                .code_hash(token_hash)
                .endowment(0)
                .salt_bytes(token_b_salt.to_be_bytes())
                .instantiate();
            let resolved_at = expired_at.saturating_sub(resolution_time);

            let market = Market {
                hash,
                token_a,
                token_b,
                expired_at,
                resolved_at,
                total_minted: 0,
                total_tokens: 0,
                abandoned_a: 0,
                abandoned_b: 0,
                outcome_a: 0,
                outcome_b: 0,
            };

            self.markets.insert(market_id, &market);
            self.count = new_count;

            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, market_id: u64, amount: u128) -> Result<(), PredictorError>  {
            let underlying_token = self.underlying_token;
            let caller = self.env().caller();
            let account_id = self.env().account_id();
            let mut underlying_token: contract_ref!(PSP22) = underlying_token.into();
            let amount = {
                let r = amount.checked_shl(1);
                r.ok_or(PredictorError::MintOverflow)?
            };
            {
                let r = underlying_token.transfer_from(caller, account_id, amount, vec![]);
                r.map_err(|e|PredictorError::MintTransferFromError(e))?;
            }

            let markets = &self.markets;
            let mut market = {
                let r = markets.get(&market_id);
                r.ok_or(PredictorError::MintForNotExistingMarket)
            }?;


            let collateral_rate = self.collateral_rate;

            let new_total_tokens = {
                let r = market.total_tokens.checked_add(amount);
                r.ok_or(PredictorError::MintOverflow)?
            };
            let collateral = scale(amount, collateral_rate);
            let minted = (amount - collateral) >> 1;
            let new_total_minted = market.total_minted + (minted << 1);

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            self.markets.insert(market_id, &market);

            {
                let r = market.token_a.mint_to(caller, minted);
                r.map_err(|e|PredictorError::MintAError(e))?;
            }
            {
                let r = market.token_b.mint_to(caller, minted);
                r.map_err(|e|PredictorError::MintBError(e))?;
            }

            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, market_id: u64, amount: u128) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let markets = &mut self.markets;
            let mut market = {
                let r = markets.get(&market_id);
                r.ok_or(PredictorError::BurnForNotExistingMarket)
            }?;
            let amount2 = {
                let r = amount.checked_shl(1);
                r.ok_or(PredictorError::MintOverflow)?
            };

            let new_total_minted = {
                let r = market.total_minted.checked_sub(amount2);
                r.ok_or(PredictorError::BurnOverflow)?
            };

            {
                let r = market.token_a.burn_from(caller, amount);
                r.map_err(|e|PredictorError::BurnAError(e))?;
            }
            {
                let r = market.token_b.burn_from(caller, amount);
                r.map_err(|e|PredictorError::BurnBError(e))?;
            }

            let mut underlying_token: contract_ref!(PSP22) = self.underlying_token.into();

            let to_withdraw = if market.total_minted == 0 {
                0
            } else {
                ratio(amount2, market.total_tokens, market.total_minted)
            };
            let new_total_tokens = market.total_tokens - to_withdraw;

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            markets.insert(market_id, &market);

            {
                let r = underlying_token.transfer(caller, to_withdraw, vec![]);
                r.map_err(|e|PredictorError::BurnTransferError(e))?;
            }

            Ok(())
        }
        
        #[ink(message)]
        pub fn give_up(&mut self, market_id: u64, amount: u128, is_a: bool) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let mut underlying_token: contract_ref!(PSP22) = self.underlying_token.into();
            let markets = &mut self.markets;
            let mut market = {
                let r = markets.get(&market_id);
                r.ok_or(PredictorError::GiveUpForNotExistingMarket)
            }?;

            let new_total_minted = {
                let r = market.total_minted.checked_sub(amount);
                r.ok_or(PredictorError::GiveUpOverflow)?
            };
            let new_abandoned = {
                let r = if is_a {
                    market.abandoned_a.checked_add(amount)
                } else {
                    market.abandoned_b.checked_add(amount)
                };
                r.ok_or(PredictorError::GiveUpOverflow)?
            };
            {
                let r = if is_a {
                    market.token_a.burn_from(caller, amount)
                } else {
                    market.token_b.burn_from(caller, amount)
                };
                r.map_err(|e|PredictorError::GiveUpTokenError(e))?;
            };

            let collateral = market.total_tokens - market.total_minted;
            
            let to_withdraw = if market.total_minted == 0 {
                0
            } else {
                ratio(amount, collateral, market.total_minted)
            };
            let new_total_tokens = market.total_tokens - to_withdraw;
            
            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            if is_a {
                market.abandoned_a = new_abandoned;
            } else {
                market.abandoned_b = new_abandoned;
            }
            markets.insert(market_id, &market);

            {
                let r = underlying_token.transfer(caller, to_withdraw, vec![]);
                r.map_err(|e|PredictorError::GiveUpTransferError(e))?;
            }

            Ok(())
        }

        #[ink(message)]
        pub fn use_abandoned(&mut self, market_id: u64, is_a: bool) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let mut underlying_token: contract_ref!(PSP22) = self.underlying_token.into();
            let markets = &mut self.markets;
            let mut market = {
                let r = markets.get(&market_id);
                r.ok_or(PredictorError::UseAbandonedForNotExistingMarket)
            }?;

            let abandoned = if is_a {
                market.abandoned_a
            } else {
                market.abandoned_b
            };
            let abandoned_balance = if is_a {
                market.token_a.balance_of(caller)
            } else {
                market.token_b.balance_of(caller)
            };
            let amount = abandoned.min(abandoned_balance);
            let new_abandoned = abandoned - amount;
            {
                let r = if is_a {
                    market.token_a.burn_from(caller, amount)
                } else {
                    market.token_b.burn_from(caller, amount)
                };
                r.map_err(|e|PredictorError::UseAbandonedTokenError(e))?;
            };
            let to_withdraw = if market.total_minted == 0 {
                0
            } else {
                ratio(amount, market.total_tokens, market.total_minted)
            };
            let new_total_minted = market.total_minted - amount;
            let new_total_tokens = market.total_tokens - to_withdraw;

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            if is_a {
                market.abandoned_a = new_abandoned;
            } else {
                market.abandoned_b = new_abandoned;
            }
            markets.insert(market_id, &market);

            {
                let r = underlying_token.transfer(caller, amount, vec![]);
                r.map_err(|e|PredictorError::UseAbandonedTransferError(e))?;
            }
            Ok(())   
        }

        #[ink(message)]
        pub fn set_outcome(&mut self, market_id: u64, outcome_a: u128, outcome_b: u128) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let admin = self.admin;
            if caller != admin {
                return Err(PredictorError::CallerIsNotAdmin);
            }
            let markets = &mut self.markets;
            let mut market = {
                let r = markets.get(&market_id);
                r.ok_or(PredictorError::SetOutcomeForNotExistingMarket)
            }?;
            if market.outcome_a != 0 || market.outcome_b != 0 {
                return Err(PredictorError::SetOutcomeTwice);
            }

            market.outcome_a = outcome_a;
            market.outcome_b = outcome_b;
            markets.insert(market_id, &market);

            Ok(())
        }

        #[ink(message)]
        pub fn burn_by_outcome(&mut self, market_id: u64) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let markets = &mut self.markets;
            let mut market = {
                let r = markets.get(&market_id);
                r.ok_or(PredictorError::BurnByOutcomeForNotExistingMarket)
            }?;
            let now = self.env().block_timestamp();
            if now < market.resolved_at {
                return Err(PredictorError::BurnByOutcomeTooEarly);
            }
            if market.outcome_a == 0 && market.outcome_b == 0 {
                return Err(PredictorError::BurnByOutcomeNoOutcome);
            }

            Ok(())
        }
    }
}
