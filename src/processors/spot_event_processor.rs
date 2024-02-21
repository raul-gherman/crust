use std::{collections::HashMap, net::TcpStream};

use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

use log::info;

use crate::{
    ctrader_open_api::{ProtoOaOrder, ProtoOaPosition, ProtoOaSpotEvent},
    engagement::{close, open},
    symbol::Symbol,
};

use crate::sender::send;

pub fn process(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    incoming_message: &ProtoOaSpotEvent,
    symbol_1: &mut Symbol,
    symbol_2: &mut Symbol,
    symbol_3: &mut Symbol,
    threshold: &f64,
    fix_volume: i64,
    bbs_orders: &HashMap<i64, ProtoOaOrder>,
    ssb_orders: &HashMap<i64, ProtoOaOrder>,
    bbs_positions: &HashMap<i64, ProtoOaPosition>,
    ssb_positions: &HashMap<i64, ProtoOaPosition>,
    bbs_label: &String,
    ssb_label: &String,
) {
    if incoming_message.symbol_id != symbol_1.id
        && incoming_message.symbol_id != symbol_2.id
        && incoming_message.symbol_id != symbol_3.id
    {
        return;
    }

    if incoming_message.symbol_id == symbol_1.id {
        if incoming_message.ask() > 0 {
            symbol_1.ask = incoming_message.ask() as f64
        };
        if incoming_message.bid() > 0 {
            symbol_1.bid = incoming_message.bid() as f64
        };
    }

    if incoming_message.symbol_id == symbol_2.id {
        if incoming_message.ask() > 0 {
            symbol_2.ask = incoming_message.ask() as f64
        };
        if incoming_message.bid() > 0 {
            symbol_2.bid = incoming_message.bid() as f64
        };
    }

    if incoming_message.symbol_id == symbol_3.id {
        if incoming_message.ask() > 0 {
            symbol_3.ask = incoming_message.ask() as f64
        };
        if incoming_message.bid() > 0 {
            symbol_3.bid = incoming_message.bid() as f64
        };
    }

    if symbol_1.ask > 0.0
        && symbol_1.bid > 0.0
        && symbol_2.ask > 0.0
        && symbol_2.bid > 0.0
        && symbol_3.ask > 0.0
        && symbol_3.bid > 0.0
    {
        let bbs = (symbol_1.ask * symbol_2.ask / symbol_3.bid) - 100000.0;
        let ssb = 100000.0 - ((symbol_1.bid * symbol_2.bid) / symbol_3.ask);

        if bbs <= 0.0 {
            if ssb_positions.len() + ssb_orders.len() > 0 {
                info!(
                    "BBS: {} | symbol_1.ask: {} | symbol_2.ask: {} | symbol_3.bid: {}",
                    &bbs, &symbol_1.ask, &symbol_2.ask, &symbol_3.bid
                );
                for (key, value) in ssb_positions {
                    if value.trade_data.label() == ssb_label {
                        send(
                            socket,
                            2111,
                            close::position(
                                incoming_message.ctid_trader_account_id,
                                *key,
                                value.trade_data.volume,
                            ),
                        );
                    }
                }
                for (key, value) in ssb_orders {
                    if value.trade_data.label() == ssb_label {
                        send(
                            socket,
                            2108,
                            close::pending_order(incoming_message.ctid_trader_account_id, *key),
                        );
                    }
                }
            }

            if bbs < *threshold {
                let variable_volume =
                    ((((symbol_1.ask * fix_volume as f64) / 100_000.0) / 100_000.0).round())
                        * 100_000.0;

                if bbs_positions.len() + bbs_orders.len() == 0 {
                    send(
                        socket,
                        2106,
                        open::buy(
                            incoming_message.ctid_trader_account_id,
                            symbol_1,
                            fix_volume,
                            bbs.to_string(),
                            bbs_label,
                        ),
                    );
                    send(
                        socket,
                        2106,
                        open::buy(
                            incoming_message.ctid_trader_account_id,
                            symbol_2,
                            variable_volume as i64,
                            bbs.to_string(),
                            bbs_label,
                        ),
                    );
                    send(
                        socket,
                        2106,
                        open::sell(
                            incoming_message.ctid_trader_account_id,
                            symbol_3,
                            fix_volume,
                            bbs.to_string(),
                            bbs_label,
                        ),
                    );
                }
            }
        }

        if ssb <= 0.0 {
            if bbs_positions.len() + bbs_orders.len() > 0 {
                info!(
                    "SSB: {} | symbol_1.bid: {} | symbol_2.bid: {} | symbol_3.ask: {}",
                    &ssb, &symbol_1.bid, &symbol_2.bid, &symbol_3.ask
                );
                for (key, value) in bbs_positions {
                    if value.trade_data.label() == bbs_label {
                        send(
                            socket,
                            2111,
                            close::position(
                                incoming_message.ctid_trader_account_id,
                                *key,
                                value.trade_data.volume,
                            ),
                        );
                    }
                }
                for (key, value) in bbs_orders {
                    if value.trade_data.label() == bbs_label {
                        send(
                            socket,
                            2108,
                            close::pending_order(incoming_message.ctid_trader_account_id, *key),
                        );
                    }
                }
            }

            if ssb < *threshold {
                let variable_volume =
                    ((((symbol_1.bid * fix_volume as f64) / 100_000.0) / 100_000.0).round())
                        * 100_000.0;

                if ssb_positions.len() + ssb_orders.len() == 0 {
                    send(
                        socket,
                        2106,
                        open::sell(
                            incoming_message.ctid_trader_account_id,
                            symbol_1,
                            fix_volume,
                            ssb.to_string(),
                            ssb_label,
                        ),
                    );
                    send(
                        socket,
                        2106,
                        open::sell(
                            incoming_message.ctid_trader_account_id,
                            symbol_2,
                            variable_volume as i64,
                            ssb.to_string(),
                            ssb_label,
                        ),
                    );
                    send(
                        socket,
                        2106,
                        open::buy(
                            incoming_message.ctid_trader_account_id,
                            symbol_3,
                            fix_volume,
                            ssb.to_string(),
                            ssb_label,
                        ),
                    );
                }
            }
        }
    }
}
