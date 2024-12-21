use std::sync::Mutex;

pub struct MetalState {
    gold_price_oz: Mutex<f64>,
    silver_price_oz: Mutex<f64>,
}

impl MetalState {
    pub fn new() -> Self {
        Self {
            gold_price_oz: Mutex::new(0.0),
            silver_price_oz: Mutex::new(0.0),
        }
    }

    pub fn update_prices(&self, gold_price: f64, silver_price: f64) {
        *self.gold_price_oz.lock().unwrap() = gold_price;
        *self.silver_price_oz.lock().unwrap() = silver_price;
    }

    pub fn get_prices(&self) -> (f64, f64) {
        (
            *self.gold_price_oz.lock().unwrap(),
            *self.silver_price_oz.lock().unwrap(),
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_and_get_prices() {
        let state = MetalState::new();
        state.update_prices(1800.5, 23.75);

        let (gold_price, silver_price) = state.get_prices();
        assert_eq!(gold_price, 1800.5);
        assert_eq!(silver_price, 23.75);
    }
}
