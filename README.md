# coinapi-rs

Unofficial CoinAPI Rust SDK for the [Coinapi.io](https://docs.coinapi.io/)

[![Build Status](https://travis-ci.org/soerenmartius/coinapi-rs.png?branch=master)](https://travis-ci.org/soerenmartius/coinapi-rs)
[![MIT licensed](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE-MIT)

Welcome to the unofficial CoinAPI Rust SDK. This repository contain SDK for our CoinAPI's documented at https://docs.coinapi.io/

## To-Do

### General

- [x] Error handling with [Failure](https://crates.io/crates/failure)
- [ ] Implement [Log](https://crates.io/crates/log)
- [ ] [Lazy](https://crates.io/crates/lazy_static) load collections
- [ ] Implement tests

### API Implementation Status

- [ ] REST API
  - [x] General
    - [x] Authorization
    - [ ] Output data format
  - [x] Metadata
    - [x] List all exchanges
    - [x] List all assets
    - [x] List all symbols
  - [x] Exchanges rates
    - [x] Get specific rate
    - [x] Get all current rates
  - [x] OHLCV
    - [x] List all periods
    - [x] Latest data
    - [x] Historical Data
  - [ ] Trades
    - [ ] Current data
    - [ ] Latest data
    - [ ] Historical data
  - [ ] Quotes
    - [ ] Current data
    - [ ] Latest data
    - [ ] Historical data
  - [ ] Order book
    - [ ] Current data
    - [ ] Latest data
    - [ ] Historical data
  - [ ] Subscription
- [ ] WebSocket API
  - [ ] General
    - [ ] Hello
  - [ ] Messages
    - [ ] Trades
    - [ ] Quotes
    - [ ] Book
    - [ ] Book5
    - [ ] Book20
    - [ ] Book50
    - [ ] Heartbeat
- [ ] FIX API
  - [ ] Login (A)
  - [ ] Logout (5)
  - [ ] Trades (X)
  - [ ] Orderbooks (W)
  - [ ] Heartbeat (0)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
coinapi = { git = "https://github.com/soerenmartius/coinapi-rs.git" }
```

### Metadata

#### List all exchanges

```rust
use coinapi_rs::metadata::*;

fn main() {
    let metadata: MetaData = Coinapi::new(String::from("YOUR_API_KEY_HERE"));
    let data = metadata.get_all_exchanges();

    match data {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("Error: {:?}", e)
    }
}
```

#### List all assets

```rust
use coinapi_rs::metadata::*;

fn main() {
    let metadata: MetaData = Coinapi::new(String::from("YOUR_API_KEY_HERE"));
    let assets = metadata.get_all_assets();

    match assets {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("Error: {:?}", e)
    }
}
```

#### List all symbols

```rust
use coinapi_rs::metadata::*;

fn main() {
    let metadata: MetaData = Coinapi::new(String::from("YOUR_API_KEY_HERE"));
    let data = metadata.get_all_symbols(None);
    //let data = metadata.get_all_symbols(Some("BITSTAMP_SPOT_BTC_USD,HITBTC_SPOT_BTS_BTC")); // with filter

    match data {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("Error: {:?}", e)
    }
}
```

### Exchange rates

#### Get specific rate

```rust
use coinapi_rs::exchangerate::*;
use chrono::Utc;

fn main() {
    let exchangerate: ExchangeRate = Coinapi::new(String::from("YOUR_API_KEY_HERE"));
    let time = Utc.ymd(2018, 7, 8).and_hms(9, 10, 11);
    let data = exchangerate.get_specific_rate("BTC", "USDT", Some(time));

    match data {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("Error: {:?}", e)
    }
}

```

#### Get all current rates

```rust
use coinapi_rs::exchangerate::*;

fn main() {
    let exchangerate: ExchangeRate = Coinapi::new(String::from("YOUR_API_KEY_HERE"));
    let data = exchangerate.get_all_rates("BTC");

        match data {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("Error: {:?}", e)
    }
}
```

### OHLCV

#### List all periods
```rust
use coinapi_rs::ohlcv::*;

fn main() {
    let ohlcv: OHLCV = Coinapi::new(String::from
    ("YOUR_API_KEY_HERE"));
    let data = ohlcv.list_all_periods();

    match data {
      Ok(r)  => println!("{:#?}", r),
      Err(e) => println!("Error: {:?}", e)
    }
}
```

#### Latest data
```rust
use coinapi_rs::ohlcv::*;
use chrono::Utc;

fn main() {
    let ohlcv: OHLCV = Coinapi::new(String::from
    ("YOUR_API_KEY_HERE"));
    let data = ohlcv.latest_data("BITSTAMP_SPOT_BTC_USD", "1MIN", Some(false), Some(100i32));

    match data {
      Ok(r)  => println!("{:#?}", r),
      Err(e) => println!("Error: {:?}", e)
    }
}
```

### Historical data
```rust
use coinapi_rs::ohlcv::*;
use chrono::Utc;

fn main() {
    let ohlcv: OHLCV = Coinapi::new(String::from
    ("YOUR_API_KEY_HERE"));
    let start_time = Utc.ymd(2020, 8, 20).and_hms(5, 55, 55);
    let end_time = Utc.ymd(2020, 8, 27).and_hms(5, 55, 55);
    let data = ohlcv.historical_data("BITSTAMP_SPOT_BTC_USD", "1MIN", start_time, Some(time_end), None, Some(100i32));

    match data {
      Ok(r)  => println!("{:#?}", r),
      Err(e) => println!("Error: {:?}", e)
    }
}

## Contributors

- [Blake Willoughby](https://github.com/byblakeorriver)
- [Soren Martius](https://github.com/soerenmartius)
