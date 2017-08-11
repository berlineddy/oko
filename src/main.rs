
extern crate oko;
extern crate csv;
extern crate clap;

use oko::spot_price::SpotPriceApi;
//use oko::spot_trading::SpotTradingApi;
use oko::TradeApi;


fn main() {

    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    
    // let t = SpotTradingApi::new(TradeApi::BTC,
    
     let matches = clap::App::new("OkCoinAPi")
                          .version("1.0")
                          .author("Eddy S.")
                          .about("Does awesome things")
                          .arg(clap::Arg::with_name("depth")
                               .short("d")
                               .long("depth")
                               .help("get market depth")
                               .takes_value(false))
                          .arg(clap::Arg::with_name("ticker")
                               .short("i")
                               .long("ticker")
                               .help("get market ticker")
                               .takes_value(false))
                          .arg(clap::Arg::with_name("trades")
                               .short("r")
                               .long("trades")
                               .help("get market trades")
                               .takes_value(false))
                           .arg(clap::Arg::with_name("api")
                               .short("a")
                               .long("api")
                               .value_name("api")
                               .help("set the API to use (BTC/ETH/ETC/LTC)")
                               .takes_value(true)
                               .required(true))
                          .get_matches();
  
    let s = match matches.value_of("api").unwrap_or("BTC"){
      "BTC" => SpotPriceApi::new(TradeApi::BTC),
      "ETH" => SpotPriceApi::new(TradeApi::ETH),
      "ETC" => SpotPriceApi::new(TradeApi::ETC),
      "LTC" => SpotPriceApi::new(TradeApi::LTC),
      _ => SpotPriceApi::new(TradeApi::BTC),
    };
    
    if matches.is_present("depth") {
      let x= s.depth().unwrap();
      for i in x.depth {
	wtr.serialize(i).expect("Could not serialize Market Depth");
      }
    }
    if matches.is_present("ticker") {
      let x = s.ticker().unwrap();
      wtr.serialize(x.ticker).expect("Could not serialize Market Ticker");
    }
    if matches.is_present("trades") {
      let x = s.trades().unwrap();
      for i in x {
	wtr.serialize(i).expect("Could not serialize Market Trades");
      }
    }
    wtr.flush().expect("Could not flush stdout");
}
