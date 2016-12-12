
#[derive(Deserialize, Debug)]
pub struct Asset {
    net: f64,
    total: f64,
}

#[derive(Deserialize, Debug)]
pub struct Amount {
    btc: f64,
    ltc: f64,
    usd: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Funds {
    asset: Asset,
    borrow: Option<Amount>,
    free: Amount,
    freezed: Amount,
    union_fund: Option<Amount>,
}

#[derive(Deserialize, Debug)]
pub struct Info {
    funds: Funds,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    result: bool,
    info: Info,
}
