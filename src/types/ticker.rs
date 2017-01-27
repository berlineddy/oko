
#[repr(C)]
#[derive(Deserialize, Debug)]
pub struct TickerData {
    buy: f64,
    high: f64,
    last: f64,
    low: f64,
    sell: f64,
    vol: f64,
}

#[repr(C)]
#[derive(Deserialize, Debug)]
pub struct Ticker {
    date: u64,
    ticker: TickerData,
}
