use types::deserialize::*;

use std::fmt;
use std::cmp::{Ord, Ordering};


#[derive(Debug, PartialEq, Serialize)]
pub enum TradeApi {
    BTC,
    LTC,
    ETH,
    ETC,
}
impl fmt::Display for TradeApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TradeApi::BTC => write!(f, "btc_usd"),
	    &TradeApi::LTC => write!(f, "ltc_usd"),
            &TradeApi::ETC => write!(f, "etc_usd"),
            &TradeApi::ETH => write!(f, "eth_usd"),
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
pub struct Trade {
    pub date: u64,
    pub date_ms: u64,
    pub price: f64,
    pub amount: f64,
    pub tid: u64,
    #[serde(rename="type")]
    pub trade_type: TradeType,
}
impl PartialEq for Trade {
    fn eq(&self, other: &Trade) -> bool {
        self.tid == other.tid
    }
}
impl Ord for Trade {
    fn cmp(&self, other: &Trade) -> Ordering {
        self.tid.cmp(&other.tid)
    }
}
impl PartialOrd for Trade {
    fn partial_cmp(&self, other: &Trade) -> Option<Ordering> {
        Some(self.tid.cmp(&other.tid))
    }
}
impl Eq for Trade {}


#[derive(Deserialize, Debug, Serialize)]
pub struct TradeResponse {
    order_id: u64,
    result: bool,
}
