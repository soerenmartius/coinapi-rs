use crate::client::Client;
use crate::model::metadata::{Exchanges, Assets, Symbols};
use serde_json::from_str;
use std::collections::BTreeMap;
use failure::Error;

#[derive(Clone)]
pub struct MetaData {
    pub client: Client
}

// handle coinapis metadata endpoints
// https://docs.coinapi.io/#metadata
impl MetaData {
    // list all exchanges
    // https://docs.coinapi.io/#list-all-exchanges
    pub fn get_all_exchanges(&self) -> Result<Exchanges, Error> {
        let data = self.client.get("/v1/exchanges", "")?;
        let exchanges: Exchanges = from_str(data.as_str())?;

        Ok(exchanges)
    }

    // list all assets
    // https://docs.coinapi.io/#list-all-assets
    pub fn get_all_assets(&self) -> Result<Assets, Error> {
        let data = self.client.get("/v1/assets", "")?;
        let assets: Assets = from_str(data.as_str())?;

        Ok(assets)
    }

    // List all symbols
    // https://docs.coinapi.io/#list-all-symbols
    pub fn get_all_symbols(&self, symbol_filter: Option<&str>) -> Result<Symbols, Error> {

        let mut parameters: BTreeMap<String, String> = BTreeMap::new();

        if let Some(filter) = symbol_filter {
            parameters.insert("filter_symbol_id".into(), filter.into());
        }

        let request = self.client.build_request(&parameters);

        let data = self.client.get("/v1/symbols", &request)?;
        let symbols: Symbols = from_str(data.as_str())?;

        Ok(symbols)
    }


}
