use std::collections::HashMap;
use crate::ctrader_open_api::{ProtoOaOrder, ProtoOaPosition, ProtoOaReconcileRes};
//use log::info;

pub fn process<'a>(
    incoming_message: &'a ProtoOaReconcileRes,
    positions: &mut HashMap<i64, ProtoOaPosition>,
    orders: &mut HashMap<i64, ProtoOaOrder>,
) {
    if !positions.is_empty() {
        positions.clear();
    }
    for position in &incoming_message.position {
        //info!("{:#?}", &position);
        positions.insert(position.position_id, position.clone());
    }

    if !orders.is_empty() {
        orders.clear();
    }
    for order in &incoming_message.order {
        //info!("{:#?}", &orders);
        orders.insert(order.order_id, order.clone());
    }
}
