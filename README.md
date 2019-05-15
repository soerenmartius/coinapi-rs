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
  - [ ] OHLCV
    - [ ] List all periods
    - [ ] Latest data
    - [ ] Historical Data
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
    - [ ] Logon (A)
    - [ ] Logout (5)
    - [ ] Trades (X)
    - [ ] Orderbooks (W)
    - [ ] Heartbeat (0)
