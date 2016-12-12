use std::fmt;


#[derive(Deserialize, Debug)]
pub struct TickerData {
    buy: f64,
    high: f64,
    last: f64,
    low: f64,
    sell: f64,
    vol: f64,
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    date: u64,
    ticker: TickerData,
}

#[derive(Deserialize, Debug)]
pub struct Depth {
    asks: Vec<[f64; 2]>,
    bids: Vec<[f64; 2]>,
}



#[derive(Debug, PartialEq)]
pub enum TradeApi {
    BTC,
    LTC,
}
impl fmt::Display for TradeApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TradeApi::BTC => write!(f, "btc_usd"),
            &TradeApi::LTC => write!(f, "ltc_usd"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum TradeType {
    #[serde(rename="sell")]
    Sell,
    #[serde(rename="buy")]
    Buy,
    #[serde(rename="market_sell")]
    MarketSell,
    #[serde(rename="")]
    MarketBuy,
}
impl fmt::Display for TradeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TradeType::Sell => write!(f, "sell"),
            &TradeType::Buy => write!(f, "buy"),
            &TradeType::MarketBuy => write!(f, "market_buy"),
            &TradeType::MarketSell => write!(f, "market_sell"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Trade {
    date: u64,
    date_ms: u64,
    price: f64,
    amount: f64,
    tid: u64,
    #[serde(rename="type")]
    trade_type: TradeType,
}

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

#[derive(Deserialize, Debug)]
pub struct TradeResponse {
    order_id: u64,
    result: bool,
}
