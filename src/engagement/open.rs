use crate::ctrader_open_api::ProtoOaNewOrderReq;
use crate::symbol::Symbol;
use log::info;
use prost::Message;

pub fn buy(
    ctid: i64,
    symbol: &mut Symbol,
    volume: i64,
    comment: String,
    label: &String,
) -> Vec<u8> {
    let request = ProtoOaNewOrderReq {
        payload_type: Some(2106),
        ctid_trader_account_id: ctid,
        symbol_id: symbol.id,
        order_type: 2,
        trade_side: 1,
        volume,
        limit_price: Some(symbol.ask / 100000.00),
        stop_price: None,
        time_in_force: None,
        expiration_timestamp: None,
        stop_loss: None,
        take_profit: None,
        comment: Some(comment.to_string()),
        base_slippage_price: None,
        slippage_in_points: None,
        label: Some(label.to_string()),
        position_id: None,
        client_order_id: None,
        relative_stop_loss: None,
        relative_take_profit: None,
        guaranteed_stop_loss: None,
        trailing_stop_loss: None,
        stop_trigger_method: None,
    };
    info!("ProtoOaNewOrderReq buy: {:#?}", &request);
    request.encode_to_vec()
}

pub fn sell(
    ctid: i64,
    symbol: &mut Symbol,
    volume: i64,
    comment: String,
    label: &String,
) -> Vec<u8> {
    let request = ProtoOaNewOrderReq {
        payload_type: Some(2106),
        ctid_trader_account_id: ctid,
        symbol_id: symbol.id,
        order_type: 2,
        trade_side: 2,
        volume,
        limit_price: Some(symbol.bid / 100000.00),
        stop_price: None,
        time_in_force: None,
        expiration_timestamp: None,
        stop_loss: None,
        take_profit: None,
        comment: Some(comment.to_string()),
        base_slippage_price: None,
        slippage_in_points: None,
        label: Some(label.to_string()),
        position_id: None,
        client_order_id: None,
        relative_stop_loss: None,
        relative_take_profit: None,
        guaranteed_stop_loss: None,
        trailing_stop_loss: None,
        stop_trigger_method: None,
    };
    info!("ProtoOaNewOrderReq sell: {:#?}", &request);
    request.encode_to_vec()
}
