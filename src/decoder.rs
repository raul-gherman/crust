use crate::ctrader_open_api::*;
use crate::processors::*;
use crate::sender::send;
use crate::symbol::Symbol;

use log::debug;
use prost::Message;
use std::collections::HashMap;
use std::env;
use std::net::TcpStream;
use std::process::exit;
use std::time::Duration;
use std::time::Instant;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;

extern crate env_logger;
use log::{error, info};

pub fn decode_incoming_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let interval = Duration::from_secs(30);
    let mut next_time = Instant::now() + interval;

    let client_id = envmnt::get_or("CLIENT_ID", "NOT_SET");
    let client_secret = envmnt::get_or("CLIENT_SECRET", "NOT_SET");

    let symbol_1_id: i64 = envmnt::get_i64("SYMBOL_1_ID", 0);
    let symbol_2_id: i64 = envmnt::get_i64("SYMBOL_2_ID", 0);
    let symbol_3_id: i64 = envmnt::get_i64("SYMBOL_3_ID", 0);

    let threshold: f64 = envmnt::get_f64("THRESHOLD", 0.0);

    let fix_volume: i64 = envmnt::get_i64("FIX_VOLUME", 0);

    let mut symbol_1: Symbol = Symbol {
        id: symbol_1_id,
        bid: 0.0,
        ask: 0.0,
    };
    let mut symbol_2: Symbol = Symbol {
        id: symbol_2_id,
        bid: 0.0,
        ask: 0.0,
    };
    let mut symbol_3: Symbol = Symbol {
        id: symbol_3_id,
        bid: 0.0,
        ask: 0.0,
    };

    let mut bbs_positions: HashMap<i64, ProtoOaPosition> = HashMap::new();
    let mut ssb_positions: HashMap<i64, ProtoOaPosition> = HashMap::new();

    let mut bbs_orders: HashMap<i64, ProtoOaOrder> = HashMap::new();
    let mut ssb_orders: HashMap<i64, ProtoOaOrder> = HashMap::new();

    let bbs_label = format!("112={}-{}-{}", symbol_1_id, symbol_2_id, symbol_3_id);
    let ssb_label = format!("221={}-{}-{}", symbol_1_id, symbol_2_id, symbol_3_id);

    let access_token = env::var("ACCESS_TOKEN").expect("$ACCESS_TOKEN not set");

    loop {
        match socket.read() {
            Ok(tungstenite_envelope) => {
                match ProtoMessage::decode(tungstenite_envelope.into_data().as_slice()) {
                    Ok(incoming_proto_message) => match incoming_proto_message.payload {
                        Some(incoming_message_payload) => {
                            let buf = incoming_message_payload.as_slice();
                            match &incoming_proto_message.payload_type {
                                5 => {
                                    info!("5");
                                }
                                50 => match ProtoErrorRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoErrorRes: {:#?}", e);
                                    }
                                },
                                51 => match ProtoHeartbeatEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);

                                        let outgoing_message: ProtoHeartbeatEvent =
                                            ProtoHeartbeatEvent {
                                                payload_type: Some(51),
                                            };
                                        info!("{:#?}", &outgoing_message);
                                        send(socket, 51, outgoing_message.encode_to_vec());
                                    }
                                    Err(e) => {
                                        error!("ProtoHeartbeatEvent: {:#?}", e);
                                    }
                                },
                                2101 => match ProtoOaApplicationAuthRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);

                                        let outgoing_message =
                                            ProtoOaGetAccountListByAccessTokenReq {
                                                payload_type: Some(2149),
                                                access_token: access_token.to_string(),
                                            };
                                        info!("{:#?}", &outgoing_message);
                                        send(socket, 2149, outgoing_message.encode_to_vec());
                                    }
                                    Err(e) => {
                                        error!("ProtoOaApplicationAuthRes: {:#?}", e);
                                    }
                                },
                                2103 => match ProtoOaAccountAuthRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);

                                        let outgoing_message = ProtoOaReconcileReq {
                                            payload_type: Some(2124),
                                            ctid_trader_account_id: incoming_message
                                                .ctid_trader_account_id,
                                            return_protection_orders: Some(false),
                                        };
                                        info!("{:#?}", &outgoing_message);
                                        send(socket, 2124, outgoing_message.encode_to_vec());
                                    }
                                    Err(e) => {
                                        error!("ProtoOaAccountAuthRes: {:#?}", e);
                                    }
                                },
                                2105 => match ProtoOaVersionRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaVersionRes: {:#?}", e);
                                    }
                                },
                                2107 => match ProtoOaTrailingSlChangedEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaTrailingSlChangedEvent: {:#?}", e);
                                    }
                                },
                                2113 => match ProtoOaAssetListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaAssetListRes: {:#?}", e);
                                    }
                                },
                                2115 => match ProtoOaSymbolsListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSymbolsListRes: {:#?}", e);
                                    }
                                },
                                2117 => match ProtoOaSymbolByIdRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSymbolByIdRes: {:#?}", e);
                                    }
                                },
                                2119 => match ProtoOaSymbolsForConversionRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSymbolsForConversionRes: {:#?}", e);
                                    }
                                },
                                2120 => match ProtoOaSymbolChangedEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSymbolChangedEvent: {:#?}", e);
                                    }
                                },
                                2122 => match ProtoOaTraderRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaTraderRes: {:#?}", e);
                                    }
                                },
                                2123 => match ProtoOaTraderUpdatedEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaTraderUpdatedEvent: {:#?}", e);
                                    }
                                },
                                2125 => match ProtoOaReconcileRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        debug!("{:#?}", &incoming_message);

                                        reconcile_processor::process(
                                            &incoming_message,
                                            &mut bbs_orders,
                                            &mut ssb_orders,
                                            &mut bbs_positions,
                                            &mut ssb_positions,
                                            &bbs_label,
                                            &ssb_label,
                                        );

                                        let symbols_to_subscribe =
                                            vec![symbol_1_id, symbol_2_id, symbol_3_id];

                                        let outgoing_message: ProtoOaSubscribeSpotsReq =
                                            ProtoOaSubscribeSpotsReq {
                                                payload_type: Some(2127),
                                                ctid_trader_account_id: incoming_message
                                                    .ctid_trader_account_id,
                                                symbol_id: symbols_to_subscribe,
                                                subscribe_to_spot_timestamp: Some(false),
                                            };
                                        info!("{:#?}", &outgoing_message);
                                        send(socket, 2127, outgoing_message.encode_to_vec());
                                    }
                                    Err(e) => {
                                        error!("ProtoOaReconcileRes: {:#?}", e);
                                    }
                                },
                                2126 => match ProtoOaExecutionEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        execution_event_processor::process(
                                            &incoming_message,
                                            &mut bbs_orders,
                                            &mut ssb_orders,
                                            &mut bbs_positions,
                                            &mut ssb_positions,
                                            &bbs_label,
                                            &ssb_label,
                                        );
                                    }
                                    Err(e) => {
                                        error!("ProtoOaExecutionEvent: {:#?}", e);
                                    }
                                },
                                2128 => match ProtoOaSubscribeSpotsRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSubscribeSpotsRes: {:#?}", e);
                                    }
                                },
                                2130 => match ProtoOaUnsubscribeSpotsRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaUnsubscribeSpotsRes: {:#?}", e);
                                    }
                                },
                                2131 => match ProtoOaSpotEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        debug!("{:#?}", &incoming_message);
                                        spot_event_processor::process(
                                            socket,
                                            &incoming_message,
                                            &mut symbol_1,
                                            &mut symbol_2,
                                            &mut symbol_3,
                                            &threshold,
                                            fix_volume,
                                            &bbs_orders,
                                            &ssb_orders,
                                            &bbs_positions,
                                            &ssb_positions,
                                            &bbs_label,
                                            &ssb_label,
                                        );
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSpotEvent: {:#?}", e);
                                    }
                                },
                                2132 => match ProtoOaOrderErrorEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaOrderErrorEvent: {:#?}", e);
                                    }
                                },
                                2134 => match ProtoOaDealListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaDealListRes: {:#?}", e);
                                    }
                                },
                                2138 => match ProtoOaGetTrendbarsRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaGetTrendbarsRes: {:#?}", e);
                                    }
                                },
                                2140 => match ProtoOaExpectedMarginRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                    }
                                    Err(e) => {
                                        error!("ProtoOaExpectedMarginRes: {:#?}", e);
                                    }
                                },
                                2141 => match ProtoOaMarginChangedEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaMarginChangedEvent: {:#?}", e);
                                    }
                                },
                                2142 => {
                                    match ProtoErrorRes::decode(buf) {
                                        Ok(incoming_message) => {
                                            info!("{:#?}", &incoming_message)
                                            // TODO: match all error codes
                                        }
                                        Err(e) => {
                                            error!("ProtoErrorRes: {:#?}", e);
                                        }
                                    }
                                }
                                2144 => match ProtoOaGetTickDataRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaGetTickDataRes: {:#?}", e);
                                    }
                                },
                                2146 => match ProtoOaMarginChangedEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaMarginChangedEvent: {:#?}", e);
                                    }
                                },
                                2147 => match ProtoOaAccountsTokenInvalidatedEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaAccountsTokenInvalidatedEvent: {:#?}", e);
                                    }
                                },
                                2148 => match ProtoOaClientDisconnectEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaClientDisconnectEvent: {:#?}", e);
                                    }
                                },
                                2150 => match ProtoOaGetAccountListByAccessTokenRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);

                                        for account in &incoming_message.ctid_trader_account {
                                            let outgoing_message = ProtoOaAccountAuthReq {
                                                payload_type: Some(2102),
                                                ctid_trader_account_id: account
                                                    .ctid_trader_account_id
                                                    as i64,
                                                access_token: access_token.to_string(),
                                            };

                                            info!("{:#?}", &outgoing_message);
                                            send(socket, 2102, outgoing_message.encode_to_vec());
                                        }
                                    }
                                    Err(e) => {
                                        error!("ProtoOaGetAccountListByAccessTokenRes: {:#?}", e);
                                    }
                                },
                                2152 => match ProtoOaGetCtidProfileByTokenRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!(
                                            "decoding ProtoOaGetCtidProfileByTokenRes: {:#?}",
                                            e
                                        );
                                    }
                                },
                                2154 => match ProtoOaAssetClassListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaAssetClassListRes: {:#?}", e);
                                    }
                                },
                                2155 => match ProtoOaDepthEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaDepthEvent: {:#?}", e);
                                    }
                                },
                                2157 => match ProtoOaSubscribeDepthQuotesRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSubscribeDepthQuotesRes: {:#?}", e);
                                    }
                                },
                                2159 => match ProtoOaUnsubscribeDepthQuotesRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!(
                                            "decoding ProtoOaUnsubscribeDepthQuotesRes: {:#?}",
                                            e
                                        );
                                    }
                                },
                                2161 => match ProtoOaSymbolCategoryListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaSymbolCategoryListRes: {:#?}", e);
                                    }
                                },
                                2163 => match ProtoOaAccountLogoutRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaAccountLogoutRes: {:#?}", e);
                                    }
                                },
                                2164 => match ProtoOaAccountDisconnectEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                        let outgoing_message = ProtoOaApplicationAuthReq {
                                            payload_type: Some(2100),
                                            client_id: client_id.to_string(),
                                            client_secret: client_secret.to_string(),
                                        };
                                        info!("{:#?}", &outgoing_message);

                                        send(socket, 2100, outgoing_message.encode_to_vec());
                                    }
                                    Err(e) => {
                                        error!("ProtoOaAccountDisconnectEvent: {:#?}", e);
                                    }
                                },
                                2165 => match ProtoOaSubscribeLiveTrendbarRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!(
                                            "decoding ProtoOaSubscribeLiveTrendbarRes: {:#?}",
                                            e
                                        );
                                    }
                                },
                                2166 => match ProtoOaUnsubscribeLiveTrendbarRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!(
                                            "decoding ProtoOaUnsubscribeLiveTrendbarRes: {:#?}",
                                            e
                                        );
                                    }
                                },
                                2168 => match ProtoOaMarginCallListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaMarginCallListRes: {:#?}", e);
                                    }
                                },
                                2170 => match ProtoOaMarginCallUpdateRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaMarginCallUpdateRes: {:#?}", e);
                                    }
                                },
                                2171 => match ProtoOaMarginCallUpdateEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaMarginCallUpdateEvent: {:#?}", e);
                                    }
                                },
                                2172 => match ProtoOaMarginCallTriggerEvent::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaMarginCallTriggerEvent: {:#?}", e);
                                    }
                                },
                                2174 => match ProtoOaRefreshTokenRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaRefreshTokenRes: {:#?}", e);
                                    }
                                },
                                2176 => match ProtoOaOrderListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaOrderListRes: {:#?}", e);
                                    }
                                },
                                2178 => match ProtoOaGetDynamicLeverageByIdRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!(
                                            "decoding ProtoOaGetDynamicLeverageByIdRes: {:#?}",
                                            e
                                        );
                                    }
                                },
                                2180 => match ProtoOaDealListByPositionIdRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaDealListByPositionIdRes: {:#?}", e);
                                    }
                                },
                                2182 => match ProtoOaOrderDetailsRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaOrderDetailsRes: {:#?}", e);
                                    }
                                },
                                2184 => match ProtoOaOrderListByPositionIdRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!(
                                            "decoding ProtoOaOrderListByPositionIdRes: {:#?}",
                                            e
                                        );
                                    }
                                },
                                2186 => match ProtoOaDealOffsetListRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaDealOffsetListRes: {:#?}", e);
                                    }
                                },
                                2188 => match ProtoOaGetPositionUnrealizedPnLRes::decode(buf) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("ProtoOaGetPositionUnrealizedPnLRes: {:#?}", e);
                                    }
                                },
                                _ => {
                                    error!(
                                        "API Client does not yet cover: {:#?}",
                                        &incoming_proto_message.payload_type
                                    )
                                }
                            }
                        }
                        None => {
                            error!("uh-oh, something went wrong here... :(")
                        }
                    },
                    Err(e) => {
                        error!("ProtoMessage::decode {:#?}", e);
                    }
                }
            }
            Err(e) => {
                error!("socket.read :{:#?}", e);
                if !socket.can_read() || !socket.can_write() {
                    match socket.close(None) {
                        Ok(_) => {
                            info!("socket.close");
                            exit(1)
                        }
                        Err(e) => {
                            error!("socket.close: {:#?}", e);
                        }
                    }
                }
            }
        }
        if Instant::now() > next_time {
            let outgoing_message: ProtoHeartbeatEvent = ProtoHeartbeatEvent {
                payload_type: Some(51),
            };
            debug!("recursive: {:#?}", &outgoing_message);
            send(socket, 51, outgoing_message.encode_to_vec());

            next_time += interval;
        }
    }
}
