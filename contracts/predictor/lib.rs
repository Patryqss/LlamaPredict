#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;

#[ink::contract]
mod predictor {
    use crate::errors::PredictorError;
    use ink::storage::Mapping;
    use market::MarketRef;

    #[ink(storage)]
    pub struct Predictor {
        admin: AccountId,
        market_hash: Hash,
        token_hash: Hash,
        router: AccountId,
        markets: Mapping<u64, MarketRef>,
        count: u64,
        collateral_rate: u16,
    }

    impl Predictor {
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

            let count = self.count;
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
            .salt_bytes(count.to_be_bytes())
            .instantiate();

            self.markets.insert(count, &market);
            self.count += 1;

            Ok(())
        }
    }
}
