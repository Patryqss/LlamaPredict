#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod predictor {
    use ink::storage::Mapping;
    use market::MarketRef;

    #[ink(storage)]
    pub struct Predictor {
        market_hash: Hash,
        markets: Mapping<u64, AccountId>,
        count: u64,
    }

    impl Predictor {
        #[ink(constructor)]
        pub fn new(
            market_hash: Hash,
        ) -> Self {
            Self { 
                market_hash,
                markets: Default::default(),
                count: 0,
            }
        }

        #[ink(message)]
        pub fn add_market(&mut self, collateral: AccountId, hash: Hash) {
            let count = self.count;

            let market = MarketRef::new(collateral, hash)
                .code_hash(self.market_hash)
                .endowment(0)
                .salt_bytes(count.to_be_bytes())
                .instantiate();
            let market = market.account_id();

            self.markets.insert(count, &market);
            self.count += 1;
        }
    }
}
