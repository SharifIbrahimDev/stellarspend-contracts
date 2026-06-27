#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, IntoVal, String, Symbol,
};

use crate::{
    events::FeeResetEventData,
    storage::{DEFAULT_FEE_BPS, DEFAULT_MIN_FEE},
    FeeContract, FeeContractClient,
};

fn setup() -> (Env, Address, FeeContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let treasury = Address::generate(&env);
    let contract_id = env.register(FeeContract, ());
    let client = FeeContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token, &treasury, &500u32, &1u64);
    (env, admin, client)
}



// --- Init / config tests ---

#[test]
fn test_init_default() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let treasury = Address::generate(&env);
    let contract_id = env.register(FeeContract, ());
    let client = FeeContractClient::new(&env, &contract_id);
    client.init(&admin, &token, &treasury);
    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.get_token(), token);
    assert_eq!(client.get_treasury(), treasury);
    assert_eq!(client.get_fee_bps(), 300);
}

#[test]
fn test_reset_fee_config_restores_defaults() {
    let (env, admin, client) = setup();
    client.set_fee_bps(&admin, &1000u32);
    client.set_min_fee(&admin, &100i128);
    assert_eq!(client.get_fee_bps(), 1000);
    assert_eq!(client.get_min_fee(), 100);
    client.reset_fee_config(&admin);
    assert_eq!(client.get_fee_bps(), 500);
    assert_eq!(client.get_min_fee(), 0);
}



#[test]
fn test_reset_fee_config_emits_event_with_restored_values() {
    let (env, admin, client) = setup();
    client.set_fee_bps(&admin, &1000u32);
    client.set_min_fee(&admin, &75i128);
    let _ = env.events().all();
    client.reset_fee_config(&admin);
    let events = env.events().all();
    let (_, topics, data) = events.last().unwrap();
    assert_eq!(
        topics,
        soroban_sdk::vec![
            &env,
            Symbol::new(&env, "fee").into_val(&env),
            Symbol::new(&env, "reset").into_val(&env)
        ]
    );
    let payload: FeeResetEventData = data.into_val(&env);
    assert_eq!(payload.admin, admin);
    assert_eq!(payload.fee_bps, DEFAULT_FEE_BPS);
    assert_eq!(payload.min_fee, DEFAULT_MIN_FEE);
    assert_eq!(payload.formatted_min_fee, String::from_str(&env, "0"));
}

#[test]
#[should_panic]
fn test_reset_fee_config_unauthorized_panics() {
    let (env, _admin, client) = setup();
    let non_admin = Address::generate(&env);
    client.reset_fee_config(&non_admin);
}



// --- Validation unit tests ---

#[test]
fn test_validate_fee_bps_valid() {
    use crate::validation::validate_fee_bps;
    assert!(validate_fee_bps(0).is_ok());
    assert!(validate_fee_bps(500).is_ok());
    assert!(validate_fee_bps(10000).is_ok());
}

#[test]
fn test_validate_fee_bps_invalid() {
    use crate::validation::validate_fee_bps;
    use crate::FeeContractError;
    assert_eq!(validate_fee_bps(10001), Err(FeeContractError::InvalidConfig));
    assert_eq!(validate_fee_bps(99999), Err(FeeContractError::InvalidConfig));
}

#[test]
fn test_validate_min_fee_valid() {
    use crate::validation::validate_min_fee;
    assert!(validate_min_fee(0).is_ok());
    assert!(validate_min_fee(100).is_ok());
    assert!(validate_min_fee(1000000).is_ok());
}

#[test]
fn test_validate_min_fee_invalid() {
    use crate::validation::validate_min_fee;
    use crate::FeeContractError;
    assert_eq!(validate_min_fee(-1), Err(FeeContractError::InvalidConfig));
    assert_eq!(validate_min_fee(-1000), Err(FeeContractError::InvalidConfig));
}
#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_collect_fee_zero_amount_panics() {
    let (env, _admin, client) = setup();
    let payer = Address::generate(&env);
    client.collect_fee(&payer, &0);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_collect_fee_negative_amount_panics() {
    let (env, _admin, client) = setup();
    let payer = Address::generate(&env);
    client.collect_fee(&payer, &-100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_collect_fee_uninitialized_panics() {
    let env = Env::default();
    let contract_id = env.register(FeeContract, ());
    let client = FeeContractClient::new(&env, &contract_id);
    let payer = Address::generate(&env);
    client.collect_fee(&payer, &100);
}

#[test]
fn test_getters_return_defaults_when_uninitialized() {
    let env = Env::default();
    let contract_id = env.register(FeeContract, ());
    let client = FeeContractClient::new(&env, &contract_id);
    
    assert_eq!(client.get_fee_bps(), DEFAULT_FEE_BPS);
    assert_eq!(client.get_min_fee(), DEFAULT_MIN_FEE);
    assert_eq!(client.get_max_fee(), 1_000_000); // DEFAULT_MAX_FEE
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_collect_fee_batch_zero_amount_panics() {
    let (env, _admin, client) = setup();
    let payer = Address::generate(&env);
    let amounts = soroban_sdk::vec![&env, 100, 0, 200];
    client.collect_fee_batch(&payer, &amounts);
}
