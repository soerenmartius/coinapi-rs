const DATA_TYPE_TRADE: &'static str = "trade";
const DATA_TYPE_QUOTE: &'static str = "quote";
const DATA_TYPE_BOOK: &'static str = "book";
const DATA_TYPE_BOOK5: &'static str = "book5";
const DATA_TYPE_BOOK20: &'static str = "book20";
const DATA_TYPE_BOOK50: &'static str = "book50";

enum DataType {
    Trade(String),
    Quote(String),
    Book(String),
    Book5(String),
    Book20(String),
    Book50(String),
}

struct ServerTime {
    server_time: u64,
}

struct Hello {
    msg_type: String,
    api_key: String,
    heartbeat: bool,
    subscribe_data_type: String,
    subscribe_filter_symbol_id: String,
    subscribe_filter_asset_id: String,
}

struct Trade {
    msg_type: String,
    symbol_id: String,
    sequence: u64,
    time_exchange: u64,
    time_coinapi: u64,
    uuid: u64,
    price: f64,
    size: f64,
    taker_side: String
}

struct Quote {
    msg_type: String,
    symbol_id: String,
    sequence: u64,
    time_exchange: u64,
    time_coinapi: u64,
    ask_price: f64,
    ask_size: f64,
    bid_price: f64,
    bid_size: f64
}

struct Book {
    msg_type: DataType,
    symbol_id: String,
}

