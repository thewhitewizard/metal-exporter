mod client;
mod metrics;
mod state;

use client::start_price_fetching;
use metrics::start_http_server;
use state::MetalState;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let refresh_interval_secs = std::env::var("REFRESH_INTERVAL_SECS")
        .unwrap_or_else(|_| "3600".to_string())
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid REFRESH_INTERVAL_SECS value; using default of 3600 seconds.");
            21600
        });

    let state = Arc::new(MetalState::new());

    start_price_fetching(state.clone(), refresh_interval_secs).await;
    start_http_server(state).await;
}
