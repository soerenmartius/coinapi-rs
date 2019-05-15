use crate::client::Client;
use crate::model::exchangerate::{
    ExchangeRates as AllRates,
    ExchangeRate  as Rate,
};
use failure::Error;
use serde_json::from_str;


#[derive(Clone)]
pub struct ExchangeRate {
    pub client: Client
}

impl ExchangeRate {
    // Get the current exchange rate between requested asset and all other assets.
    // https://docs.coinapi.io/#get-all-current-rates
    pub fn get_all_rates(&self, asset_id_base: &str) -> Result<AllRates, Error> {
        let data = self.client.get(format!("/v1/exchangerate/{}", asset_id_base).as_str(), "")?;
        let rates: AllRates = from_str(data.as_str())?;

        Ok(rates)
    }

    // Get exchange rate between pair of requested assets at specific or current time.
    // https://docs.coinapi.io/#get-specific-rate
    pub fn get_specific_rate(&self, asset_id_base: &str, asset_id_quote: &str) -> Result<Rate, Error> {
        let data = self.client.get(
            format!("/v1/exchangerate/{asset_id_base}/{asset_id_quote}",
                    asset_id_base,
                    asset_id_quote).as_str(), "")?;

        let rate: Rate = from_str(data.as_str())?;

        Ok(rate)
    }
}
