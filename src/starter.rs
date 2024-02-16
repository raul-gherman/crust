use crate::ctrader_open_api::{ProtoOaApplicationAuthReq, ProtoOaPayloadType};
use crate::decoder::decode_incoming_message;
use crate::sender::send;

use prost::*;
use std::env;

use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use url::Url;

extern crate env_logger;
use log::{error, info};

pub fn start() {
    match env::var("CTRADER_ENDPOINT") {
        Ok(ctrader_endpoint) => {
            let (mut socket, response): (
                WebSocket<MaybeTlsStream<std::net::TcpStream>>,
                tungstenite::http::Response<Option<Vec<u8>>>,
            ) = connect(Url::parse(&ctrader_endpoint).unwrap())
                .expect("Can't connect to {ctrader_endpoint}");
            info!("Connected to {ctrader_endpoint}, {:#?}", response);

            let client_id = env::var("CLIENT_ID").expect("$CLIENT_ID not set");
            let client_secret = env::var("CLIENT_SECRET").expect("$CLIENT_SECRET not set");

            let outgoing_message = ProtoOaApplicationAuthReq {
                payload_type: Some(ProtoOaPayloadType::ProtoOaApplicationAuthReq.into()),
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
            };
            info!("{:#?}", &outgoing_message);

            send(
                &mut socket,
                outgoing_message.payload_type.unwrap() as u32,
                outgoing_message.encode_to_vec(),
            );

            decode_incoming_message(&mut socket);
        }
        Err(e) => error!("could not find ENV {}: {}", "CTRADER_ENDPOINT", e),
    };
}
