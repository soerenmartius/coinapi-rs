use crate::client::Client;
use crate::metadata::MetaData;
use crate::exchangerate::ExchangeRate;
use crate::ohlcv::OHLCV;

//#[derive(Clone)]
pub trait Coinapi {
    fn new(api_key: String) -> Self;
}

impl Coinapi for MetaData {
    fn new(api_key: String) -> MetaData {
        MetaData {
            client: Client::new(api_key),
        }
    }
}

impl Coinapi for ExchangeRate {
    fn new(api_key: String) -> ExchangeRate {
        ExchangeRate {
            client: Client::new(api_key),
        }
    }
}

impl Coinapi for OHLCV {
    fn new(api_key: String) -> OHLCV {
        OHLCV {
            client: Client::new(api_key),
        }
    }
}

