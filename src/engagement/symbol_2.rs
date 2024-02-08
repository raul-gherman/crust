use crate::ctrader_open_api::ProtoOaNewOrderReq;
use crate::encoder::*;
use crate::symbol::Symbol;
use prost::Message;

pub fn symbol_2_open_buy(symbol_2: &mut Symbol, volume: i64) -> Vec<u8> {
    let symbol_2_buy_order_request = ProtoOaNewOrderReq {
        payload_type: Some(2106),
        ctid_trader_account_id: 1,
        symbol_id: symbol_2.id,
        order_type: 2,
        trade_side: 1,
        volume, // _variableLotSize = (long) Math.Floor(100000 * Math.Round(_first.Ask * _baseLotSize, 2)) * 100;
        limit_price: Some(111.0),
        stop_price: None,
        time_in_force: None,
        expiration_timestamp: None,
        stop_loss: None,
        take_profit: None,
        comment: Some(String::from("comment")),
        base_slippage_price: None,
        slippage_in_points: None,
        label: Some(String::from("label")),
        position_id: None,
        client_order_id: None,
        relative_stop_loss: None,
        relative_take_profit: None,
        guaranteed_stop_loss: None,
        trailing_stop_loss: None,
        stop_trigger_method: None,
    };
    encode_proto_message_to_byte_vector(
        symbol_2_buy_order_request.payload_type.unwrap() as u32,
        symbol_2_buy_order_request.encode_to_vec(),
    )
}

pub fn symbol_2_open_sell(symbol_2: &mut Symbol) -> Vec<u8> {
    let symbol_2_sell_order_request = ProtoOaNewOrderReq {
        payload_type: Some(2106),
        ctid_trader_account_id: 1,
        symbol_id: symbol_2.id,
        order_type: 2,
        trade_side: 2,
        volume: 10000,
        limit_price: Some(111.0),
        stop_price: None,
        time_in_force: None,
        expiration_timestamp: None,
        stop_loss: None,
        take_profit: None,
        comment: Some(String::from("comment")),
        base_slippage_price: None,
        slippage_in_points: None,
        label: Some(String::from("label")),
        position_id: None,
        client_order_id: None,
        relative_stop_loss: None,
        relative_take_profit: None,
        guaranteed_stop_loss: None,
        trailing_stop_loss: None,
        stop_trigger_method: None,
    };
    encode_proto_message_to_byte_vector(
        symbol_2_sell_order_request.payload_type.unwrap() as u32,
        symbol_2_sell_order_request.encode_to_vec(),
    )
}