
extern crate oco;

use oco::spot_price::SpotPriceApi;
use oco::spot_trading::SpotTradingApi;
use oco::TradeApi;
use oco::APIError;
use std::error::Error;

fn main() {
    let s = SpotPriceApi::new(TradeApi::BTC);
    let t = SpotTradingApi::new(TradeApi::BTC,

    let f = t.trade_history(400);
    println!("{:#?}", f);
    match f {
        Ok(x) => println!("{:#?}", x.len()),
        Err(x) => println!("{:#?}", x.description()),
    }
}
