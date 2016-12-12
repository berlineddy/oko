#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate hyper;
extern crate md5;
extern crate serde_urlencoded;

pub mod oko_types;
pub use oko_types::*;

pub mod spot_price;

pub mod spot_trading;

mod oko_error;
pub use oko_error::OkoError;
pub use oko_error::APIError;

mod client;
