extern crate oko;
extern crate csv;
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use oko::spot_price::SpotPriceApi;
use oko::spot_trading::SpotTradingApi;
use oko::TradeApi;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    // let t = SpotTradingApi::new(TradeApi::BTC,

    let matches = clap::App::new("OkCoinAPi")
        .version("1.0")
        .author("Eddy S.")
        .about("Does awesome things")
        .arg(
            clap::Arg::with_name("depth")
                .short("d")
                .long("depth")
                .help("get market depth")
                .takes_value(false)
                .requires("api"),
        )
        .arg(
            clap::Arg::with_name("ticker")
                .short("i")
                .long("ticker")
                .help("get market ticker")
                .takes_value(false)
                .requires("api"),
        )
        .arg(
            clap::Arg::with_name("trades")
                .short("r")
                .long("trades")
                .help("get market trades")
                .takes_value(false)
                .requires("api"),
        )
        .arg(
            clap::Arg::with_name("api")
                .short("a")
                .long("api")
                .value_name("api")
                .help("set the API to use (BTC/ETH/ETC/LTC)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("keys")
                .short("k")
                .long("keys")
                .value_name("keys")
                .help("path to the keys.toml file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("userinfo")
                .short("u")
                .long("userinfo")
                .help("get userinfo")
                .requires("keys"),
        )
        .arg(
            clap::Arg::with_name("trade_history")
                .short("y")
                .long("trade_history")
                .help("get trade history for a market")
                .value_name("since")
                .takes_value(true)
                .requires("keys")
                .requires("api"),
        )
        .get_matches();

    let api = match matches.value_of("api").unwrap_or("BTC") {
        "BTC" => TradeApi::BTC,
        "ETH" => TradeApi::ETH,
        "ETC" => TradeApi::ETC,
        "LTC" => TradeApi::LTC,
        _ => TradeApi::BTC,
    };

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
    wtr.flush().expect("Could not flush stdout");

    if matches.is_present("keys") {
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


        if matches.is_present("userinfo") {
            println!("{:#?}", s.userinfo());
        }
        if matches.is_present("trade_history") {
            println!(
                "{:#?}",
                s.trade_history(
                    matches
                        .value_of("trade_history")
                        .expect("since date not provided!")
                        .parse()
                        .expect("since date is not parseable (use unix timestamps!)"),
                )
            );

        }
    }
}
