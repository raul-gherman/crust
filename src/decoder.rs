use crate::ctrader_open_api::*;
use crate::encoder::*;

use log::warn;
use prost::*;
use std::env;
use std::net::TcpStream;
use std::time::Duration;
use std::time::Instant;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message as message;
use tungstenite::WebSocket;

extern crate env_logger;
use log::{debug, error};

/*
let mut outgoing_messages_queue: Vec<ProtoMessage> = vec![];
outgoing_messages_queue.insert(0, outgoing_message);
outgoing_messages_queue.pop();
*/

pub fn start(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let client_id = env::var("CLIENT_ID").expect("$CLIENT_ID is not set");
    let client_secret = env::var("CLIENT_SECRET").expect("$CLIENT_SECRET is not set");

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
            debug!("{:#?}", &outgoing_message);
            decode_incoming_message(socket)
        }
        Err(e) => {
            error!("sending {:?} :: {:?}", &outgoing_message, &e);
        }
    }
}

pub fn decode_incoming_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let interval = Duration::from_secs(30);
    let mut next_time = Instant::now() + interval;
    loop {
        match socket.read() {
            Ok(incoming_message) => {
                match ProtoMessage::decode(incoming_message.into_data().as_slice()) {
                    Ok(incoming_proto_message) => match incoming_proto_message.payload {
                        Some(incoming_message_payload_type) => match &incoming_proto_message
                            .payload_type
                        {
                            5 => {
                                debug!("5");
                            }
                            50 => {
                                debug!("50");
                            }
                            51 => match ProtoHeartbeatEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
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
                                            debug!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2101 => match ProtoOaApplicationAuthRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                    let access_token =
                                        env::var("ACCESS_TOKEN").expect("$ACCESS_TOKEN is not set");
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
                                            debug!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => debug!("{e:#?}"),
                            },
                            2103 => match ProtoOaAccountAuthRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);

                                    let outgoing_message: ProtoOaReconcileReq =
                                        ProtoOaReconcileReq {
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
                                            debug!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2105 => match ProtoOaVersionRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2107 => match ProtoOaTrailingSlChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2113 => match ProtoOaAssetListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2115 => match ProtoOaSymbolsListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2117 => match ProtoOaSymbolByIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2119 => match ProtoOaSymbolsForConversionRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2120 => match ProtoOaSymbolChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2122 => match ProtoOaTraderRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2123 => match ProtoOaTraderUpdatedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2125 => match ProtoOaReconcileRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                    for position in &incoming_message.position {
                                        debug!("{:#?}", &position);
                                    }

                                    let symbols_to_subscribe = vec![1, 2, 3];

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
                                            debug!("{:#?}", &outgoing_message);
                                        }
                                        Err(e) => {
                                            error!("sending {:?} :: {:?}", &outgoing_message, &e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2126 => match ProtoOaExecutionEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2128 => match ProtoOaSubscribeSpotsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2130 => match ProtoOaUnsubscribeSpotsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2131 => match ProtoOaSpotEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    //debug!("{:#?}", &incoming_message);
                                    // TODO: process spot event
                                    process_spot_event(&incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2132 => match ProtoOaOrderErrorEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2134 => match ProtoOaDealListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2138 => match ProtoOaGetTrendbarsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2140 => match ProtoOaExpectedMarginRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message);
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2141 => match ProtoOaMarginChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2142 => match ProtoErrorRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                    // todo: match all error codes
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2144 => match ProtoOaGetTickDataRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2146 => match ProtoOaMarginChangedEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2147 => {
                                match ProtoOaAccountsTokenInvalidatedEvent::decode(
                                    incoming_message_payload_type.as_slice(),
                                ) {
                                    Ok(incoming_message) => {
                                        debug!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("{:?}", e);
                                    }
                                }
                            }
                            2148 => match ProtoOaClientDisconnectEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2150 => {
                                match ProtoOaGetAccountListByAccessTokenRes::decode(
                                    incoming_message_payload_type.as_slice(),
                                ) {
                                    Ok(incoming_message) => {
                                        debug!("{:#?}", &incoming_message);
                                        let access_token = env::var("ACCESS_TOKEN")
                                            .expect("$ACCESS_TOKEN is not set");

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
                                                    debug!("{:#?}", &outgoing_message);
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
                                        error!("{:?}", e);
                                    }
                                }
                            }
                            2152 => match ProtoOaGetCtidProfileByTokenRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2154 => match ProtoOaAssetClassListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2155 => match ProtoOaDepthEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2157 => match ProtoOaSubscribeDepthQuotesRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2159 => match ProtoOaUnsubscribeDepthQuotesRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2161 => match ProtoOaSymbolCategoryListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2163 => match ProtoOaAccountLogoutRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2164 => match ProtoOaAccountDisconnectEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2165 => match ProtoOaSubscribeLiveTrendbarRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2166 => match ProtoOaUnsubscribeLiveTrendbarRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2168 => match ProtoOaMarginCallListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2170 => match ProtoOaMarginCallUpdateRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2171 => match ProtoOaMarginCallUpdateEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2172 => match ProtoOaMarginCallTriggerEvent::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2174 => match ProtoOaRefreshTokenRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2176 => match ProtoOaOrderListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2178 => match ProtoOaGetDynamicLeverageByIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2180 => match ProtoOaDealListByPositionIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2182 => match ProtoOaOrderDetailsRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2184 => match ProtoOaOrderListByPositionIdRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2186 => match ProtoOaDealOffsetListRes::decode(
                                incoming_message_payload_type.as_slice(),
                            ) {
                                Ok(incoming_message) => {
                                    debug!("{:#?}", &incoming_message)
                                }
                                Err(e) => {
                                    error!("{:?}", e);
                                }
                            },
                            2188 => {
                                match ProtoOaGetPositionUnrealizedPnLRes::decode(
                                    incoming_message_payload_type.as_slice(),
                                ) {
                                    Ok(incoming_message) => {
                                        debug!("{:#?}", &incoming_message)
                                    }
                                    Err(e) => {
                                        error!("{:?}", e);
                                    }
                                }
                            }
                            _ => {
                                debug!(
                                    "not covered yet: {:#?}",
                                    &incoming_proto_message.payload_type
                                )
                            }
                        },
                        None => {
                            error!("uh-oh, something went wrong here... :(")
                        }
                    },
                    Err(e) => {
                        error!("{:?}", e);
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

fn process_spot_event(spot_event: &ProtoOaSpotEvent) {
    match spot_event.symbol_id {
        1 | 2 | 3 => {
            debug!("{:#?}", spot_event);
        }
        _ => {
            warn!("process_spot_event :: {:#?}", spot_event);
        }
    }
}
