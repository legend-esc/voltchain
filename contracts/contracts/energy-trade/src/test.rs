#![cfg(test)]
#![allow(deprecated)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_trade_and_storage() {
    let env = Env::default();
    let contract_id = env.register_contract(None, EnergyTradeContract);
    let client = EnergyTradeContractClient::new(&env, &contract_id);

    env.mock_all_auths();

    let prosumer = Address::generate(&env);
    let consumer = Address::generate(&env);

    // Perform trade
    let result = client.trade(&prosumer, &consumer, &10, &50);
    assert_eq!(result, symbol_short!("SUCCESS"));

    // Verify storage
    let trades = client.get_trades();
    assert_eq!(trades.len(), 1);
    let trade = trades.get(0).unwrap();
    assert_eq!(trade.prosumer, prosumer);
    assert_eq!(trade.consumer, consumer);
    assert_eq!(trade.amount_kwh, 10);
    assert_eq!(trade.price_per_kwh, 50);
}
