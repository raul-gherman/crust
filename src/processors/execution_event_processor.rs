use crate::{
    ctrader_open_api::{
        ProtoOaExecutionEvent, ProtoOaExecutionType, ProtoOaOrder, ProtoOaOrderType,
        ProtoOaPosition, ProtoOaPositionStatus,
    },
    flag::Flag,
};
use log::{error, info};
use std::collections::HashMap;

pub fn process<'a>(
    incoming_message: &'a ProtoOaExecutionEvent,
    bbs_orders: &mut HashMap<i64, ProtoOaOrder>,
    ssb_orders: &mut HashMap<i64, ProtoOaOrder>,
    bbs_positions: &mut HashMap<i64, ProtoOaPosition>,
    ssb_positions: &mut HashMap<i64, ProtoOaPosition>,
    bbs_label: &String,
    ssb_label: &String,
    flag: &mut Flag,
) {
    match incoming_message.execution_type() {
        ProtoOaExecutionType::OrderAccepted => match &incoming_message.order {
            Some(o) => {
                if o.trade_data.label() == bbs_label {
                    info!(
                        "ProtoOaExecutionType::OrderAccepted: + bbs_order: {:#?}",
                        &o
                    );
                    bbs_orders.insert(o.order_id, o.clone());
                }
                if o.trade_data.label() == ssb_label {
                    info!(
                        "ProtoOaExecutionType::OrderAccepted: + ssb_order: {:#?}",
                        &o
                    );
                    ssb_orders.insert(o.order_id, o.clone());
                }
            }
            None => (),
        },

        ProtoOaExecutionType::OrderFilled => {
            match &incoming_message.position {
                Some(p) => {
                    match p.position_status() {
                        ProtoOaPositionStatus::PositionStatusOpen => {
                            if p.trade_data.label() == bbs_label {
                                info!("ProtoOaPositionStatus::PositionStatusOpen: + bbs_position: {:#?}", &p);
                                bbs_positions.insert(p.position_id, p.clone());
                            }
                            if p.trade_data.label() == ssb_label {
                                info!("ProtoOaPositionStatus::PositionStatusOpen: + ssb_position: {:#?}", &p);
                                ssb_positions.insert(p.position_id, p.clone());
                            }

                            match &incoming_message.order {
                                Some(o) => {
                                    if bbs_orders.contains_key(&o.order_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusOpen: - bbs_order: {:#?}", &o);
                                        bbs_orders.remove(&o.order_id);
                                    }
                                    if ssb_orders.contains_key(&o.order_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusOpen: - ssb_order: {:#?}", &o);
                                        ssb_orders.remove(&o.order_id);
                                    }
                                }
                                None => (),
                            }
                        }

                        ProtoOaPositionStatus::PositionStatusClosed => {
                            match &incoming_message.position {
                                Some(p) => {
                                    if bbs_positions.contains_key(&p.position_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusClosed: - bbs_position: {:#?}", &p);
                                        bbs_positions.remove(&p.position_id);
                                    }
                                    if ssb_positions.contains_key(&p.position_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusClosed: - ssb_position: {:#?}", &p);
                                        ssb_positions.remove(&p.position_id);
                                    }
                                }
                                None => (),
                            }
                        }

                        ProtoOaPositionStatus::PositionStatusCreated => {
                            match &incoming_message.order {
                                Some(o) => {
                                    match o.order_type() {
                                        ProtoOaOrderType::Market
                                        | ProtoOaOrderType::MarketRange
                                        | ProtoOaOrderType::StopLossTakeProfit => {
                                            // TODO
                                        }

                                        ProtoOaOrderType::Limit
                                        | ProtoOaOrderType::Stop
                                        | ProtoOaOrderType::StopLimit => {
                                            if o.trade_data.label() == bbs_label {
                                                info!("ProtoOaPositionStatus::PositionStatusCreated: + bbs_order: {:#?}", &o);
                                                bbs_orders.insert(o.order_id, o.clone());
                                            }
                                            if o.trade_data.label() == ssb_label {
                                                info!("ProtoOaPositionStatus::PositionStatusCreated: + ssb_order: {:#?}", &o);
                                                ssb_orders.insert(o.order_id, o.clone());
                                            }
                                        }
                                    }
                                }
                                None => (),
                            }
                        }

                        ProtoOaPositionStatus::PositionStatusError => {
                            match &incoming_message.position {
                                Some(p) => {
                                    error!("POSITION_STATUS_ERROR: {:#?}", &p);
                                }
                                None => {
                                    error!("POSITION_STATUS_ERROR");
                                }
                            }
                        }
                    }
                }
                None => (),
            }
        }

        ProtoOaExecutionType::OrderReplaced => {
            //info!("OrderReplaced");
            match &incoming_message.order {
                Some(o) => {
                    if o.trade_data.label() == bbs_label {
                        if bbs_orders.contains_key(&o.order_id) {
                            info!(
                                "ProtoOaExecutionType::OrderReplaced: - bbs_order: {:#?}",
                                &o
                            );
                            bbs_orders.remove(&o.order_id);
                        }
                        info!(
                            "ProtoOaExecutionType::OrderReplaced: + bbs_order: {:#?}",
                            &o
                        );
                        bbs_orders.insert(o.order_id, o.clone());
                    }
                    if o.trade_data.label() == ssb_label {
                        if ssb_orders.contains_key(&o.order_id) {
                            info!(
                                "ProtoOaExecutionType::OrderReplaced: - ssb_order: {:#?}",
                                &o
                            );
                            ssb_orders.remove(&o.order_id);
                        }
                        info!(
                            "ProtoOaExecutionType::OrderReplaced: + ssb_order: {:#?}",
                            &o
                        );
                        ssb_orders.insert(o.order_id, o.clone());
                    }
                }
                None => (),
            }
        }

        ProtoOaExecutionType::OrderCancelled => {
            //info!("OrderCancelled");
            match &incoming_message.order {
                Some(o) => {
                    if bbs_orders.contains_key(&o.order_id) {
                        info!(
                            "ProtoOaExecutionType::OrderCancelled: - bbs_order: {:#?}",
                            &o
                        );
                        bbs_orders.remove(&o.order_id);
                    }
                    if ssb_orders.contains_key(&o.order_id) {
                        info!(
                            "ProtoOaExecutionType::OrderCancelled: - ssb_order: {:#?}",
                            &o
                        );
                        ssb_orders.remove(&o.order_id);
                    }
                }
                None => (),
            }
        }

        ProtoOaExecutionType::OrderExpired => {
            //info!("OrderExpired");
            match &incoming_message.order {
                Some(o) => {
                    if bbs_orders.contains_key(&o.order_id) {
                        info!("ProtoOaExecutionType::OrderExpired: - bbs_order: {:#?}", &o);
                        bbs_orders.remove(&o.order_id);
                    }
                    if ssb_orders.contains_key(&o.order_id) {
                        info!("ProtoOaExecutionType::OrderExpired: - ssb_order: {:#?}", &o);
                        ssb_orders.remove(&o.order_id);
                    }
                }
                None => (),
            }
        }
        ProtoOaExecutionType::OrderRejected => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::OrderRejected: {:#?}", &o);
            }
            None => {
                info!("ProtoOaExecutionType::OrderRejected");
            }
        },

        ProtoOaExecutionType::OrderCancelRejected => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::OrderCancelRejected: {:#?}", &o);
            }
            None => {
                info!("ProtoOaExecutionType::OrderCancelRejected");
            }
        },

        ProtoOaExecutionType::Swap => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::Swap: {:#?}", &o);
            }
            None => {
                info!("ProtoOaExecutionType::Swap");
            }
        },

        ProtoOaExecutionType::DepositWithdraw => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::DepositWithdraw: {:#?}", &o);
            }
            None => {
                info!("ProtoOaExecutionType::DepositWithdraw");
            }
        },

        ProtoOaExecutionType::OrderPartialFill => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::OrderPartialFill: {:#?}", &o);
            }
            None => {
                info!("ProtoOaExecutionType::OrderPartialFill");
            }
        },

        ProtoOaExecutionType::BonusDepositWithdraw => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::BonusDepositWithdraw: {:#?}", &o);
            }
            None => {
                info!("ProtoOaExecutionType::BonusDepositWithdraw");
            }
        },
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
