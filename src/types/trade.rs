
use types::deserialize::*;

use std::fmt;
use std::cmp::{Ord, Ordering};


#[derive(Debug, PartialEq, Serialize, Clone, Copy)]
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
impl<T> From<T> for TradeApi 
    where T: Into<String>{
    fn from(val: T) -> Self {
        let x = val.into();
        match x.as_ref() {
            "BTC" => TradeApi::BTC,
            "ETH" => TradeApi::ETH,
            "ETC" => TradeApi::ETC,
            "LTC" => TradeApi::LTC,
            _ => TradeApi::BTC,
        }
    }
}

#[derive(PartialEq, Deserialize, Debug, Serialize)]
pub enum TradeType {
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell_market")]
    MarketSell,
    #[serde(rename = "buy_market")]
    MarketBuy,
}
impl fmt::Display for TradeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TradeType::Ask => write!(f, "ask"),
            &TradeType::Bid => write!(f, "bid"),
            &TradeType::Sell => write!(f, "sell"),
            &TradeType::Buy => write!(f, "buy"),
            &TradeType::MarketBuy => write!(f, "buy_market"),
            &TradeType::MarketSell => write!(f, "sell_market"),
        }
    }
}
impl<T> From<T> for TradeType 
    where T: Into<String>{
    fn from(val: T) -> Self {
        let x = val.into();
        match x.as_ref() {
            "ask" => TradeType::Ask,
            "bid" => TradeType::Bid,
            "sell" => TradeType::Sell,
            "buy" => TradeType::Buy,
            "buy_market" => TradeType::MarketBuy,
            "sell_market" => TradeType::MarketSell,
            _ => TradeType::Buy,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Trade {
    pub date: u64,
    pub date_ms: u64,
    #[serde(deserialize_with = "string_to_f64")]
    pub price: f64,
    #[serde(deserialize_with = "string_to_f64")]
    pub amount: f64,
    pub tid: u64,
    #[serde(rename = "type")]
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
