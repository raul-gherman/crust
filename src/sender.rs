use log::{debug, error};
use prost::Message;
use std::net::TcpStream;
use tungstenite::{stream::MaybeTlsStream, Message as message, WebSocket};

use crate::ctrader_open_api::ProtoMessage;

pub fn send(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    payload_type: u32,
    payload: Vec<u8>,
) {
    
    let message = ProtoMessage {
        payload_type: payload_type,
        payload: Some(payload),
        client_msg_id: None,
    };
    
    let encoded_message = message::from(message.encode_to_vec());
    match socket.send(encoded_message) {
        Ok(()) => {
            debug!("socket.send {:#?}", &payload_type)
        }
        Err(e) => {
            error!("socket.send {:#?} :: {:#?}", &payload_type, &e);
        }
    }
}
