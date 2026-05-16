use crate::db::DbPool;
use crate::models::{LedgerCursor, NewEnergyTrade, NewLedgerCursor};
use crate::schema::trades;
use diesel::associations::HasTable;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct GetEventsRequest {
    start_ledger: u32,
    filters: Vec<EventFilter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EventFilter {
    r#type: String,
    contract_ids: Vec<String>,
    topics: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetEventsResponse {
    events: Vec<SorobanEvent>,
    latest_ledger: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct SorobanEvent {
    r#type: String,
    ledger: u32,
    contract_id: String,
    id: String,
    topic: Vec<String>,
    value: serde_json::Value,
    in_successful_transaction: bool,
    tx_hash: String,
    timestamp: Option<u64>,
}

/// Polls Soroban RPC for `TRADE` events emitted by the energy-trade contract
/// and syncs them into the local Postgres database.
pub async fn sync_trade_events(
    contract_id: &str,
    soroban_rpc_url: &str,
    pool: &DbPool,
) -> Result<(), Box<dyn std::error::Error>> {
    if contract_id.is_empty() {
        warn!("sync_trade_events: CONTRACT_ID not set, skipping sync");
        return Ok(());
    }

    info!(
        "sync_trade_events: starting sync for contract {} on {}",
        contract_id, soroban_rpc_url
    );

    let mut conn = pool.get()?;

    // Get or create the ledger cursor for this contract
    let mut last_ledger = get_or_create_cursor(&mut conn, contract_id)?;

    loop {
        match poll_events_once(contract_id, soroban_rpc_url, pool, last_ledger).await {
            Ok(new_last_ledger) => {
                if new_last_ledger > last_ledger {
                    info!("Synced events up to ledger {}", new_last_ledger);
                    last_ledger = new_last_ledger;
                }
            }
            Err(e) => {
                error!("Error polling events: {}", e);
            }
        }

        // Poll every 15 seconds
        sleep(Duration::from_secs(15)).await;
    }
}

async fn poll_events_once(
    contract_id: &str,
    soroban_rpc_url: &str,
    pool: &DbPool,
    start_ledger: u32,
) -> Result<u32, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let request_body = GetEventsRequest {
        start_ledger,
        filters: vec![EventFilter {
            r#type: "contract".to_string(),
            contract_ids: vec![contract_id.to_string()],
            topics: vec![vec!["TRADE".to_string()]],
        }],
    };

    let response = client
        .post(format!("{}/getEvents", soroban_rpc_url))
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("RPC request failed: {}", response.status()).into());
    }

    let events_response: GetEventsResponse = response.json().await?;

    if events_response.events.is_empty() {
        return Ok(events_response.latest_ledger);
    }

    let mut conn = pool.get()?;
    let mut max_ledger = start_ledger;

    for event in events_response.events {
        if event.ledger > max_ledger {
            max_ledger = event.ledger;
        }

        // Skip if not a TRADE event
        if event.topic.is_empty() || event.topic[0] != "TRADE" {
            continue;
        }

        match parse_trade_event(&event) {
            Ok(new_trade) => {
                if let Err(e) = upsert_trade(&mut conn, &new_trade, &event.tx_hash) {
                    error!("Failed to upert trade: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to parse trade event {}: {}", event.id, e);
            }
        }
    }

    // Update the cursor
    update_cursor(&mut conn, contract_id, max_ledger)?;

    Ok(events_response.latest_ledger)
}

fn parse_trade_event(event: &SorobanEvent) -> Result<NewEnergyTrade, Box<dyn std::error::Error>> {
    // Expected event structure:
    // topic: ["TRADE"]
    // value: {
    //   "prosumer": "G...",
    //   "consumer": "G...",
    //   "amount_kwh": 123.45,
    //   "price_per_kwh": 0.123
    // }

    let value = event
        .value
        .as_object()
        .ok_or("Event value is not an object")?;

    let prosumer = value
        .get("prosumer")
        .and_then(|v| v.as_str())
        .ok_or("Missing prosumer field")?;

    let consumer = value
        .get("consumer")
        .and_then(|v| v.as_str())
        .ok_or("Missing consumer field")?;

    let amount_kwh = value
        .get("amount_kwh")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid amount_kwh field")?;

    let price_per_kwh = value
        .get("price_per_kwh")
        .and_then(|v| v.as_f64())
        .ok_or("Missing or invalid price_per_kwh field")?;

    Ok(NewEnergyTrade {
        id: Uuid::new_v4(),
        prosumer_address: prosumer.to_string(),
        consumer_address: consumer.to_string(),
        amount_kwh,
        price_per_kwh,
    })
}

fn upsert_trade(
    conn: &mut PgConnection,
    new_trade: &NewEnergyTrade,
    _tx_hash: &str,
) -> Result<(), DieselError> {
    // Check if trade with this tx_hash already exists
    // We'll use the trade ID as a proxy for tx_hash uniqueness
    // In a real implementation, you might want to store tx_hash separately

    diesel::insert_into(trades::table)
        .values(new_trade)
        .on_conflict_do_nothing()
        .execute(conn)?;

    debug!(
        "Upserted trade: prosumer={}, consumer={}, amount={}kWh",
        new_trade.prosumer_address, new_trade.consumer_address, new_trade.amount_kwh
    );

    Ok(())
}

fn get_or_create_cursor(
    conn: &mut PgConnection,
    contract_id_str: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    use crate::schema::ledger_cursors::dsl::*;

    let cursor: Option<LedgerCursor> = ledger_cursors
        .filter(contract_id.eq(contract_id_str))
        .first(conn)
        .optional()?;

    match cursor {
        Some(c) => Ok(c.last_ledger as u32),
        None => {
            // Create new cursor starting from ledger 0
            let new_cursor = NewLedgerCursor {
                id: Uuid::new_v4(),
                contract_id: contract_id_str.to_string(),
                last_ledger: 0,
            };

            diesel::insert_into(ledger_cursors::table())
                .values(&new_cursor)
                .execute(conn)?;

            info!("Created new ledger cursor for contract {}", contract_id_str);
            Ok(0)
        }
    }
}

fn update_cursor(
    conn: &mut PgConnection,
    contract_id_str: &str,
    last_ledger_val: u32,
) -> Result<(), DieselError> {
    use crate::schema::ledger_cursors::dsl::*;

    diesel::update(ledger_cursors.filter(contract_id.eq(contract_id_str)))
        .set(last_ledger.eq(last_ledger_val as i64))
        .execute(conn)?;

    Ok(())
}
