use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeRates {
    asset_id_base: String,
    rates: Vec<ExchangeRate>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExchangeRate {
    asset_id_base: Option<String>,
    asset_id_quote: String,
    time: DateTime<Utc>,
    rate: f32
}
