use crate::Client;
use crate::Location;
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
pub extern "C" fn open_meteo_forecast_options_new() -> *mut Options {
    let mut opts = Options::default();
    opts.current.push("temperature_2m".into());
    Box::into_raw(Box::new(opts))
}

#[no_mangle]
pub extern "C" fn open_meteo_forecast_options_set_location(options: *mut Options, lat: f64, lng: f64) {
    let mut opts = unsafe {*Box::from_raw(options)};
    opts.location = Location {
        lat: lat,
        lng: lng,
    };
}
