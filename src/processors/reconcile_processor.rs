use crate::ctrader_open_api::{ProtoOaOrder, ProtoOaPosition, ProtoOaReconcileRes};
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
            info!("ProtoOaReconcileRes add BBS {:#?}", &order);
            bbs_orders.insert(order.order_id, order.clone());
        }
        if order.trade_data.label() == ssb_label {
            info!("ProtoOaReconcileRes add SSB {:#?}", &order);
            ssb_orders.insert(order.order_id, order.clone());
        }
    }

    for position in &incoming_message.position {
        if position.trade_data.label() == bbs_label {
            info!("ProtoOaReconcileRes ad BBS {:#?}", &position);
            bbs_positions.insert(position.position_id, position.clone());
        }
        if position.trade_data.label() == ssb_label {
            info!("ProtoOaReconcileRes add SSB {:#?}", &position);
            ssb_positions.insert(position.position_id, position.clone());
        }
    }
}
