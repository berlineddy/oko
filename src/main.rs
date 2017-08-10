
extern crate oko;
extern crate csv;

use oko::spot_price::SpotPriceApi;
use oko::spot_trading::SpotTradingApi;
use oko::TradeApi;
use oko::APIError;

use std::error::Error;
use std::thread;


fn main() {

    let s = SpotPriceApi::new(TradeApi::BTC);
    // let t = SpotTradingApi::new(TradeApi::BTC,
  
    let x = s.ticker().unwrap();
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.serialize(x);
    wtr.flush();
}
