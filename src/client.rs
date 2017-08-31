use std::env;
use std::io::Read;

use hyper;
use hyper::header::{ContentType, Accept, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde_json;

use OkoError;
use APIError;

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error_code: u32,
    result: bool,
}

#[derive(Debug)]
pub struct Client {
    handle: hyper::Client,
}
impl Client {
    pub fn new() -> Client {
        Client {
            handle: if let Ok(proxy) = env::var("http_proxy") {
                let s = proxy.split(":").collect::<Vec<&str>>();
                assert_eq!(s.len(), 2);

                let proxy_url = s[0].to_owned();
                let proxy_port: u16 = s[1].parse().expect("http_proxy PORT missconfigured!");

                hyper::Client::with_http_proxy(proxy_url, proxy_port)
            } else {
                hyper::Client::new()
            },
        }
    }

    pub fn get<T: hyper::client::IntoUrl>(&self, url: T) -> Result<String, OkoError> {
        let mut res = try!(self.handle.get(url).send());
        let mut x = String::new();
        try!(res.read_to_string(&mut x));
        Ok(x)
    }

    pub fn post<T: hyper::client::IntoUrl, S: Into<String>>(
        &self,
        url: T,
        body: S,
    ) -> Result<String, OkoError> {

        let mut res = try!(
            self.handle
                .post(url)
                .header(ContentType::form_url_encoded())
                .header(Accept(
                    vec![qitem(Mime(TopLevel::Star, SubLevel::Star, vec![]))],
                ))
                .body(&body.into())
                .send()
        );
        let mut x = String::new();
        try!(res.read_to_string(&mut x));
        match serde_json::from_str::<ErrorResponse>(&x) {
            Ok(val) => Err(APIError::new(val.error_code).into()),
            Err(_) => Ok(x),
        }

    }
}
