use crate::Client;

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
