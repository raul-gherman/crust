mod ctrader_open_api;
mod decoder;
mod encoder;
pub mod processors;
pub mod engagement;
mod symbol;

extern crate env_logger;

use crate::decoder::*;
use log::{error, info};
use std::env;
use tungstenite::connect;
use url::Url;

fn main() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Micros))
        .init();

    match env::var("CTRADER_ENDPOINT") {
        Ok(ctrader_endpoint) => {
            let (mut socket, _response) = connect(Url::parse(&ctrader_endpoint).unwrap())
                .expect("Can't connect to {ctrader_endpoint}");
            info!("Connected to {ctrader_endpoint}");

            start(&mut socket);
        }
        Err(e) => error!("could not find ENV {}: {}", "CTRADER_ENDPOINT", e),
    };
}
