use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use dotenvy::dotenv;

mod config;
mod db;
mod models;
mod handlers;
mod schema;
mod stellar_sync;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cfg = config::Config::from_env();
    log::info!("Starting VoltChain Backend API on {}:{}", cfg.host, cfg.port);

    if cfg.contract_id.is_none() {
        log::warn!("CONTRACT_ID not set — on-chain integration disabled");
    } else {
        log::info!("Contract ID: {}", cfg.contract_id.as_deref().unwrap());
    }

    if cfg.admin_secret_key.is_none() {
        log::warn!("ADMIN_SECRET_KEY not set — on-chain contract calls disabled");
    }

    log::info!("Database URL: [REDACTED]");
    log::info!("Soroban RPC: {}", cfg.soroban_rpc_url);
    log::info!("Network: {}", cfg.stellar_network);

    let pool = db::init_pool(&cfg.database_url);
    let bind_addr = format!("{}:{}", cfg.host, cfg.port);

    // Spawn background sync task if contract_id is configured
    if let Some(ref contract_id) = cfg.contract_id {
        let contract_id_clone = contract_id.clone();
        let soroban_rpc_url_clone = cfg.soroban_rpc_url.clone();
        let pool_clone = pool.clone();
        
        tokio::spawn(async move {
            if let Err(e) = stellar_sync::sync_trade_events(
                &contract_id_clone,
                &soroban_rpc_url_clone,
                &pool_clone,
            ).await {
                log::error!("Background sync task failed: {}", e);
            }
        });
        
        log::info!("Started background trade sync task");
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .wrap(Logger::default())
            .wrap(cors)
            .service(handlers::health_check)
            .service(handlers::get_all_trades)
            .service(handlers::create_trade)
            .service(handlers::get_trade)
    })
    .bind(bind_addr)?
    .run()
    .await
}
