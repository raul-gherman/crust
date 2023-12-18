use prost::Message;

use crate::ctrader_open_api::ProtoMessage;

pub fn encode_proto_message_to_byte_vector(payoad_type: u32, payload: Vec<u8>) -> Vec<u8> {
    let message = ProtoMessage {
        payload_type: payoad_type,
        payload: Some(payload),
        client_msg_id: None,
    };

    message.encode_to_vec()
}
