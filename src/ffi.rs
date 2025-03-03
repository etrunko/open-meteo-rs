use tokio::runtime;
use std::ffi;

use crate::Client;
use crate::Location;
use crate::forecast::ForecastResult;
use crate::forecast::Options;

#[no_mangle]
pub extern "C" fn open_meteo_client_new() -> *mut Client {
    let client = Client::new();
    Box::into_raw(Box::new(client))
}

#[no_mangle]
pub extern "C" fn open_meteo_client_free(client: *mut Client) {
    unsafe {
        drop(Box::from_raw(client));
    };
}

#[no_mangle]
pub extern "C" fn open_meteo_forecast_options_new(lat: f64, lng: f64) -> *mut Options {
    let mut opts = Options::default();
    opts.location = Location {
        lat: lat,
        lng: lng,
    };

    opts.current.push("temperature_2m".into());
    Box::into_raw(Box::new(opts))
}

#[no_mangle]
pub extern "C" fn open_meteo_forecast_options_set_location(opts: &mut Options, lat: f64, lng: f64) {
    opts.location = Location {
        lat: lat,
        lng: lng,
    };
}

#[no_mangle]
pub extern "C" fn open_meteo_client_forecast(client: &mut Client, opts: *mut Options) -> *const ffi::c_char {
    let runtime = runtime::Runtime::new().unwrap();
    let result = runtime.block_on(async {
        match client.forecast(unsafe {*Box::from_raw(opts)}).await {
            Ok(res) => res,
            Err(_) => ForecastResult::default(),
        }
    });

    ffi::CString::new(format!("{:#?}", result)).unwrap().into_raw()
}
