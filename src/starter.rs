use crate::ctrader_open_api::ProtoOaApplicationAuthReq;
use crate::decoder::decode_incoming_message;
use crate::sender::send;

use prost::Message;

use tungstenite::connect;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use url::Url;

extern crate env_logger;
use log::info;

pub fn start() {
    let ctrader_endpoint = envmnt::get_or("CTRADER_ENDPOINT", "NOT_SET");

    let (mut socket, response): (
        WebSocket<MaybeTlsStream<std::net::TcpStream>>,
        tungstenite::http::Response<Option<Vec<u8>>>,
    ) = connect(Url::parse(&ctrader_endpoint).unwrap())
        .expect("Can't connect to {ctrader_endpoint}");
    info!("Connected to {ctrader_endpoint}, {:#?}", response);

    let client_id = envmnt::get_or("CLIENT_ID", "NOT_SET");
    let client_secret = envmnt::get_or("CLIENT_SECRET", "NOT_SET");

    let outgoing_message = ProtoOaApplicationAuthReq {
        payload_type: Some(2100),
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
