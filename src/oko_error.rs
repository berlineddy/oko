use std::error;
use std::io;
use std::fmt;
use std::error::Error;

use hyper;
use serde_json;

#[derive(Debug)]
pub enum OkoError {
    Io(io::Error),
    Http(hyper::Error),
    Parser(serde_json::Error),
    API(APIError),
}

impl From<APIError> for OkoError {
    fn from(err: APIError) -> OkoError {
        OkoError::API(err)
    }
}
impl From<hyper::Error> for OkoError {
    fn from(err: hyper::Error) -> OkoError {
        OkoError::Http(err)
    }
}
impl From<serde_json::Error> for OkoError {
    fn from(err: serde_json::Error) -> OkoError {
        OkoError::Parser(err)
    }
}
impl From<io::Error> for OkoError {
    fn from(err: io::Error) -> OkoError {
        OkoError::Io(err)
    }
}
impl error::Error for OkoError {
    fn description(&self) -> &str {
        match self {
            &OkoError::API(ref x) => x.description(),
            &OkoError::Io(ref x) => x.description(),
            &OkoError::Http(ref x) => x.description(),
            &OkoError::Parser(ref x) => x.description(),
        }
    }
}
impl fmt::Display for OkoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}


#[derive(Deserialize, Debug)]
pub struct APIError {
    code: u32,
}
impl APIError {
    pub fn new(code: u32) -> APIError {
        APIError { code: code }
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl error::Error for APIError {
    fn description(&self) -> &str {
        match self.code {
            503 => "Too many requests (Http)",
            10000 => "Illegal request",
            10001 => "Illegal parameters",
            10002 => "Authentication failure",
            10003 => "This connection has requested other user data",
            10004 => "This connection did not request this user data",
            10005 => "System error",
            10007 => "Signature does not match",
            10009 => "Order does not exist",
            10010 => "Insufficient funds",
            10011 => "Order quantity too low",
            10012 => "Only support btc_usd ltc_usd",
            10014 => "Order price must be between 0 - 1,000,000",
            10015 => "Channel subscription temporally not available",
            10016 => "Insufficient coins",
            10017 => "WebSocket authorization error",
            10018 => "borrow amount less than lower limit [usd:100,btc:0.1,ltc:1]",
            10019 => "loan agreement not checked",
            10023 => "fail to get latest ticker",
            10024 => "balance not sufficient",
            10025 => "quota is full, cannot borrow temporarily",
            10026 => "Loan (including reserved loan) and margin cannot be withdrawn",
            10027 => "Cannot withdraw within 24 hrs of authentication information modification",
            10029 => "Account has unpaid loan, please cancel/pay off the loan before withdraw",
            10032 => "Please enabled phone/google authenticator",
            10035 => "Insufficient BTC/LTC",
            10037 => "Trade password not set",
            10042 => "Admin password error",
            10044 => "fail to cancel borrowing order",
            10047 => "this function is disabled for sub-account",
            10050 => "can't cancel more than once",
            10060 => "Your withdrawal has been locked. Please contact support.",
            10100 => "user frozen",
            10216 => "non-public API",
            10049 => "User can not have more than 50 unfilled small orders (amount<0.5BTC)",
            20001 => "user does not exist",
            20002 => "user frozen",
            20003 => "frozen due to force liquidation",
            20004 => "contract account frozen",
            20005 => "user contract account does not exist",
            20006 => "required field can not be null",
            20007 => "illegal parameter",
            20008 => "contract account fund balance is zero",
            20009 => "contract status error",
            20010 => "risk rate information does not exist",
            20011 => "risk rate bigger than 90% before opening position",
            20012 => "risk rate bigger than 90% after opening position",
            20013 => "temporally no counter party price",
            20014 => "system error",
            20015 => "order does not exist",
            20016 => "liquidation quantity bigger than holding",
            20017 => "not authorized/illegal order ID",
            20018 => "order price higher than 105% or lower than 95% of the price of last minute",
            20019 => "IP restrained to access the resource",
            20020 => "secret key does not exist",
            20021 => "index information does not exist",
            20022 => "wrong API interface",
            20023 => "fixed margin user",
            20024 => "signature does not match",
            20025 => "leverage rate error",
            20026 => "API Permission Error",
            20027 => "no transaction record",
            20028 => "no such contract",
            20029 => "Amount is large than available funds",
            20030 => "Account still has debts",
            20038 => {
                "Due to regulation, this function is not availavle in the country/region your \
                 currently reside in."
            }
            20049 => "Request frequency too high",
            _ => "UNKNOWEN ERROR",
        }
    }
}
