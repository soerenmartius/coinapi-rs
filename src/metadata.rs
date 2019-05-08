use crate::client::Client;
use crate::model::metadata::{Exchanges, Assets, Symbols};
use serde_json::from_str;

#[derive(Clone)]
pub struct MetaData {
    pub client: Client
}

// handle coinapis metadata endpoints
// https://docs.coinapi.io/#metadata
impl MetaData {
    // list all exchanges
    // https://docs.coinapi.io/#list-all-exchanges
    pub fn get_all_exchanges(&self) -> Result<Exchanges, ()> {
        let data = self.client.get("/v1/exchanges", "").unwrap(); // toDo: error handling
        let exchanges: Exchanges = from_str(data.as_str()).unwrap();
        Ok(exchanges)
    }

    // list all assets
    // https://docs.coinapi.io/#list-all-assets
    pub fn get_all_assets(&self) -> Result<Assets, ()> {
        let data = self.client.get("/v1/assets", "").unwrap();  // toDo: error handling
        let assets: Assets = from_str(data.as_str()).unwrap();

        Ok(assets)
    }

    // List all symbols
    // https://docs.coinapi.io/#list-all-symbols
    pub fn get_all_symbols(&self) -> Result<Symbols, ()> {
        let data = self.client.get("/v1/symbols", "").unwrap();
        let symbols: Symbols = from_str(data.as_str()).unwrap();

        Ok(symbols)
    }
}
