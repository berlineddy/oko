extern crate oko;
extern crate csv;
extern crate clap;
extern crate toml;

use oko::spot_price::SpotPriceApi;
use oko::spot_trading::SpotTradingApi;
use oko::{TradeApi, TradeType};
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    // let t = SpotTradingApi::new(TradeApi::BTC,

    let matches = clap::App::new("OkCoinAPi")
        .version("1.0")
        .author("Eddy S.")
        .about("Does awesome things")
        
        // -------------------        
        .arg(
            clap::Arg::with_name("api")
                .long("api")
                .value_name("api")
                .help("set the API to use (BTC/ETH/ETC/LTC)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("keys")
                .long("keys")
                .value_name("keys")
                .help("path to the keys.toml file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("price")
            .long("price")
            .value_name("price")
            .help("the price")
            .takes_value(true)
        )
        .arg(
            clap::Arg::with_name("amount")
            .long("amount")
            .value_name("amount")
            .help("the amount to trade")
            .takes_value(true)
        )
        .arg(
            clap::Arg::with_name("since")
            .long("since")
            .value_name("since")
            .help("date from when to start")
            .takes_value(true)
        )
        .arg(
            clap::Arg::with_name("type")
            .long("type")
            .value_name("type")
            .help("specify trade type (sell/buy/buy_market/sell_market)")
            .takes_value(true)
        )

        
        // --------------------
        .arg(
            clap::Arg::with_name("depth")
                .long("depth")
                .help("get market depth")
                .takes_value(false)
                .requires("api"),
        )
        .arg(
            clap::Arg::with_name("ticker")
                .long("ticker")
                .help("get markuet ticker")
                .takes_value(false)
                .requires("api"),
        )
        .arg(
            clap::Arg::with_name("trades")
                .long("trades")
                .help("get market trades")
                .takes_value(false)
                .requires("api"),
        )

        // -------------------
        .arg(
            clap::Arg::with_name("userinfo")
                .long("userinfo")
                .help("get userinfo")
                .requires("keys"),
        )

        // -------------------
        .arg(
            clap::Arg::with_name("trade_history")
                .long("trade_history")
                .help("get trade history for a market")
                .requires("since")
                .requires("keys")
                .requires("api"),
        )


        // ------------------
        .arg(
            clap::Arg::with_name("create_trade")
                .long("create_trade")
                .help("create a trade")
                .requires("api")
                .requires("type")
                //.requires("price")  => not required for market_sell
                //.requires("amount") => not required for market_buy
                .requires("keys"),
        )

        .group(
            clap::ArgGroup::with_name("commands")
             .args(&["create_trade", "trade_history", "userinfo","trades", "ticker", "depth"])
             .required(true),
        )

        .get_matches();

    //-----------------------------------------------------------------------------------------------------

    let api: TradeApi = matches.value_of("api").unwrap_or("BTC").into();

    {
        // SPOT PRICE API
        let s = SpotPriceApi::new(api);

        if matches.is_present("depth") {
            let x = s.depth().unwrap();
            for i in x.depth {
                wtr.serialize(i).expect("Could not serialize Market Depth");
            }
        }
        if matches.is_present("ticker") {
            let x = s.ticker().unwrap();
            wtr.serialize(x.ticker).expect(
                "Could not serialize Market Ticker",
            );
        }
        if matches.is_present("trades") {
            let x = s.trades().unwrap();
            for i in x {
                wtr.serialize(i).expect("Could not serialize Market Trades");
            }
        }
    }

    {
        // PERSONAL API
        // ----------------------------

        let (apikey, secretkey) = {
            let mut file = File::open(matches.value_of("keys").unwrap_or("./key.toml"))
                .expect("could not read keyfile!");
            let mut keys = String::new();
            file.read_to_string(&mut keys).expect(
                "keys file not readable!",
            );
            let values = keys.parse::<toml::Value>().expect("keys not parsable!");

            (
                {
                    values["apiKey"]
                        .as_str()
                        .expect("no apiKey in file!")
                        .to_owned()
                },
                {
                    values["secretKey"]
                        .as_str()
                        .expect("no apiKey in file!")
                        .to_owned()
                },
            )
        };
        let s = SpotTradingApi::new(api, apikey, secretkey);

        // ----------------
        if matches.is_present("userinfo") {
            println!("{:#?}", s.userinfo());
        }

        // -----------------
        if matches.is_present("trade_history") {
            println!(
                "{:#?}",
                s.trade_history(
                    matches
                        .value_of("since")
                        .expect("since date not provided!")
                        .parse()
                        .expect("since date is not parseable (use unix timestamps!)"),
                )
            );
        }

        //--------------------
        if matches.is_present("create_trade") {
            let ttype: TradeType = matches
                .value_of("type")
                .expect("trade type need to be specified!")
                .into();
            if ttype == TradeType::Buy || ttype == TradeType::Sell {
                assert!(matches.is_present("amount"));
                assert!(matches.is_present("price"));
                let r = s.trade(
                    ttype, 
                    matches.value_of("amount").expect("amount is needed!").parse().expect("could not parse amount"),
                    matches.value_of("price").expect("price is needed!").parse().expect("could not parse price"))
                    .expect("could not create trade request!");
                wtr.serialize(r).expect("Could not serialize trade response");
            }
            else if ttype == TradeType::MarketBuy {
                assert!(matches.is_present("price"));
            }
            else if ttype == TradeType::MarketSell {
                assert!(matches.is_present("amount"));
            }

        }

    }
    wtr.flush().expect("Could not flush stdout");


}
