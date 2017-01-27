
extern crate libc;

use spot_price;
use TradeApi;
use std::ptr;

pub type SpotPriceApi = libc::c_void;

#[no_mangle]
pub extern "C" fn btc_spot_price_api_new() -> *mut libc::c_void {
    let s: Box<spot_price::SpotPriceApi> = Box::new(spot_price::SpotPriceApi::new(TradeApi::BTC));
    Box::into_raw(s) as *mut libc::c_void
}

#[no_mangle]
pub extern "C" fn btc_spot_price_api_del(handle: *mut SpotPriceApi) {
    if handle.is_null() {
        return;
    }
    unsafe {
        Box::from_raw(handle as *mut spot_price::SpotPriceApi);
    }
}


#[repr(C)]
pub struct TickerData {
    buy: libc::c_double,
    high: libc::c_double,
    last: libc::c_double,
    low: libc::c_double,
    sell: libc::c_double,
    vol: libc::c_double,
    date: libc::c_ulonglong,
}


#[no_mangle]
pub extern "C" fn btc_spot_price_api_get_ticker(handle: *mut SpotPriceApi) -> *mut TickerData {
    if handle.is_null() {
        let mut r = ptr::null();
        return r;
    }
    let mut a: Box<spot_price::SpotPriceApi> =
        unsafe { Box::from_raw(handle as *mut spot_price::SpotPriceApi) };
    let t = a.as_mut().ticker();
    println!("{:#?}", t);
    Box::into_raw(a) as *mut libc::c_void;

    Box::into_raw(Box::new(TickerData {
        buy: 0.0,
        high: 0.0,
        last: 0.0,
        low: 0.0,
        sell: 0.0,
        vol: 0.0,
        date: 0,
    }))
}
