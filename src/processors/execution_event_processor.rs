use crate::ctrader_open_api::{
    ProtoOaExecutionEvent, ProtoOaExecutionType, ProtoOaOrder, ProtoOaOrderType, ProtoOaPosition,
    ProtoOaPositionStatus,
};
use log::{debug, error, info, warn};
use std::collections::HashMap;

pub fn process<'a>(
    incoming_message: &'a ProtoOaExecutionEvent,
    bbs_orders: &mut HashMap<i64, ProtoOaOrder>,
    ssb_orders: &mut HashMap<i64, ProtoOaOrder>,
    bbs_positions: &mut HashMap<i64, ProtoOaPosition>,
    ssb_positions: &mut HashMap<i64, ProtoOaPosition>,
    bbs_label: &String,
    ssb_label: &String,
) {
    match incoming_message.execution_type() {
        ProtoOaExecutionType::OrderAccepted => match &incoming_message.order {
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
                            info!("ProtoOaExecutionType::OrderAccepted: add BBS {:#?}", &o);
                            bbs_orders.insert(o.order_id, o.clone());
                        }
                        if o.trade_data.label() == ssb_label {
                            info!("ProtoOaExecutionType::OrderAccepted: add SSB {:#?}", &o);
                            ssb_orders.insert(o.order_id, o.clone());
                        }
                    }
                }
            }
            None => {
                warn!("ProtoOaExecutionType::OrderAccepted");
            }
        },

        ProtoOaExecutionType::OrderFilled => {
            match &incoming_message.position {
                Some(p) => {
                    match p.position_status() {
                        ProtoOaPositionStatus::PositionStatusOpen => {
                            if p.trade_data.label() == bbs_label {
                                info!(
                                    "ProtoOaPositionStatus::PositionStatusOpen: add BBS {:#?}",
                                    &p
                                );
                                bbs_positions.insert(p.position_id, p.clone());
                            }
                            if p.trade_data.label() == ssb_label {
                                info!(
                                    "ProtoOaPositionStatus::PositionStatusOpen: add SSB {:#?}",
                                    &p
                                );
                                ssb_positions.insert(p.position_id, p.clone());
                            }

                            match &incoming_message.order {
                                Some(o) => {
                                    if bbs_orders.contains_key(&o.order_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusOpen: remove BBS {:#?}", &o);
                                        bbs_orders.remove(&o.order_id);
                                    }
                                    if ssb_orders.contains_key(&o.order_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusOpen: remove SSB {:#?}", &o);
                                        ssb_orders.remove(&o.order_id);
                                    }
                                }
                                None => {
                                    warn!("ProtoOaExecutionType::PositionStatusOpen");
                                }
                            }
                        }

                        ProtoOaPositionStatus::PositionStatusClosed => {
                            match &incoming_message.position {
                                Some(p) => {
                                    if bbs_positions.contains_key(&p.position_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusClosed: remove BBS {:#?}", &p);
                                        bbs_positions.remove(&p.position_id);
                                    }
                                    if ssb_positions.contains_key(&p.position_id) {
                                        info!("ProtoOaPositionStatus::PositionStatusClosed: remove SSB {:#?}", &p);
                                        ssb_positions.remove(&p.position_id);
                                    }
                                }
                                None => {
                                    warn!("ProtoOaExecutionType::PositionStatusClosed");
                                }
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
                                                info!("ProtoOaPositionStatus::PositionStatusCreated: add BBS {:#?}", &o);
                                                bbs_orders.insert(o.order_id, o.clone());
                                            }
                                            if o.trade_data.label() == ssb_label {
                                                info!("ProtoOaPositionStatus::PositionStatusCreated: add SSB {:#?}", &o);
                                                ssb_orders.insert(o.order_id, o.clone());
                                            }
                                        }
                                    }
                                }
                                None => {
                                    warn!("ProtoOaExecutionType::PositionStatusCreated");
                                }
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
                None => {
                    warn!("ProtoOaExecutionType::PositionStatusError");
                }
            }
        }

        ProtoOaExecutionType::OrderReplaced => match &incoming_message.order {
            Some(o) => {
                if o.trade_data.label() == bbs_label {
                    if bbs_orders.contains_key(&o.order_id) {
                        info!("ProtoOaExecutionType::OrderReplaced: remove BBS {:#?}", &o);
                        bbs_orders.remove(&o.order_id);
                    }
                    info!("ProtoOaExecutionType::OrderReplaced: add BBS {:#?}", &o);
                    bbs_orders.insert(o.order_id, o.clone());
                }
                if o.trade_data.label() == ssb_label {
                    if ssb_orders.contains_key(&o.order_id) {
                        info!("ProtoOaExecutionType::OrderReplaced: remove SSB {:#?}", &o);
                        ssb_orders.remove(&o.order_id);
                    }
                    info!("ProtoOaExecutionType::OrderReplaced: add SSB {:#?}", &o);
                    ssb_orders.insert(o.order_id, o.clone());
                }
            }
            None => {
                warn!("ProtoOaExecutionType::OrderReplaced");
            }
        },

        ProtoOaExecutionType::OrderCancelled => match &incoming_message.order {
            Some(o) => {
                if bbs_orders.contains_key(&o.order_id) {
                    info!("ProtoOaExecutionType::OrderCancelled: remove BBS {:#?}", &o);
                    bbs_orders.remove(&o.order_id);
                }
                if ssb_orders.contains_key(&o.order_id) {
                    info!("ProtoOaExecutionType::OrderCancelled: remove SSB {:#?}", &o);
                    ssb_orders.remove(&o.order_id);
                }
            }
            None => {
                warn!("ProtoOaExecutionType::OrderCancelled");
            }
        },

        ProtoOaExecutionType::OrderExpired => match &incoming_message.order {
            Some(o) => {
                if bbs_orders.contains_key(&o.order_id) {
                    info!("ProtoOaExecutionType::OrderExpired: remove BBS {:#?}", &o);
                    bbs_orders.remove(&o.order_id);
                }
                if ssb_orders.contains_key(&o.order_id) {
                    info!("ProtoOaExecutionType::OrderExpired: remove SSB {:#?}", &o);
                    ssb_orders.remove(&o.order_id);
                }
            }
            None => {
                warn!("ProtoOaExecutionType::OrderExpired");
            }
        },
        ProtoOaExecutionType::OrderRejected => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::OrderRejected: {:#?}", &o);
            }
            None => {
                warn!("ProtoOaExecutionType::OrderRejected");
            }
        },

        ProtoOaExecutionType::OrderCancelRejected => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::OrderCancelRejected: {:#?}", &o);
            }
            None => {
                warn!("ProtoOaExecutionType::OrderCancelRejected");
            }
        },

        ProtoOaExecutionType::Swap => match &incoming_message.order {
            Some(o) => {
                debug!("ProtoOaExecutionType::Swap: {:#?}", &o);
            }
            None => {
                warn!("ProtoOaExecutionType::Swap");
            }
        },

        ProtoOaExecutionType::DepositWithdraw => match &incoming_message.order {
            Some(o) => {
                debug!("ProtoOaExecutionType::DepositWithdraw: {:#?}", &o);
            }
            None => {
                warn!("ProtoOaExecutionType::DepositWithdraw");
            }
        },

        ProtoOaExecutionType::OrderPartialFill => match &incoming_message.order {
            Some(o) => {
                info!("ProtoOaExecutionType::OrderPartialFill: {:#?}", &o);
            }
            None => {
                warn!("ProtoOaExecutionType::OrderPartialFill");
            }
        },

        ProtoOaExecutionType::BonusDepositWithdraw => match &incoming_message.order {
            Some(o) => {
                debug!("ProtoOaExecutionType::BonusDepositWithdraw: {:#?}", &o);
            }
            None => {
                warn!("ProtoOaExecutionType::BonusDepositWithdraw");
            }
        },
    }
}
