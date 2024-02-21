mod ctrader_open_api;
mod decoder;
mod engagement;
mod processors;
mod sender;
mod starter;
mod symbol;

extern crate env_logger;

use starter::start;

fn main() {
    env_logger::builder()
        .format_timestamp(Some(env_logger::TimestampPrecision::Micros))
        .init();

    start()
}
