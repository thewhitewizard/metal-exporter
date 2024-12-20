use crate::state::MetalState;
use std::sync::Arc;
use tokio::signal;
use warp::Filter;

const OZ_TO_GRAM: f64 = 31.1035;
const PURITY_22K: f64 = 0.9167;
const PURITY_21K: f64 = 0.8750;
const PURITY_18K: f64 = 0.7500;

pub async fn start_http_server(state: Arc<MetalState>) {
    let metrics_route = warp::path("metrics").map(move || {
        generate_metrics(state.clone())
    });

    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C signal");
        println!("Received termination signal, shutting down gracefully.");
    };

    println!("Starting server on port 8080...");
    let (_, server) = warp::serve(metrics_route)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), shutdown_signal);

    server.await;
}

fn generate_metrics(state: Arc<MetalState>) -> String {
    let (gold_price_oz, silver_price_oz) = state.get_prices();
    let gold_gram_price = gold_price_oz / OZ_TO_GRAM;

    format!(
        "gold_oz_price {}\n\
         gold_oz_22k_price {}\n\
         gold_oz_21k_price {}\n\
         gold_oz_18k_price {}\n\
         gold_gram_price {}\n\
         gold_gram_22k_price {}\n\
         gold_gram_21k_price {}\n\
         gold_gram_18k_price {}\n\
         silver_oz_price {}\n",
        gold_price_oz,
        gold_price_oz * PURITY_22K,
        gold_price_oz * PURITY_21K,
        gold_price_oz * PURITY_18K,
        gold_gram_price,
        gold_gram_price * PURITY_22K,
        gold_gram_price * PURITY_21K,
        gold_gram_price * PURITY_18K,
        silver_price_oz
    )
}
