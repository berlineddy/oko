
extern crate oko;

use oko::spot_price::SpotPriceApi;
use oko::spot_trading::SpotTradingApi;
use oko::TradeApi;
use oko::APIError;

use std::error::Error;
use std::thread;



fn main() {

    let s = SpotPriceApi::new(TradeApi::BTC);
    // let t = SpotTradingApi::new(TradeApi::BTC,

    // let mut last = 195055566;
    // let mut t = s.trades(last).expect("error on getting trades");
    loop {
        let d = s.depth().expect("---");

        let a_min = (&d.asks).into_iter().fold(100000f64, |mut min, i: &[f64; 2]| {
            if min > i[0] {
                min = i[0];
            }
            min
        });
        let b_max = (&d.bids).into_iter().fold(0.0f64, |mut max, i: &[f64; 2]| {
            if max < i[0] {
                max = i[0];
            }
            max
        });

        let mut asks: Vec<[f64; 2]> =
            d.asks.into_iter().filter(|x| (x[0] - a_min) < 0.5f64).collect();
        let mut bids: Vec<[f64; 2]> =
            d.bids.into_iter().filter(|x| (b_max - x[0]) < 0.5f64).collect();
        // println!("asks s: {:?}", asks);
        // println!("bids s: {:?}", bids);

        let a_sum = asks.into_iter().fold(0.0f64, |acc, i: [f64; 2]| acc + i[1]);
        let b_sum = bids.into_iter().fold(0.0f64, |acc, i: [f64; 2]| acc + i[1]);
        println!("t\tasks\t/\tbid");
        println!("max\t{:?}\t/\t{:?}\t{:?}", a_min, b_max, a_min - b_max);
        println!("sum\t{:?}\t/\t{:?}", a_sum, b_sum);

        //
        thread::sleep_ms(1000);
        // let n_t = s.trades(last).expect("error on getting trades");
        // t.extend(n_t);
        // t.sort();
        // t.dedup();
        // t.shrink_to_fit();
        // let max = (&t).into_iter().max_by_key(|x| x.tid).expect("gnaaaa");
        // let min = (&t).into_iter().min_by_key(|x| x.tid).expect("gnaaaa");


        //        if (last != max.tid) {
        // println!("COUNT: {:#?}", t.len());
        // println!("MAX: {:#?}", max.tid);
        // println!("MIN: {:#?}", min.tid);
        // println!("LAST: {:#?}", last);
        // last = max.tid;
        // println!("NEW-LAST: {:#?}", last);
        // }
    }
}
