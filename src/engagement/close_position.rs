use prost::Message;

pub fn close_position_request(ctid: i64, position_id: i64, position_volume: i64) -> Vec<u8> {
    let close_position_request = crate::ctrader_open_api::ProtoOaClosePositionReq {
        payload_type: Some(2111),
        ctid_trader_account_id: ctid,
        position_id,
        volume: position_volume
    };
    crate::encoder::encode_proto_message_to_byte_vector(
        close_position_request.payload_type.unwrap() as u32,
        close_position_request.encode_to_vec(),
    )
}
