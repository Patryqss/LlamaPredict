#![cfg_attr(not(feature = "std"), no_std, no_main)]


#[ink::contract]
mod predictor {
    use ink::contract_ref;
    use ink::storage::Mapping;
    use market::MarketRef;
    use traits::errors::PredictorError;
    use traits::Predictor;
    use psp22::PSP22;
    use ink::prelude::vec;

    #[ink(storage)]
    pub struct PredictorContract {
        admin: AccountId,
        market_hash: Hash,
        token_hash: Hash,
        router: AccountId,
        markets: Mapping<u64, MarketRef>,
        markets_ids: Mapping<AccountId, u64>,
        count: u64,
        collateral_rate: u16,
    }

    impl PredictorContract {
        #[ink(constructor)]
        pub fn new(
            market_hash: Hash,
            token_hash: Hash,
            router: AccountId,
        ) -> Self {
            let admin = Self::env().caller();
            Self { 
                admin,
                market_hash,
                token_hash,
                router,
                markets: Default::default(),
                markets_ids: Default::default(),
                count: 0,
                collateral_rate: 0,
            }
        }


        #[ink(message)]
        pub fn add_market(&mut self, 
            collateral: AccountId, 
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
            let market_hash = self.market_hash;
            let router = self.router;
            let token_hash = self.token_hash;
            let collateral_rate = self.collateral_rate;
            let market = MarketRef::new(
                token_hash, 
                router, 
                collateral, 
                hash, 
                collateral_rate,
                expired_at,
                resolution_time,
            ).code_hash(market_hash)
            .endowment(0)
            .salt_bytes(market_id.to_be_bytes())
            .instantiate();
            let market_address = market.account_id();
            let new_count = market_id + 1;

            self.markets.insert(market_id, &market);
            self.markets_ids.insert(market_address, &market_id);
            self.count = new_count;

            Ok(())
        }
    }

    impl Predictor for PredictorContract {
        #[ink(message)]
        fn add_tokens(&self, underlying_token: AccountId, from: AccountId, amount: u128) -> Result<(), PredictorError> {
            let caller = self.env().caller();
            if let None = self.markets_ids.get(&caller) {
                return Err(PredictorError::CallerIsNotMarket);
            }
            let mut underlying_token: contract_ref!(PSP22) = underlying_token.into();
            {
                let r = underlying_token.transfer_from(from, caller, amount, vec![]);
                r.ok().ok_or(PredictorError::AddTokensPSP22Error)?; 

            }

            Ok(())
        }
    }
}
