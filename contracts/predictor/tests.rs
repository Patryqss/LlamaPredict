use ink::primitives::{AccountId, Hash};
use crate::predictor::PredictorContract;

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
    let resolved_at = 0;
    let resolution_time = 0;
    let collateral_rate = 0;

    set_caller(admin);
    set_callee(predictor_id);
    let mut predictor = PredictorContract::new(token_hash, router);
    {
        let r = predictor.add_market(underlying_token, hash, resolved_at, resolution_time, collateral_rate);
        r.expect("OK");       
    }
    return;
}