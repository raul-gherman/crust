// mod combination;
mod ctrader_open_api;
mod decoder;
mod encoder;
mod symbol;

extern crate env_logger;

use crate::decoder::*;
use log::info;
use std::env;
use tungstenite::connect;
use url::Url;

fn main() {
    std::env::set_var("RUST_LOG", "DEBUG");
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Micros))
        .init();

    let ctrader_endpoint = env::var("CTRADER_ENDPOINT").expect("$CTRADER_ENDPOINT is not set");
    info!("CTRADER_ENDPOINT: {ctrader_endpoint:?}");

    let (mut socket, _response) = connect(Url::parse(&ctrader_endpoint).unwrap())
        .expect("Can't connect to {ctrader_endpoint}");
    info!("Connected to {ctrader_endpoint}");

    start(&mut socket);
}
