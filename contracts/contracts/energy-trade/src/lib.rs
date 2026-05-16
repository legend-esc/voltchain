#![no_std]
#![allow(deprecated)]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TradeRecord {
    pub prosumer: Address,
    pub consumer: Address,
    pub amount_kwh: u32,
    pub price_per_kwh: u32,
    pub timestamp: u64,
}

#[contract]
pub struct EnergyTradeContract;

#[contractimpl]
impl EnergyTradeContract {
    /// Record a trade between a prosumer and a consumer.
    pub fn trade(
        env: Env,
        prosumer: Address,
        consumer: Address,
        amount_kwh: u32,
        price_per_kwh: u32,
    ) -> Symbol {
        prosumer.require_auth();

        let record = TradeRecord {
            prosumer: prosumer.clone(),
            consumer: consumer.clone(),
            amount_kwh,
            price_per_kwh,
            timestamp: env.ledger().timestamp(),
        };

        // Store trade in a vector (simplified for demo, in production use map or specialized storage)
        let mut trades: Vec<TradeRecord> = env
            .storage()
            .persistent()
            .get(&symbol_short!("TRADES"))
            .unwrap_or(Vec::new(&env));
        trades.push_back(record);
        env.storage()
            .persistent()
            .set(&symbol_short!("TRADES"), &trades);

        // Emit trade event
        env.events().publish(
            (symbol_short!("TRADE"), prosumer, consumer),
            (amount_kwh, price_per_kwh),
        );

        symbol_short!("SUCCESS")
    }

    /// Retrieve all recorded trades.
    pub fn get_trades(env: Env) -> Vec<TradeRecord> {
        env.storage()
            .persistent()
            .get(&symbol_short!("TRADES"))
            .unwrap_or(Vec::new(&env))
    }
}

mod test;
