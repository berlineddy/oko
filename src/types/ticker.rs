use types::deserialize::*;

#[derive(Deserialize, Debug, Serialize)]
pub struct TickerData {
    #[serde(deserialize_with = "string_to_f64")]
    pub buy: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub high: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub last: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub low: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub sell: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub vol: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Ticker {
    #[serde(deserialize_with = "string_to_u64")]
    pub date: u64,
    pub ticker: TickerData,
}
