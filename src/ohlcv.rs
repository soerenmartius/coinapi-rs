use crate::client::Client;
use crate::model::ohlcv::{
    Periods as AllPeriods,
    OHLCVs as AllOHLCs
};
use chrono::{DateTime, Utc};
use serde_json::from_str;
use std::collections::BTreeMap;
use failure::Error;

#[derive(Clone)]
pub struct OHLCV {
    pub client: Client
}

impl OHLCV {
    // Get full list of supported time periods available for requesting OHLCV timeseries data.
    // https://docs.coinapi.io/#list-all-periods
    pub fn list_all_periods(&self) -> Result<AllPeriods, Error> {
        let response = self.client.get("/v1/ohlcv/periods", "")?;
        let periods: AllPeriods = from_str(response.as_str())?;

        Ok(periods)
    }

    // Get OHLCV latest timeseries data returned in time descending order.
    // https://docs.coinapi.io/#latest-data
    pub fn latest_data(
        &self, symbol_id: &str,
        period_id: &str,
        include_empty_items: Option<bool>,
        limit: Option<i32>) -> Result<AllOHLCs, Error> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        
        parameters.insert("period_id".into(), period_id.into());
        
        if let Some(include_empty) = include_empty_items {
            parameters.insert("include_empty_items".into(), include_empty.to_string());
        }

        if let Some(lim) = limit {
            parameters.insert("limit".into(), lim.to_string());
        }

        let parameters = self.client.build_request(&parameters);

        let response = self.client.get(
            format!("/v1/ohlcv/{}/latest", symbol_id).as_str(),
            &parameters
        )?;

        let ohlcs: AllOHLCs = from_str(response.as_str())?;

        Ok(ohlcs)
    }

    // Get OHLCV timeseries data returned in time ascending order.
    // https://docs.coinapi.io/#historical-data
    pub fn historical_data(&self, symbol_id: &str,
        period_id: &str,
        time_start: DateTime<Utc>,
        time_end: Option<DateTime<Utc>>,
        include_empty_items: Option<bool>,
        limit: Option<i32>) -> Result<AllOHLCs, Error> {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        
        parameters.insert("period_id".into(), period_id.into());
            
        parameters.insert("time_start".into(), time_start.to_rfc3339());

        if let Some(end) = time_end {
            parameters.insert("time_end".into(), end.to_rfc3339());
        }

        if let Some(include_empty) = include_empty_items {
            parameters.insert("include_empty_items".into(), include_empty.to_string());
        }

        if let Some(lim) = limit {
            parameters.insert("limit".into(), lim.to_string());
        }

        let parameters = self.client.build_request(&parameters);
    
        let response = self.client.get(
            format!("/v1/ohlcv/{}/history", symbol_id).as_str(),
            &parameters
        )?;
    
        let ohlcs: AllOHLCs = from_str(response.as_str())?;
    
        Ok(ohlcs)
    }
}
