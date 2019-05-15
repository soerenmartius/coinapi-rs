use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Periods {
    AllPeriods(Vec<Period>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Period {
    period_id: String,
    length_seconds: u32,
    length_months: u8,
    unit_count: u8,
    unit_name: String, // toDo: use enum?
    display_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum OHCLVs {
    AllOHCLs(Vec<OHCLV>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OHCLV {
    time_period_start: DateTime<Utc>,
    time_period_end: DateTime<Utc>,
    time_open: DateTime<Utc>,
    time_close: DateTime<Utc>,
    price_open: f32,
    price_high: f32,
    price_close: f32,
    price_low: f32,
    volume_traded: f32,
    trades_count: u32,
}
