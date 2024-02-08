use prost::Message;

pub fn cancel_order_request(order_id: i64) -> Vec<u8> {
    let cancel_order_request = crate::ctrader_open_api::ProtoOaCancelOrderReq {
        payload_type: Some(2108),
        ctid_trader_account_id: 1,
        order_id,
    };
    crate::encoder::encode_proto_message_to_byte_vector(
        cancel_order_request.payload_type.unwrap() as u32,
        cancel_order_request.encode_to_vec(),
    )
}
