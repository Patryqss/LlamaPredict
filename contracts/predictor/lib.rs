#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod predictor {
    use ink::storage::Mapping;
    use market::MarketRef;

    #[ink(storage)]
    pub struct Predictor {
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
            Self { 
                market_hash,
                token_hash,
                router,
                markets: Default::default(),
                count: 0,
                collateral_rate: 0,
            }
        }

        #[ink(message)]
        pub fn add_market(&mut self, collateral: AccountId, hash: Hash) {
            let count = self.count;
            let market_hash = self.market_hash;
            let router = self.router;
            let token_hash = self.token_hash;
            let collateral_rate = self.collateral_rate;

            let market = MarketRef::new(
                token_hash, router, collateral, hash, collateral_rate
            ).code_hash(market_hash)
            .endowment(0)
            .salt_bytes(count.to_be_bytes())
            .instantiate();

            self.markets.insert(count, &market);
            self.count += 1;
        }
    }
}
