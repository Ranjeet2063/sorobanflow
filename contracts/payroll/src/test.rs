#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, Env,
};

#[test]
fn test_create_and_claim_stream() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SorobanFlowPayroll, ());
    let client = SorobanFlowPayrollClient::new(&env, &contract_id);

    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_client = TokenClient::new(&env, &token_contract.address());
    let token_admin_client = StellarAssetClient::new(&env, &token_contract.address());

    token_admin_client.mint(&sender, &10000);

    let start_time = 1000;
    let stop_time = 2000;
    let total_amount = 5000;

    let stream_id = client.create_stream(
        &sender,
        &recipient,
        &token_contract.address(),
        &total_amount,
        &start_time,
        &stop_time,
    );

    assert_eq!(stream_id, 1);

    let schedule = client.get_schedule(&stream_id).unwrap();
    assert_eq!(schedule.total_amount, 5000);
    assert_eq!(schedule.claimed_amount, 0);

    // Fast forward halfway through the stream
    env.ledger().set_timestamp(1500);

    let claimed = client.claim_stream(&recipient, &stream_id);
    assert_eq!(claimed, 2500);
    assert_eq!(token_client.balance(&recipient), 2500);

    // Fast forward to after completion
    env.ledger().set_timestamp(2500);

    let final_claimed = client.claim_stream(&recipient, &stream_id);
    assert_eq!(final_claimed, 2500);
    assert_eq!(token_client.balance(&recipient), 5000);
}
