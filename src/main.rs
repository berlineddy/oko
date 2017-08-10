
extern crate oko;
extern crate csv;

use oko::spot_price::SpotPriceApi;
use oko::spot_trading::SpotTradingApi;
use oko::TradeApi;
use oko::APIError;

use std::error::Error;
use std::thread;


fn main() {

    let s = SpotPriceApi::new(TradeApi::ETH);
    // let t = SpotTradingApi::new(TradeApi::BTC,
  
    let x = s.depth().unwrap();
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.serialize(x.asks);
    wtr.flush();
}
