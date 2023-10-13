// mod combination;
mod decoder;
mod symbol;
use crate::decoder::*;
mod ctrader_open_api;
use tungstenite::connect;
use url::Url;
extern crate env_logger;
use log::info;

fn main() {
    /*
       let key: &str = "HOME";
       match env::var(key) {
           Ok(val) => println!("{key}: {val:?}"),
           Err(e) => println!("couldn't interpret {key}: {e}"),
       }
    */

    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::builder()
        // setting this to None disables the timestamp
        .format_timestamp(Some(env_logger::TimestampPrecision::Micros))
        .init();

    let server = "wss://demo.ctraderapi.com:5035";

    let (mut socket, _response) =
        connect(Url::parse(&server).unwrap()).expect("Can't connect to {server}");
    info!("Connected to {server}");

    start(&mut socket);
}
