use crate::ctrader_open_api::*;
use crate::encoder::*;
use crate::processors::*;
use crate::symbol::Symbol;

use prost::*;
use std::collections::HashMap;
use std::env;
use std::net::TcpStream;
use std::time::Duration;
use std::time::Instant;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message as message;
use tungstenite::WebSocket;

extern crate env_logger;
use log::{debug, error, info};

/*
let mut outgoing_messages_queue: Vec<ProtoMessage> = vec![];
outgoing_messages_queue.insert(0, outgoing_message);
outgoing_messages_queue.pop();
*/

pub fn start(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let client_id = env::var("CLIENT_ID").expect("$CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("$CLIENT_SECRET not set");

    let outgoing_message: ProtoOaApplicationAuthReq = ProtoOaApplicationAuthReq {
        payload_type: Some(2100),
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
    };

    match socket.send(message::from(encode_proto_message_to_byte_vector(
        outgoing_message.payload_type.unwrap() as u32,
        outgoing_message.encode_to_vec(),
    ))) {
        Ok(()) => {
            info!("{:#?}", &outgoing_message);
            decode_incoming_message(socket)
        }
        Err(e) => {
            error!("sending {:?} :: {:?}", &outgoing_message, &e);
        }
    }
}

fn decode_incoming_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let interval = Duration::from_secs(30);
    let mut next_time = Instant::now() + interval;

    let symbol_1_id: i64 = envmnt::get_i64("SYMBOL_1_ID", 0);
    let symbol_2_id: i64 = envmnt::get_i64("SYMBOL_2_ID", 0);
    let symbol_3_id: i64 = envmnt::get_i64("SYMBOL_3_ID", 0);

    let threshold: f64 = envmnt::get_f64("THRESHOLD", 0.0);

    let mut symbol_1: Symbol = Symbol {
        id: symbol_1_id,
        bid: 0.0,
        ask: 0.0,
        // spread: 0,
    };
    let mut symbol_2: Symbol = Symbol {
        id: symbol_2_id,
        bid: 0.0,
        ask: 0.0,
        // spread: 0,
    };
    let mut symbol_3: Symbol = Symbol {
        id: symbol_3_id,
        bid: 0.0,
        ask: 0.0,
        // spread: 0,
    };

    let mut positions: HashMap<i64, ProtoOaPosition> = HashMap::new();
    let mut orders: HashMap<i64, ProtoOaOrder> = HashMap::new();

    loop {
        match socket.read() {
            Ok(incoming_message) => {
                match ProtoMessage::decode(incoming_message.into_data().as_slice()) {
                    Ok(incoming_proto_message) => match incoming_proto_message.payload {
                        Some(incoming_message_payload_type) => match &incoming_proto_message
                            .payload_type
                        {
                            5 => {
                                info!("5");
                            }
                            50 => {
                                info!("50");
                            }
                            51 => match ProtoHeartbeatEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                    let outgoing_message: ProtoHeartbeatEvent =
                                        ProtoHeartbeatEvent {
                                            payload_type: Some(51),
                                        };
                                    match socket.send(message::from(
                                        encode_proto_message_to_byte_vector(
                                            outgoing_message.payload_type.unwrap() as u32,
                                            outgoing_message.encode_to_vec(),
                                        ),
                                    )) {
                                        Ok(()) => {
                                            info!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2101 => match ProtoOaApplicationAuthRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                    let access_token =
                                        env::var("ACCESS_TOKEN").expect("$ACCESS_TOKEN not set");
                                    let outgoing_message = ProtoOaGetAccountListByAccessTokenReq {
                                        payload_type: Some(2149),
                                        access_token: access_token.to_string(),
                                    };
                                    match socket.send(message::from(
                                        encode_proto_message_to_byte_vector(
                                            outgoing_message.payload_type.unwrap() as u32,
                                            outgoing_message.encode_to_vec(),
                                        ),
                                    )) {
                                        Ok(()) => {
                                            info!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2103 => match ProtoOaAccountAuthRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);

                                    let outgoing_message = ProtoOaReconcileReq {
                                        payload_type: Some(2124),
                                        ctid_trader_account_id: incoming_message
                                            .ctid_trader_account_id,
                                        return_protection_orders: Some(true),
                                    };
                                    match socket.send(message::from(
                                        encode_proto_message_to_byte_vector(
                                            outgoing_message.payload_type.unwrap() as u32,
                                            outgoing_message.encode_to_vec(),
                                        ),
                                    )) {
                                        Ok(..) => {
                                            info!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2105 => match ProtoOaVersionRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2107 => match ProtoOaTrailingSlChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2113 => match ProtoOaAssetListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2115 => match ProtoOaSymbolsListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2117 => match ProtoOaSymbolByIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2119 => match ProtoOaSymbolsForConversionRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2120 => match ProtoOaSymbolChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2122 => match ProtoOaTraderRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2123 => match ProtoOaTraderUpdatedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2125 => match ProtoOaReconcileRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                    reconcile_processor::process(
                                        &incoming_message,
                                        &mut positions,
                                        &mut orders,
                                    );

                                    let symbols_to_subscribe =
                                        vec![symbol_1_id, symbol_2_id, symbol_3_id];

                                    let outgoing_message: ProtoOaSubscribeSpotsReq =
                                        ProtoOaSubscribeSpotsReq {
                                            payload_type: Some(2127),
                                            ctid_trader_account_id: incoming_message
                                                .ctid_trader_account_id,
                                            symbol_id: symbols_to_subscribe,
                                            subscribe_to_spot_timestamp: Some(true),
                                        };

                                    match socket.send(message::from(
                                        encode_proto_message_to_byte_vector(
                                            outgoing_message.payload_type.unwrap() as u32,
                                            outgoing_message.encode_to_vec(),
                                        ),
                                    )) {
                                        Ok(..) => {
                                            info!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2126 => match ProtoOaExecutionEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                    execution_event_processor::process(
                                        &incoming_message,
                                        &mut positions,
                                        &mut orders,
                                    );
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2128 => match ProtoOaSubscribeSpotsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2130 => match ProtoOaUnsubscribeSpotsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2131 => match ProtoOaSpotEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    spot_event_processor::process(
                                        &incoming_message,
                                        &mut symbol_1,
                                        &mut symbol_2,
                                        &mut symbol_3,
                                        &threshold,
                                    );
                                    // info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2132 => match ProtoOaOrderErrorEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2134 => match ProtoOaDealListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2138 => match ProtoOaGetTrendbarsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2140 => match ProtoOaExpectedMarginRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2141 => match ProtoOaMarginChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2142 => match ProtoErrorRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                    // TODO: match all error codes
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2144 => match ProtoOaGetTickDataRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2146 => match ProtoOaMarginChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2147 => {
                                match ProtoOaAccountsTokenInvalidatedEvent::decode(
                                    incoming_message_payload_type.as_slice(),
                                ) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("decoding {:?}", e);
                                    }
                                }
                            }
                            2148 => match ProtoOaClientDisconnectEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2150 => {
                                match ProtoOaGetAccountListByAccessTokenRes::decode(
                                    incoming_message_payload_type.as_slice(),
                                ) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message);
                                        let access_token = env::var("ACCESS_TOKEN")
                                            .expect("$ACCESS_TOKEN not set");

                                        for account in &incoming_message.ctid_trader_account {
                                            let outgoing_message = ProtoOaAccountAuthReq {
                                                payload_type: Some(2102),
                                                ctid_trader_account_id: account
                                                    .ctid_trader_account_id
                                                    as i64,
                                                access_token: access_token.to_string(),
                                            };

                                            match socket.send(message::from(
                                                encode_proto_message_to_byte_vector(
                                                    outgoing_message.payload_type.unwrap() as u32,
                                                    outgoing_message.encode_to_vec(),
                                                ),
                                            )) {
                                                Ok(()) => {
                                                    info!("{:#?}", &outgoing_message);
                                                }
                                                Err(e) => {
                                                    error!(
                                                        "sending {:?} :: {:?}",
                                                        &outgoing_message, &e
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("decoding {:?}", e);
                                    }
                                }
                            }
                            2152 => match ProtoOaGetCtidProfileByTokenRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2154 => match ProtoOaAssetClassListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2155 => match ProtoOaDepthEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2157 => match ProtoOaSubscribeDepthQuotesRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2159 => match ProtoOaUnsubscribeDepthQuotesRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2161 => match ProtoOaSymbolCategoryListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2163 => match ProtoOaAccountLogoutRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2164 => match ProtoOaAccountDisconnectEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2165 => match ProtoOaSubscribeLiveTrendbarRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2166 => match ProtoOaUnsubscribeLiveTrendbarRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2168 => match ProtoOaMarginCallListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2170 => match ProtoOaMarginCallUpdateRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2171 => match ProtoOaMarginCallUpdateEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2172 => match ProtoOaMarginCallTriggerEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2174 => match ProtoOaRefreshTokenRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2176 => match ProtoOaOrderListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2178 => match ProtoOaGetDynamicLeverageByIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2180 => match ProtoOaDealListByPositionIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2182 => match ProtoOaOrderDetailsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2184 => match ProtoOaOrderListByPositionIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2186 => match ProtoOaDealOffsetListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    info!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("decoding {:?}", e);
                                }
                            },
                            2188 => {
                                match ProtoOaGetPositionUnrealizedPnLRes::decode(
                                    incoming_message_payload_type.as_slice(),
                                ) {
                                    Ok(incoming_message) => {
                                        info!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("decoding {:?}", e);
                                    }
                                }
                            }
                            _ => {
                                error!(
                                    "API Client does not yet cover: {:#?}",
                                    &incoming_proto_message.payload_type
                                )
                            }
                        },
                        None => {
                            error!("uh-oh, something went wrong here... :(")
                        }
                    },
                    Err(e) => {
                        error!("decoding {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
        if Instant::now() > next_time {
            let outgoing_message: ProtoHeartbeatEvent = ProtoHeartbeatEvent {
                payload_type: Some(51),
            };
            match socket.send(message::from(encode_proto_message_to_byte_vector(
                outgoing_message.payload_type.unwrap() as u32,
                outgoing_message.encode_to_vec(),
            ))) {
                Ok(()) => {
                    debug!("{:#?}", &outgoing_message);
                }
                Err(e) => {
                    error!("sending {:?} :: {:?}", &outgoing_message, &e);
                }
            }
            next_time += interval;
        }
    }
}
