use crate::{
    ctrader_open_api::{ProtoOaOrder, ProtoOaPosition, ProtoOaReconcileRes},
    flag::Flag,
};
use log::info;
use std::collections::HashMap;

pub fn process<'a>(
    incoming_message: &'a ProtoOaReconcileRes,
    bbs_orders: &mut HashMap<i64, ProtoOaOrder>,
    ssb_orders: &mut HashMap<i64, ProtoOaOrder>,
    bbs_positions: &mut HashMap<i64, ProtoOaPosition>,
    ssb_positions: &mut HashMap<i64, ProtoOaPosition>,
    bbs_label: &String,
    ssb_label: &String,
    flag: &mut Flag,
) {
    if !bbs_orders.is_empty() {
        bbs_orders.clear();
    }
    if !ssb_orders.is_empty() {
        ssb_orders.clear();
    }
    if !bbs_positions.is_empty() {
        bbs_positions.clear();
    }
    if !ssb_positions.is_empty() {
        ssb_positions.clear();
    }

    for order in &incoming_message.order {
        if order.trade_data.label() == bbs_label {
            info!("ProtoOaReconcileRes adding bbs_orders: {:#?}", &order);
            bbs_orders.insert(order.order_id, order.clone());
        }
        if order.trade_data.label() == ssb_label {
            info!("ProtoOaReconcileRes adding ssb_orders: {:#?}", &order);
            ssb_orders.insert(order.order_id, order.clone());
        }
    }

    for position in &incoming_message.position {
        if position.trade_data.label() == bbs_label {
            info!("ProtoOaReconcileRes adding bbs_positions: {:#?}", &position);
            bbs_positions.insert(position.position_id, position.clone());
        }
        if position.trade_data.label() == ssb_label {
            info!("ProtoOaReconcileRes adding ssb_positions: {:#?}", &position);
            ssb_positions.insert(position.position_id, position.clone());
        }
    }

    if bbs_orders.len() + bbs_positions.len() > 0 {
        flag.bbs_flag = true
    } else {
        flag.bbs_flag = false
    }

    if ssb_orders.len() + ssb_positions.len() > 0 {
        flag.ssb_flag = true
    } else {
        flag.ssb_flag = false
    }
}
