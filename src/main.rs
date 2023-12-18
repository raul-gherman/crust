// mod combination;
mod decoder;
mod encoder;
mod symbol;
use crate::decoder::*;
use std::env;
mod ctrader_open_api;
use tungstenite::connect;
use url::Url;
extern crate env_logger;
use log::{error, info};

fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::builder()
        // setting this to None disables the timestamp
        .format_timestamp(Some(env_logger::TimestampPrecision::Micros))
        .init();

    std::env::set_var("CTRADER_ENDPOINT", "wss://demo.ctraderapi.com:5035");
    let ctrader_endpoint: &str = "CTRADER_ENDPOINT";
    match env::var(ctrader_endpoint) {
        Ok(server) => {
            info!("{ctrader_endpoint}: {server:?}");

            let (mut socket, _response) =
                connect(Url::parse(&server).unwrap()).expect("Can't connect to {ctrader_endpoint}");
            info!("Connected to {ctrader_endpoint}");

            start(&mut socket);
        }
        Err(e) => {
            error!(
                "couldn't retrieve ENV value {:?} :: {:?}",
                &ctrader_endpoint, &e
            );
        }
    }
}
