use crate::state::MetalState;
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

const METAL_API_URL: &str = "https://data-asg.goldprice.org/dbXRates/EUR";

#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub xau_price: f64,
    pub xag_price: f64,
}

pub async fn start_price_fetching(state: Arc<MetalState>, refresh_interval_secs: u64) {
    tokio::spawn(async move {
        let client = Client::new();

        loop {
            match fetch_prices(&client).await {
                Ok((gold, silver)) => {
                    state.update_prices(gold, silver);
                    println!(
                        "[{}] Refreshed prices: gold = {}, silver = {}",
                        chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                        gold,
                        silver
                    );
                }
                Err(err) => eprintln!("Error fetching prices: {}", err),
            }
            sleep(Duration::from_secs(refresh_interval_secs)).await;
        }
    });
}

async fn fetch_prices(client: &Client) -> Result<(f64, f64), reqwest::Error> {
    let response = client.get(METAL_API_URL).send().await?.json::<ApiResponse>().await?;
    let gold_price = response.items[0].xau_price;
    let silver_price = response.items[0].xag_price;
    Ok((gold_price, silver_price))
}
