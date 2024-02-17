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
    #[cfg(feature = "std")]
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
        underlying_token: AccountId,
        hash: Hash,
        token_a: ConditionalPSP22Ref,
        token_b: ConditionalPSP22Ref,
        collateral_rate: u16,
        expired_at: Timestamp,
        resolved_at: Timestamp,
        total_minted: u128,
        total_tokens: u128,
        abandoned_a: u128,
        abandoned_b: u128,
        outcome_a: u128,
        outcome_b: u128,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo), derive(StorageLayout))]
    pub struct UserMarketData {
        deposited: u128,
        claimed: u128,
    }

    #[ink(storage)]
    pub struct PredictorContract {
        admin: AccountId,
        token_hash: Hash,
        router: AccountId,
        markets: Mapping<u64, Market>,
        user_market_data: Mapping<(AccountId, u64), UserMarketData>,
        count: u64,
    }

    impl PredictorContract {
        #[ink(constructor)]
        pub fn new(
            token_hash: Hash,
            router: AccountId,
        ) -> Self {
            let admin = Self::env().caller();
            Self { 
                admin,
                token_hash,
                router,
                markets: Default::default(),
                user_market_data: Default::default(),
                count: 0,
            }
        }

        #[ink(message)]
        pub fn get_market(&self, market_id: u64) -> Option<Market> {
            self.markets.get(&market_id)
        }

        #[ink(message)]
        pub fn get_user_market_data(&self, account_id: AccountId, market_id: u64) -> Option<UserMarketData> {
            self.user_market_data.get((account_id, market_id))
        }

        #[ink(message)]
        pub fn add_market(&mut self, 
            underlying_token: AccountId,
            hash: Hash,
            resolved_at: Timestamp,
            resolution_time: u64,
            collateral_rate: u16,
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
            let expired_at = resolved_at.saturating_add(resolution_time);

            let market = Market {
                underlying_token,
                hash,
                token_a,
                token_b,
                collateral_rate,
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
            let caller = self.env().caller();
            let account_id = self.env().account_id();
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::MintForNotExistingMarket)
            }?;
            let mut underlying_token_ref: contract_ref!(PSP22) = market.underlying_token.into();
            let amount = {
                let r = amount.checked_shl(1);
                r.ok_or(PredictorError::MintOverflow)?
            };
            // Transfer from underlying token must take place before we get market by id
            // Otherwise, inside transfer_from, contract could be called once again.
            // By the end of function we would have different market state than we should.
            // As underlying_token is passed as parameter, it is super important
            // To validate it as soon as possible
            {
                let r = underlying_token_ref.transfer_from(caller, account_id, amount, vec![]);
                r.map_err(|e|PredictorError::MintTransferFromError(e))?;
            }

            let new_total_tokens = {
                let r = market.total_tokens.checked_add(amount);
                r.ok_or(PredictorError::MintOverflow)?
            };
            let collateral = scale(amount, market.collateral_rate);
            let minted = (amount - collateral) >> 1;
            let new_total_minted = market.total_minted + (minted << 1);

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            self.markets.insert(market_id, &market);

            // Update amounts deposited by user
            match self.user_market_data.get((caller, market_id)) {
                Some(mut user_market_data) => {
                    user_market_data.deposited += amount;
                    self.user_market_data.insert((caller, market_id), &user_market_data);
                },
                None => {
                    let user_market_data: UserMarketData = UserMarketData {
                        deposited: amount,
                        claimed: 0,
                    };
                    self.user_market_data.insert((caller, market_id), &user_market_data);
                }
            };
            
            

            // Token A and B are trusted
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

            // Token A and B are trusted
            {
                let r = market.token_a.burn_from(caller, amount);
                r.map_err(|e|PredictorError::BurnAError(e))?;
            }
            {
                let r = market.token_b.burn_from(caller, amount);
                r.map_err(|e|PredictorError::BurnBError(e))?;
            }

            let mut underlying_token: contract_ref!(PSP22) = market.underlying_token.into();

            let to_withdraw = if market.total_minted == 0 {
                0
            } else {
                ratio(amount2, market.total_tokens, market.total_minted)
            };
            let new_total_tokens = market.total_tokens - to_withdraw;

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            markets.insert(market_id, &market);

            // TODO: Add user data claimed

            // As transfer is last operation, reentracy does not introduce any risk
            {
                let r = underlying_token.transfer(caller, to_withdraw, vec![]);
                r.map_err(|e|PredictorError::BurnTransferError(e))?;
            }

            Ok(())
        }
        
        #[ink(message)]
        pub fn give_up(&mut self, market_id: u64, amount: u128, is_a: bool) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::GiveUpForNotExistingMarket)
            }?;
            let mut underlying_token: contract_ref!(PSP22) = market.underlying_token.into();

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
            self.markets.insert(market_id, &market);


            // TODO: Add user data claimed

            // As transfer is last operation, reentracy does not introduce any risk
            {
                let r = underlying_token.transfer(caller, to_withdraw, vec![]);
                r.map_err(|e|PredictorError::GiveUpTransferError(e))?;
            }

            Ok(())
        }

        #[ink(message)]
        pub fn use_abandoned(&mut self, market_id: u64, is_a: bool) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::UseAbandonedForNotExistingMarket)
            }?;
            let mut underlying_token: contract_ref!(PSP22) = market.underlying_token.into();

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
            self.markets.insert(market_id, &market);


            // TODO: Add user data claimed

            // As transfer is last operation, reentracy does not introduce any risk
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
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::SetOutcomeForNotExistingMarket)
            }?;
            if market.outcome_a != 0 || market.outcome_b != 0 {
                return Err(PredictorError::SetOutcomeTwice);
            }

            market.outcome_a = outcome_a;
            market.outcome_b = outcome_b;
            self.markets.insert(market_id, &market);

            Ok(())
        }

        #[ink(message)]
        pub fn burn_by_outcome(&mut self, market_id: u64, amount: u128) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::BurnByOutcomeForNotExistingMarket)
            }?;
            let now = self.env().block_timestamp();
            // Admin can set outcome anytime, so we cannot allow to burn by outcome before market exipration
            if now < market.expired_at {
                return Err(PredictorError::BurnByOutcomeTooEarly);
            }
            if market.outcome_a == 0 && market.outcome_b == 0 {
                return Err(PredictorError::BurnByOutcomeNoOutcome);
            }
            let to_burn = if market.outcome_a == 0 {
                let r = market.token_b.burn_from(caller, amount);
                r.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;
                amount
            } else if market.outcome_b == 0 {
                let r = market.token_a.burn_from(caller, amount);
                r.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;
                amount
            } else if market.outcome_a > market.outcome_b {
                let amount_b = ratio(amount, market.outcome_b, market.outcome_a);

                let r_b = market.token_b.burn_from(caller, amount_b);
                r_b.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                let amount_a = ratio(amount_b, market.outcome_a, market.outcome_b);

                let r_a = market.token_a.burn_from(caller, amount_a);
                r_a.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                amount_a + amount_b
            } else {
                let amount_a = ratio(amount, market.outcome_a, market.outcome_b);

                let r_a = market.token_a.burn_from(caller, amount_a);
                r_a.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                let amount_b = ratio(amount_a, market.outcome_b, market.outcome_a);

                let r_b = market.token_b.burn_from(caller, amount_b);
                r_b.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                amount_a + amount_b
            };

            let new_total_minted = market.total_minted - to_burn;
            let to_withdraw = if market.total_minted == 0 {
                0
            } else {
                ratio(to_burn, market.total_tokens, market.total_minted)
            };
            let new_total_tokens = market.total_tokens - to_withdraw;

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            self.markets.insert(market_id, &market);

            // TODO: Add user data claimed

            let mut underlying_token: contract_ref!(PSP22) = market.underlying_token.into();
            {
                let r = underlying_token.transfer(caller, to_withdraw, vec![]);
                r.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;
            }
            Ok(())
        }
    }
}
