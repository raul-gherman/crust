use log::{debug, error};
use std::net::TcpStream;
use tungstenite::{stream::MaybeTlsStream, Message as message, WebSocket};

use crate::encoder::encode_proto_message_to_byte_vector;

pub fn send(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    payload_type: u32,
    payload: Vec<u8>,
) {
    let message = message::from(encode_proto_message_to_byte_vector(payload_type, payload));
    match socket.send(message::from(message)) {
        Ok(()) => {
            debug!("socket.send {:?}", &payload_type)
        }
        Err(e) => {
            error!("socket.send {:?} :: {:?}", &payload_type, &e);
        }
    }
}
