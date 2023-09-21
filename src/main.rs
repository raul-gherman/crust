// mod combination;
mod decoder;
mod symbol;

use crate::decoder::*;

mod ctrader_open_api;

use tungstenite::connect;
use url::Url;

fn main() {
    /*
       let key: &str = "HOME";
       match env::var(key) {
           Ok(val) => println!("{key}: {val:?}"),
           Err(e) => println!("couldn't interpret {key}: {e}"),
       }
    */

    let server = "wss://demo.ctraderapi.com:5035";

    let (mut socket, _response) =
        connect(Url::parse(&server).unwrap()).expect("Can't connect to {server}");
    println!("Connected to {server}");

    start(&mut socket);
}
