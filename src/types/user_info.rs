use types::deserialize::*;

#[derive(Deserialize, Debug, Serialize)]
pub struct Asset {
#[serde(deserialize_with = "string_to_f64")]
    net: f64,
#[serde(deserialize_with = "string_to_f64")]
    total: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Amount {
#[serde(deserialize_with = "string_to_f64")]
    btc: f64,
#[serde(deserialize_with = "string_to_f64")]
    ltc: f64,
#[serde(deserialize_with = "string_to_f64")]
    usd: f64,
#[serde(deserialize_with = "string_to_f64")]
    eth: f64,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Funds {
    asset: Asset,
    borrow: Option<Amount>,
    free: Amount,
    freezed: Amount,
    union_fund: Option<Amount>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Info {
    funds: Funds,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UserInfo {
    result: bool,
    info: Info,
}
