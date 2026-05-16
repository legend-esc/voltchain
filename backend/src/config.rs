use std::env;

/// Runtime configuration loaded from environment variables.
#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub contract_id: Option<String>,
    pub soroban_rpc_url: String,
    pub stellar_network: String,
    pub admin_secret_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            contract_id: env::var("CONTRACT_ID").ok().filter(|s| !s.is_empty()),
            soroban_rpc_url: env::var("SOROBAN_RPC_URL")
                .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string()),
            stellar_network: env::var("STELLAR_NETWORK").unwrap_or_else(|_| "testnet".to_string()),
            admin_secret_key: env::var("ADMIN_SECRET_KEY").ok().filter(|s| !s.is_empty()),
        }
    }
}
