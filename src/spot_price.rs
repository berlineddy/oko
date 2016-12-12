use serde_json;

use client::Client;
use {Depth, Trade, TradeApi, Ticker, OkoError};

#[derive(Debug)]
pub struct SpotPriceApi {
    client: Client,
    api: TradeApi,
}

impl SpotPriceApi {
    pub fn new(api: TradeApi) -> SpotPriceApi {
        SpotPriceApi {
            client: Client::new(),
            api: api,
        }
    }

    pub fn ticker(&self) -> Result<Ticker, OkoError> {
        Ok(try!(serde_json::from_str::<Ticker>(&try!(self.client
            .get(&format!("https://www.okcoin.com/api/v1/ticker.do?symbol={}",
                          if self.api == TradeApi::BTC {
                              "btc_usd"
                          } else {
                              "ltc_usd"
                          }))))))
    }

    pub fn trades(&self, since: u64) -> Result<Vec<Trade>, OkoError> {
        Ok(try!(serde_json::from_str::<Vec<Trade>>(&try!(self.client
            .get(&format!("https://www.okcoin.com/api/v1/trades.do?since={}?symbol={}",
                          since,
                          if self.api == TradeApi::BTC {
                              "btc_usd"
                          } else {
                              "ltc_usd"
                          }))))))
    }

    pub fn depth(&self) -> Result<Depth, OkoError> {
        Ok(try!(serde_json::from_str::<Depth>(&try!(self.client
            .get(&format!("https://www.okcoin.com/api/v1/depth.do?symbol={}",
                          if self.api == TradeApi::BTC {
                              "btc_usd"
                          } else {
                              "ltc_usd"
                          }))))))
    }
}
