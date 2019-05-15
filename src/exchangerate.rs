use crate::client::Client;
use crate::model::exchangerate::{
    ExchangeRates as AllRates,
    ExchangeRate  as Rate,
};
use failure::Error;
use serde_json::from_str;
use chrono::{DateTime, Utc};


#[derive(Clone)]
pub struct ExchangeRate {
    pub client: Client
}

impl ExchangeRate {
    // Get the current exchange rate between requested asset and all other assets.
    // https://docs.coinapi.io/#get-all-current-rates
    pub fn get_all_rates(&self, base_currency: &str) -> Result<AllRates, Error> {
        let response = self.client.get(
            format!("/v1/exchangerate/{}",
                    base_currency).as_str(),
            "",
        )?;
        let rates: AllRates = from_str(response.as_str())?;

        Ok(rates)
    }

    // Get exchange rate between pair of requested assets at specific or current time.
    // https://docs.coinapi.io/#get-specific-rate
    pub fn get_specific_rate(
        &self, base_currency: &str,
        quote_currency: &str, time: Option<DateTime<Utc>>) -> Result<Rate, Error> {
        let response = self.client.get(
            format!("/v1/exchangerate/{}/{}",
                    base_currency,
                    quote_currency).as_str(), "")?;

        let rate: Rate = from_str(response.as_str())?;

        Ok(rate)
    }
}
