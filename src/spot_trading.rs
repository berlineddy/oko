
use serde_json;

use client::Client;
use {OkoError, TradeApi, Trade, UserInfo, TradeResponse, TradeType};

use md5;
use serde_urlencoded;

use std::string::String;
use std::fmt::Write;



#[derive(Debug)]
pub struct SpotTradingApi {
    client: Client,
    api: TradeApi,
    api_key: String,
    secret: String,
}

impl SpotTradingApi {
    fn sign(&self, mut args: Vec<(String, Option<String>)>) -> String {
        args.push(("api_key".into(), Some(self.api_key.clone())));

        let mut _arg = String::new();
        _arg = serde_urlencoded::to_string(&args).expect("formating key failed!");

        args.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        let mut sign_s = serde_urlencoded::to_string(&args).expect("formating key failed!");

        write!(sign_s, "&{}={}", "secret_key", self.secret.clone()).expect("formating key failed!");

        let mut sign_k = String::new();
        for value in md5::compute(sign_s.as_bytes()).into_iter() {
            write!(sign_k, "{:02X}", value).expect("formating key failed!");
        }

        write!(_arg, "&{}={}", "sign", sign_k).expect("formating key failed!");
        _arg
    }

    pub fn new<T: Into<String>>(api: TradeApi, key: T, secret: T) -> SpotTradingApi {
        SpotTradingApi {
            client: Client::new(),
            api: api,
            api_key: key.into(),
            secret: secret.into(),
        }
    }

    pub fn userinfo(&self) -> Result<UserInfo, OkoError> {
        let x: Vec<(String, Option<String>)> = Vec::new();

        Ok(try!(
            serde_json::from_str::<UserInfo>(&try!(self.client.post(
                &format!(
                    "https://www.okcoin.com/api/v1/userinfo.do"
                ),
                self.sign(x),
            )))
        ))
    }

    pub fn trade_history(&self, since: u64) -> Result<Vec<Trade>, OkoError> {
        let x: Vec<(String, Option<String>)> = vec![
            ("since".into(), Some(format!("{}", since))),
            ("symbol".into(), Some(format!("{}", self.api))),
        ];

        Ok(try!(
            serde_json::from_str::<Vec<Trade>>(&try!(self.client.post(
                &format!(
                    "https://www.okcoin.com/api/v1/trade_history.do"
                ),
                self.sign(x),
            )))
        ))
    }

    pub fn trade(
        &self,
        trade_type: TradeType,
        amount: f64,
        price: f64,
    ) -> Result<TradeResponse, OkoError> {
        let x: Vec<(String, Option<String>)> = vec![
            ("price".into(), Some(format!("{}", price))),
            ("type".into(), Some(format!("{}", trade_type))),
            ("amount".into(), Some(format!("{}", amount))),
            ("symbol".into(), Some(format!("{}", self.api))),
        ];

        Ok(try!(serde_json::from_str::<TradeResponse>(
            &try!(self.client.post(
                &format!(
                    "https://www.okcoin.com/api/v1/trade.do"
                ),
                self.sign(x),
            )),
        )))

    }
}
