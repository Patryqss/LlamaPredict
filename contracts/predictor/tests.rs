use ink::primitives::{AccountId, Hash};
use crate::predictor::{PredictorContract, MARKET_BALANCES, UNDERLYING_BALANCES};

fn set_caller(caller: AccountId) {
    ink::env::test::set_caller::<ink::env::DefaultEnvironment>(caller);
}
fn set_callee(caller: AccountId) {
    ink::env::test::set_callee::<ink::env::DefaultEnvironment>(caller);
}

#[test]
pub fn run() {
    let hash = Hash::default();
    let token_hash = Hash::default();
    let router = AccountId::from([0x0; 32]);
    let admin = AccountId::from([0x1; 32]);
    let underlying_token = AccountId::from([0x2; 32]);
    let predictor_id = AccountId::from([0x3; 32]);
    let user = AccountId::from([0x4; 32]);
    let resolved_at = 0;
    let resolution_time = 0;
    let collateral_rate = 0;
    let market_id = 0;
    let token_a = 0;
    let token_b = 1;

    unsafe { 
        UNDERLYING_BALANCES = Some(std::collections::HashMap::new()); 
        MARKET_BALANCES = Some(std::collections::HashMap::new());
    }
    set_caller(admin);
    set_callee(predictor_id);
    let mut predictor = PredictorContract::new(token_hash, router);
    {
        let r = predictor.add_market(underlying_token, hash, resolved_at, resolution_time, collateral_rate);
        r.expect("add market");
    }
    set_caller(user);
    {
        let r = predictor.add_market(underlying_token, hash, resolved_at, resolution_time, collateral_rate);
        r.expect_err("user cannot create market");
    }
    {
        unsafe { UNDERLYING_BALANCES.as_mut().unwrap().insert((underlying_token, user), 100); }
        let r = predictor.mint(market_id, 100);
        r.expect("mint failed");
    }
    {
        let r = predictor.burn(market_id, 25);
        r.expect("burn failed");
    }
    set_caller(admin);
    {
        let outcome_a = 100;
        let outcome_b = 20;
        let r = predictor.set_outcome(market_id, outcome_a, outcome_b);
        r.expect("set outcome failed");
    }
    set_caller(user);
    {
        let r = predictor.burn_by_outcome(market_id, 25);
        r.expect("burn by outcome failed");
        let b_left = unsafe { MARKET_BALANCES.as_ref().unwrap().get(&(token_b, user)).unwrap() };
        assert_eq!(*b_left, 20);
    }
}