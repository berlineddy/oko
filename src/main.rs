
extern crate oko;

use oko::spot_price::SpotPriceApi;
use oko::spot_trading::SpotTradingApi;
use oko::TradeApi;
use oko::APIError;

use std::error::Error;
use std::thread;



fn main() {

    let s = SpotPriceApi::new(TradeApi::BTC);


    let mut last = 195055566;
    let mut t = s.trades(last).expect("error on getting trades");
    loop {
        thread::sleep_ms(1000);
        let n_t = s.trades(last).expect("error on getting trades");
        t.extend(n_t);
        t.sort();
        t.dedup();
        t.shrink_to_fit();
        let max = (&t).into_iter().max_by_key(|x| x.tid).expect("gnaaaa");
        let min = (&t).into_iter().min_by_key(|x| x.tid).expect("gnaaaa");


        //        if (last != max.tid) {
        println!("COUNT: {:#?}", t.len());
        // println!("MAX: {:#?}", max.tid);
        // println!("MIN: {:#?}", min.tid);
        // println!("LAST: {:#?}", last);
        last = max.tid;
        // println!("NEW-LAST: {:#?}", last);
        // }
    }
}
