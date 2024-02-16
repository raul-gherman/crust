use crate::ctrader_open_api::{ProtoOaCancelOrderReq, ProtoOaClosePositionReq};
use log::info;
use prost::Message;

pub fn pending_order(ctid: i64, order_id: i64) -> Vec<u8> {
    let request: ProtoOaCancelOrderReq = ProtoOaCancelOrderReq {
        payload_type: Some(2108),
        ctid_trader_account_id: ctid,
        order_id,
    };
    info!("ProtoOaCancelOrderReq: {:#?}", &request);
    request.encode_to_vec()
}

pub fn position(ctid: i64, position_id: i64, volume: i64) -> Vec<u8> {
    let request: ProtoOaClosePositionReq = ProtoOaClosePositionReq {
        payload_type: Some(2111),
        ctid_trader_account_id: ctid,
        position_id,
        volume,
    };
    info!("ProtoOaClosePositionReq: {:#?}", &request);
    request.encode_to_vec()
}
