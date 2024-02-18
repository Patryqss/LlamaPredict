#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;

#[ink::contract]
mod predictor {
    use conditional_psp22::ConditionalPSP22Ref;
    use ink::contract_ref;
    use ink::storage::Mapping;
    use primitive_types::{U128, U256};
    use psp22::PSP22;
    use psp22::PSP22Error;
    use ink::prelude::vec;
    use crate::errors::PredictorError;
    #[cfg(feature = "std")]
    use ink::storage::traits::StorageLayout;

    use amm_traits::Router;

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
        
        #[cfg(not(test))]
        token_a: ConditionalPSP22Ref,
        #[cfg(test)]
        token_a: u64,
        #[cfg(not(test))]
        token_b: ConditionalPSP22Ref,
        #[cfg(test)]
        token_b: u64,

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
    pub struct MarketResponse {
        market: Market,
        balance_a: u128,
        balance_b: u128,
    }

    #[derive(Default, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo), derive(StorageLayout))]
    pub struct UserMarketData {
        deposited: u128,
        claimed: u128,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo), derive(StorageLayout))]
    pub struct UserResponse {
        user: UserMarketData,
        balance_a: u128,
        balance_b: u128,
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

    #[cfg(test)]
    pub static mut UNDERLYING_BALANCES: Option<std::collections::HashMap<(AccountId, AccountId), u128>> = None;
    #[cfg(test)]
    pub static mut MARKET_BALANCES: Option<std::collections::HashMap<(u64, AccountId), u128>> = None;

    #[cfg(test)]
    fn market_token_id(_token: &u64) -> AccountId {
        AccountId::from([0x0; 32])
    }
    #[cfg(not(test))]
    fn market_token_id(token: &ConditionalPSP22Ref) -> AccountId {
        *token.as_ref()
    }

    impl PredictorContract {
        #[cfg(test)]
        fn instantiate_token(&self, _router: AccountId, _token_hash: Hash, salt: u64) -> u64 {
            salt
        }
        #[cfg(not(test))]
        fn instantiate_token(&self, router: AccountId, token_hash: Hash, salt: u64) -> ConditionalPSP22Ref {
            ConditionalPSP22Ref::new(router)
                .code_hash(token_hash)
                .endowment(0)
                .salt_bytes(salt.to_be_bytes())
                .instantiate()
        }
        #[cfg(test)]
        fn market_token_mint_to(&self, token: &u64, caller: AccountId, amount: u128) -> Result<(), PSP22Error>{
            unsafe {
                let balances = MARKET_BALANCES.as_mut().unwrap();
                let key = (*token, caller);
                let balance = balances.get(&key).unwrap_or(&0);
                let new_balance = balance.checked_add(amount).ok_or(PSP22Error::InsufficientBalance)?;
                balances.insert(key, new_balance);
            }
            Ok(())
        }
        #[cfg(not(test))]
        fn market_token_mint_to(
            &self, 
            token: &mut ConditionalPSP22Ref, 
            caller: AccountId, 
            amount: u128
        ) -> Result<(), PSP22Error> {
            token.mint_to(caller, amount)
        }
        #[cfg(test)]
        fn market_token_burn_from(&self, token: &u64, caller: AccountId, amount: u128) -> Result<(), PSP22Error>{
            unsafe {
                let balances = MARKET_BALANCES.as_mut().unwrap();
                let key = (*token, caller);
                let balance = balances.get(&key).unwrap_or(&0);
                let new_balance = balance.checked_sub(amount).ok_or(PSP22Error::InsufficientBalance)?;
                balances.insert(key, new_balance);
            }
            Ok(())
        }
        #[cfg(not(test))]
        fn market_token_burn_from(
            &self, 
            token: &mut ConditionalPSP22Ref, 
            caller: AccountId, 
            amount: u128
        ) -> Result<(), PSP22Error> {
            token.burn_from(caller, amount)
        }
        #[cfg(test)]
        fn market_token_balance_of(&self, _token: &u64, _user: AccountId) -> u128 {
            0
        }
        #[cfg(not(test))]
        fn market_token_balance_of(
            &self, 
            token: &ConditionalPSP22Ref, 
            user: AccountId, 
        ) -> u128 {
            token.balance_of(user)
        }
        #[cfg(test)]
        fn underlying_transfer_from(&self, underlying_token: AccountId, caller: AccountId, amount: u128) -> Result<(), PSP22Error>{
            unsafe {
                let balances = UNDERLYING_BALANCES.as_mut().unwrap();
                let key = (underlying_token, caller);
                let balance = balances.get(&key).unwrap_or(&0);
                let new_balance = balance.checked_sub(amount).ok_or(PSP22Error::InsufficientBalance)?;
                balances.insert(key, new_balance);
            }
            Ok(())
        }
        #[cfg(not(test))]
        fn underlying_transfer_from(
            &self, 
            underlying_token: AccountId, 
            caller: AccountId, 
            amount: u128
        ) -> Result<(), PSP22Error> {
            let account_id = self.env().account_id();
            let mut underlying_token_ref: contract_ref!(PSP22) = underlying_token.into();
            underlying_token_ref.transfer_from(caller, account_id, amount, vec![])
        }
        #[cfg(test)]
        fn underlying_transfer(&self, _underlying_token: AccountId, _caller: AccountId, _amount: u128) -> Result<(), PSP22Error>{
            Ok(())
        }
        #[cfg(not(test))]
        fn underlying_transfer(
            &self, 
            underlying_token: AccountId, 
            caller: AccountId, 
            amount: u128
        ) -> Result<(), PSP22Error> {
            let mut underlying_token_ref: contract_ref!(PSP22) = underlying_token.into();
            underlying_token_ref.transfer(caller, amount, vec![])
        }

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
        pub fn get_market(&self, market_id: u64) -> Option<MarketResponse> {
            let market = match self.markets.get(&market_id) {
                Some(market) => market,
                None => return None,
            };
            let router: contract_ref!(Router) = self.router.into();
            let token_a = market_token_id(&market.token_a);
            let token_b = market_token_id(&market.token_b);
            let (balance_a, balance_b) = match router.get_reserves(token_a, token_b) {
                Ok((balance_a, balance_b)) => (balance_a, balance_b),
                Err(_) => (0, 0),
            };
            Some(MarketResponse{
                market,
                balance_a,
                balance_b,
            })
        }

        #[ink(message)]
        pub fn get_user_market_data(&self, account_id: AccountId, market_id: u64) -> Option<UserResponse> {
            let user_market_data = match self.user_market_data.get((account_id, market_id)) {
                Some(user_market_data) => user_market_data,
                None => return None,
            };
            let market = match self.markets.get(market_id) {
                Some(market) => market,
                None => return None,
            };
            let token_a: contract_ref!(PSP22) = market_token_id(&market.token_a).into();
            let token_b: contract_ref!(PSP22) = market_token_id(&market.token_b).into();
            
            let balance_a = token_a.balance_of(account_id);
            let balance_b = token_b.balance_of(account_id);
            Some(UserResponse{
                user: user_market_data,
                balance_a,
                balance_b,
            })
        }

        #[ink(message)]
        pub fn set_router(&mut self, router: AccountId) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let admin = self.admin;
            if caller != admin {
                return Err(PredictorError::CallerIsNotAdmin);
            }
            self.router = router;
            Ok(())
        }

        #[ink(message)]
        pub fn add_market(&mut self, 
            underlying_token: AccountId,
            hash: Hash,
            resolved_at: Timestamp,
            resolution_time: u64,
            collateral_rate: u16,
        ) -> Result<u64, PredictorError>{
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
            let token_a = self.instantiate_token(router, token_hash, token_a_salt);
            let token_b = self.instantiate_token(router, token_hash, token_b_salt);
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

            Ok(market_id)
        }

        fn increase_user_deposited(
            &mut self,
            caller: AccountId,
            market_id: u64,
            amount: u128,
        ) {
            let umd = self.user_market_data.get((caller, market_id)).unwrap_or_default();
            
            let user_market_data = UserMarketData {
                deposited: umd.deposited.saturating_add(amount),
                claimed: umd.claimed,
            };

            self.user_market_data.insert((caller, market_id), &user_market_data);
        }

        fn increase_user_claimed(
            &mut self,
            caller: AccountId,
            market_id: u64,
            amount: u128,
        ) {
            let umd = self.user_market_data.get((caller, market_id)).unwrap_or_default();
            
            let user_market_data = UserMarketData {
                deposited: umd.deposited,
                claimed: umd.claimed.saturating_add(amount),
            };

            self.user_market_data.insert((caller, market_id), &user_market_data);
        }

        #[ink(message)]
        pub fn mint(&mut self, market_id: u64, amount: u128) -> Result<(), PredictorError>  {
            let caller = self.env().caller();
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::MintForNotExistingMarket)
            }?;
            // TODO: underlying token should be passed as argument
            // We cannot get market before transfer as it would introduce reentrancy risk
            {
                let r = self.underlying_transfer_from(market.underlying_token, caller, amount);
                r.map_err(|e|PredictorError::MintTransferFromError(e))?;
            }

            let new_total_tokens = {
                let r = market.total_tokens.checked_add(amount);
                r.ok_or(PredictorError::MintOverflow)?
            };
            let collateral = scale(amount, market.collateral_rate);
            let minted = amount - collateral;
            let new_total_minted = market.total_minted + minted;
            let minted_per_token = minted >> 1;

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            self.markets.insert(market_id, &market);

            self.increase_user_deposited(caller, market_id, amount);

            // Token A and B are trusted
            {
                let r = self.market_token_mint_to(&mut market.token_a, caller, minted_per_token);
                r.map_err(|e|PredictorError::MintAError(e))?;
            }
            {
                let r = self.market_token_mint_to(&mut market.token_b, caller, minted_per_token);
                r.map_err(|e|PredictorError::MintBError(e))?;
            }

            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self, market_id: u64, amount: u128) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            let mut market = {
                let r = self.markets.get(&market_id);
                r.ok_or(PredictorError::BurnForNotExistingMarket)
            }?;

            // Token A and B are trusted
            {
                let r = self.market_token_burn_from(&mut market.token_a, caller, amount);
                r.map_err(|e|PredictorError::BurnAError(e))?;
            }
            {
                let r = self.market_token_burn_from(&mut market.token_b, caller, amount);
                r.map_err(|e|PredictorError::BurnBError(e))?;
            }

            let burned = amount << 1;
            let new_total_minted = {
                let r = market.total_minted.checked_sub(burned);
                r.ok_or(PredictorError::BurnOverflow)?
            };


            let to_withdraw = if market.total_minted == 0 {
                0
            } else {
                ratio(burned, market.total_tokens, market.total_minted)
            };
            let new_total_tokens = market.total_tokens - to_withdraw;
            let user_market_key = (caller, market_id);
            let mut user_market_data = self.user_market_data.get(user_market_key).unwrap_or_default();
            let new_user_claimed = user_market_data.claimed.saturating_add(to_withdraw);

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            self.markets.insert(market_id, &market);

            user_market_data.claimed = new_user_claimed;
            self.user_market_data.insert(user_market_key, &user_market_data);

            // As transfer is last operation, reentracy does not introduce any risk
            {
                let r = self.underlying_transfer(market.underlying_token, caller, to_withdraw);
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
                    self.market_token_burn_from(&mut market.token_a, caller, amount)
                } else {
                    self.market_token_burn_from(&mut market.token_b, caller, amount)
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
            let user_market_key = (caller, market_id);
            let mut user_market_data = self.user_market_data.get(user_market_key).unwrap_or_default();
            let new_user_claimed = user_market_data.claimed.saturating_add(to_withdraw);
            
            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            if is_a {
                market.abandoned_a = new_abandoned;
            } else {
                market.abandoned_b = new_abandoned;
            }
            self.markets.insert(market_id, &market);

            user_market_data.claimed = new_user_claimed;
            self.user_market_data.insert(user_market_key, &user_market_data);

            // As transfer is last operation, reentracy does not introduce any risk
            {
                let r = self.underlying_transfer(market.underlying_token, caller, to_withdraw);
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

            let abandoned = if is_a {
                market.abandoned_a
            } else {
                market.abandoned_b
            };
            let abandoned_balance = if is_a {
                self.market_token_balance_of(&mut market.token_a, caller)
            } else {
                self.market_token_balance_of(&mut market.token_b, caller)
            };
            let amount = abandoned.min(abandoned_balance);
            let new_abandoned = abandoned - amount;
            {
                let r = if is_a {
                    self.market_token_burn_from(&mut market.token_a, caller, amount)
                } else {
                    self.market_token_burn_from(&mut market.token_b, caller, amount)
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
            let user_market_key = (caller, market_id);
            let mut user_market_data = self.user_market_data.get(user_market_key).unwrap_or_default();
            let new_user_claimed = user_market_data.claimed.saturating_add(to_withdraw);

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            if is_a {
                market.abandoned_a = new_abandoned;
            } else {
                market.abandoned_b = new_abandoned;
            }
            self.markets.insert(market_id, &market);

            user_market_data.claimed = new_user_claimed;
            self.user_market_data.insert(user_market_key, &user_market_data);

            // As transfer is last operation, reentracy does not introduce any risk
            {
                let r = self.underlying_transfer(market.underlying_token, caller, to_withdraw);
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
                let r = self.market_token_burn_from(&mut market.token_b, caller, amount);
                r.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;
                amount
            } else if market.outcome_b == 0 {
                let r = self.market_token_burn_from(&mut market.token_a, caller, amount);
                r.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;
                amount
            } else if market.outcome_a > market.outcome_b {
                let amount_b = ratio(amount, market.outcome_b, market.outcome_a);

                let r_b = self.market_token_burn_from(&mut market.token_b, caller, amount_b);
                r_b.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                let amount_a = ratio(amount_b, market.outcome_a, market.outcome_b);

                let r_a = self.market_token_burn_from(&mut market.token_a, caller, amount_a);
                r_a.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                amount_a + amount_b
            } else {
                let amount_a = ratio(amount, market.outcome_a, market.outcome_b);

                let r_a = self.market_token_burn_from(&mut market.token_a, caller, amount_a);
                r_a.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;

                let amount_b = ratio(amount_a, market.outcome_b, market.outcome_a);

                let r_b = self.market_token_burn_from(&mut market.token_b, caller, amount_b);
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
            let user_market_key = (caller, market_id);
            let mut user_market_data = self.user_market_data.get(user_market_key).unwrap_or_default();
            let new_user_claimed = user_market_data.claimed.saturating_add(to_withdraw);

            market.total_minted = new_total_minted;
            market.total_tokens = new_total_tokens;
            self.markets.insert(market_id, &market);

            user_market_data.claimed = new_user_claimed;
            self.user_market_data.insert(user_market_key, &user_market_data);

            {
                let r = self.underlying_transfer(market.underlying_token, caller, to_withdraw);
                r.map_err(|e|PredictorError::BurnByOutcomeBurnError(e))?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;