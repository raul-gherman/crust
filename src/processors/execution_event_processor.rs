use crate::ctrader_open_api::{
    ProtoOaExecutionEvent, ProtoOaExecutionType, ProtoOaOrder, ProtoOaOrderType, ProtoOaPosition,
};
use log::{error, info};
use std::collections::HashMap;

pub fn process<'a>(
    incoming_message: &'a ProtoOaExecutionEvent,
    positions: &mut HashMap<i64, ProtoOaPosition>,
    orders: &mut HashMap<i64, ProtoOaOrder>,
) {
    match incoming_message.execution_type() {
        ProtoOaExecutionType::OrderAccepted => info!("OrderAccepted"),
        ProtoOaExecutionType::OrderFilled => {
            info!("OrderFilled");
            match &incoming_message.position {
                Some(p) => {
                    match p.position_status {
                        1 /* POSITION_STATUS_OPEN */    => {
                            positions.insert(p.position_id, p.clone());
                            match &incoming_message.order {
                                Some(o) => {
                                    if orders.contains_key(&o.order_id) {
                                        orders.remove(&o.order_id);
                                    }
                                },
                                None => todo!(),
                            }
                        }
                        2 /* POSITION_STATUS_CLOSED */  => {
                            match &incoming_message.position {
                                Some(p) => {
                                    if positions.contains_key(&p.position_id) {
                                        positions.remove(&p.position_id);
                                    }
                                },
                                None => todo!(),
                            }
                        }
                        3 /* POSITION_STATUS_CREATED */ => {
                            match &incoming_message.order {
                                Some(o) => {
                                    match o.order_type() {
                                        ProtoOaOrderType::Market | ProtoOaOrderType::MarketRange | ProtoOaOrderType::StopLossTakeProfit => {

                                        },
                                        ProtoOaOrderType::Limit | ProtoOaOrderType::Stop | ProtoOaOrderType::StopLimit => {
                                            orders.insert(o.order_id, o.clone());
                                        },
                                    }
                                },
                                None => todo!(),
                            }
                        }
                        4 /* POSITION_STATUS_ERROR */   => {
                            error!("POSITION_STATUS_ERROR");
                        }
                        _ => {}
                    }
                }
                None => todo!(),
            }
        }
        crate::ctrader_open_api::ProtoOaExecutionType::OrderReplaced => {
            info!("OrderReplaced");
            match &incoming_message.order {
                Some(o) => {
                    if orders.contains_key(&o.order_id) {
                        orders.remove(&o.order_id);
                        orders.insert(o.order_id, o.clone());
                    }
                }
                None => todo!(),
            }
        }
        crate::ctrader_open_api::ProtoOaExecutionType::OrderCancelled => {
            info!("OrderCancelled");
            match &incoming_message.order {
                Some(o) => {
                    if orders.contains_key(&o.order_id) {
                        orders.remove(&o.order_id);
                    }
                }
                None => todo!(),
            }
        }
        crate::ctrader_open_api::ProtoOaExecutionType::OrderExpired => {
            info!("OrderExpired");
            match &incoming_message.order {
                Some(o) => {
                    if orders.contains_key(&o.order_id) {
                        orders.remove(&o.order_id);
                    }
                }
                None => todo!(),
            }
        }
        crate::ctrader_open_api::ProtoOaExecutionType::OrderRejected => info!("OrderRejected"),
        crate::ctrader_open_api::ProtoOaExecutionType::OrderCancelRejected => {
            info!("OrderCancelRejected")
        }
        crate::ctrader_open_api::ProtoOaExecutionType::Swap => info!("Swap"),
        crate::ctrader_open_api::ProtoOaExecutionType::DepositWithdraw => info!("DepositWithdraw"),
        crate::ctrader_open_api::ProtoOaExecutionType::OrderPartialFill => {
            info!("OrderPartialFill")
        }
        crate::ctrader_open_api::ProtoOaExecutionType::BonusDepositWithdraw => {
            info!("BonusDepositWithdraw")
        }
    }
}
