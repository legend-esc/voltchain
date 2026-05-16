use crate::db::DbPool;
use crate::models::{EnergyTrade, NewEnergyTrade, TradeResponse};
use crate::schema::trades;
use actix_web::{HttpResponse, Responder, get, post, web};
use chrono::Utc;
use diesel::prelude::*;
use log::{error, info, warn};
use serde_json::json;
use uuid::Uuid;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[get("/trades")]
pub async fn get_all_trades(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let results = trades::table
        .load::<EnergyTrade>(&mut conn)
        .expect("Error loading trades");

    HttpResponse::Ok().json(results)
}

#[post("/trades")]
pub async fn create_trade(
    pool: web::Data<DbPool>,
    new_trade: web::Json<NewEnergyTrade>,
    cfg: web::Data<crate::config::Config>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // Check if contract integration is available
    let tx_hash = match (&cfg.contract_id, &cfg.admin_secret_key) {
        (Some(contract_id), Some(admin_secret_key)) => {
            match invoke_trade_contract(contract_id, admin_secret_key, &new_trade).await {
                Ok(hash) => {
                    info!("Trade contract invoked successfully with tx_hash: {}", hash);
                    Some(hash)
                }
                Err(e) => {
                    error!("Failed to invoke trade contract: {}", e);
                    // Return error response without saving to DB
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Contract invocation failed",
                        "details": format!("{}", e)
                    }));
                }
            }
        }
        _ => {
            warn!("CONTRACT_ID or ADMIN_SECRET_KEY not set, proceeding with database-only trade");
            None
        }
    };

    // Create trade record
    let trade = EnergyTrade {
        id: new_trade.id,
        prosumer_address: new_trade.prosumer_address.clone(),
        consumer_address: new_trade.consumer_address.clone(),
        amount_kwh: new_trade.amount_kwh,
        price_per_kwh: new_trade.price_per_kwh,
        timestamp: Utc::now().naive_utc(),
        tx_hash: tx_hash.clone(),
    };

    // Insert into database
    match diesel::insert_into(trades::table)
        .values(&trade)
        .execute(&mut conn)
    {
        Ok(_) => {
            let response = TradeResponse {
                id: trade.id,
                prosumer_address: trade.prosumer_address,
                consumer_address: trade.consumer_address,
                amount_kwh: trade.amount_kwh,
                price_per_kwh: trade.price_per_kwh,
                timestamp: trade.timestamp,
                tx_hash,
            };
            HttpResponse::Created().json(response)
        }
        Err(e) => {
            error!("Failed to save trade to database: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database save failed",
                "details": format!("{}", e)
            }))
        }
    }
}

async fn invoke_trade_contract(
    contract_id: &str,
    admin_secret_key: &str,
    trade: &NewEnergyTrade,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create transaction using Soroban RPC
    let client = reqwest::Client::new();

    // Build transaction payload for trade function call
    let transaction_data = json!({
        "source": admin_secret_key,
        "operations": [{
            "type": "invoke_contract_function",
            "contract": contract_id,
            "function": "trade",
            "args": [
                {"type": "string", "value": trade.prosumer_address},
                {"type": "string", "value": trade.consumer_address},
                {"type": "u128", "value": (trade.amount_kwh * 1_000_000.0) as u128}, // Convert to appropriate precision
                {"type": "u128", "value": (trade.price_per_kwh * 1_000_000.0) as u128}
            ]
        }]
    });

    // Submit transaction to Soroban RPC
    let response = client
        .post("https://soroban-testnet.stellar.org/transactions")
        .json(&transaction_data)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Contract invocation failed: {}", response.status()).into());
    }

    let result: serde_json::Value = response.json().await?;

    // Extract transaction hash from response
    let tx_hash = result
        .get("hash")
        .and_then(|h| h.as_str())
        .ok_or("No transaction hash in response")?
        .to_string();

    Ok(tx_hash)
}

#[get("/trades/{id}")]
pub async fn get_trade(pool: web::Data<DbPool>, trade_id: web::Path<Uuid>) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = trades::table
        .find(trade_id.into_inner())
        .first::<EnergyTrade>(&mut conn);

    match result {
        Ok(trade) => HttpResponse::Ok().json(trade),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
