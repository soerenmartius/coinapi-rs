use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Exchanges {
    AllExchanges(Vec<Exchange>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exchange {
    exchange_id: String,
    website: String,
    name: String,
    data_start: Option<NaiveDate>,
    data_end: Option<NaiveDate>,
    data_quote_start: Option<DateTime<Utc>>,
    data_quote_end: Option<DateTime<Utc>>,
    data_orderbook_start: Option<DateTime<Utc>>,
    data_orderbook_end: Option<DateTime<Utc>>,
    data_trade_start: Option<DateTime<Utc>>,
    data_trade_end: Option<DateTime<Utc>>,
    data_trade_count: Option<u32>,
    data_symbols_count: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Assets {
    AllAssets(Vec<Asset>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    asset_id: String,
    name: Option<String>,
    // ISO 4217 currency code
    type_is_crypto: Option<u8>, // toDo: convert to bool?
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Symbols {
    AllSymbols(Vec<Symbol>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Symbol {
    symbol_id: String,
    exchange_id: String,
    symbol_type: String,
    asset_id_base: Option<String>,
    asset_id_quote: Option<String>,
    asset_id_unit: Option<String>,
    data_start: Option<NaiveDate>,
    data_end: Option<NaiveDate>,
    volume_1hrs: Option<f32>,
    volume_1hrs_usd: Option<f32>,
    volume_1day: Option<f32>,
    volume_1day_usd: Option<f32>,
    volume_1mth: Option<f32>,
    volume_1mth_usd: Option<f32>,

    // Additional output variables for symbol_type = INDEX
    index_id: Option<String>,
    index_display_name: Option<String>,
    index_display_description: Option<String>,

    // Additional output variables for symbol_type = FUTURES
    future_delivery_time: Option<DateTime<Utc>>,

    // Additional output variables for symbol_type = OPTION
    option_type_is_call: Option<bool>,
    option_strike_price: Option<f32>,
    option_contract_unit: Option<f32>,
    option_exercise_style: Option<String>,
    option_expiration_time: Option<DateTime<Utc>>,
}
