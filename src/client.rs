use crate::state::MetalState;
use reqwest::Client;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

const METAL_API_URL: &str = "https://data-asg.goldprice.org";

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


pub struct ClientWrapper {
    client: Client,
    base_url: String,
}

impl ClientWrapper {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn fetch_prices(&self) -> Result<(f64, f64), reqwest::Error> {
        let url = format!("{}/dbXRates/EUR", self.base_url);
        let response = self.client.get(&url).send().await?.json::<ApiResponse>().await?;
        let gold_price = response.items[0].xau_price;
        let silver_price = response.items[0].xag_price;
        Ok((gold_price, silver_price))
    }
}


pub async fn start_price_fetching(state: Arc<MetalState>, refresh_interval_secs: u64) {
    tokio::spawn(async move {
        let client = ClientWrapper::new(METAL_API_URL.to_string());
        loop {
            match client.fetch_prices().await {
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_fetch_prices_success() {
        let _mock = mock("GET", "/dbXRates/EUR")
            .with_status(200)
            .with_body("{\"items\":[{\"xauPrice\":1800.5,\"xagPrice\":23.75}]}")
            .with_header("content-type", "application/json")
            .create();
        
        let client = ClientWrapper::new(mockito::server_url()); 
        let result = client.fetch_prices().await;

        assert!(result.is_ok());
        let (gold, silver) = result.unwrap();
        assert_eq!(gold, 1800.5);
        assert_eq!(silver, 23.75);
    }

    #[tokio::test]
    async fn test_fetch_prices_failure() {
        let _mock = mock("GET", "/dbXRates/EUR")
            .with_status(500)
            .create();

        let client = ClientWrapper::new(mockito::server_url());
        let result = client.fetch_prices().await;

        assert!(result.is_err());
    }
}
