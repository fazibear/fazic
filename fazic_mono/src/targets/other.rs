extern crate hyper;
extern crate hyper_native_tls;

use self::hyper::Client;
use self::hyper::net::HttpsConnector;
use self::hyper_native_tls::NativeTlsClient;
use std::io::Read;

const HOST: &str = "http://localhost:8080";

pub fn load(name: &String) -> Result<String, String> {
    let mut resp = String::new();
    let url = format!("{}/file/{}", HOST, name);

    let response = http()
        .get(&url)
        .send();

    match response {
        Ok(mut res) => {
            let _ = res.read_to_string(&mut resp);
            Ok(resp)
        },
        Err(error) => {
            Err(error.to_string())
        },
    }
}


pub fn save(name: &String, body: &String) -> Result<String, String> {
    let mut resp = String::new();

    let url = format!("{}/file/{}", HOST, name);

    let response = http()
        .post(&url)
        .body(body)
        .send();

    match response {
        Ok(mut res) => {
            let _ = res.read_to_string(&mut resp);
            Ok(resp)
        },
        Err(error) => {
            Err(error.to_string())
        },
    }
}

fn http() -> Client {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    Client::with_connector(connector)
}

pub fn set_main_loop_callback<F>(mut f: F) where F: FnMut() {
    loop {
        f();
    }
}
