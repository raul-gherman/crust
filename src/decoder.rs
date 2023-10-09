use crate::ctrader_open_api::*;
use prost::*;
use std::net::TcpStream;
use std::thread;
// use std::thread::spawn;
// use chrono::{DateTime, Local};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::Message as message;
use tungstenite::WebSocket;

const ACCESS_TOKEN: &str = "gbAKM7bBXoyiP6I3S-mA2_UwyBD-zklsinmrZaT0w1Y";
const CLIENT_ID: &str = "4460_EpyxopCGdxPprbux9hTDaDtaDhfv4T2G0yF1OrnIg7LGYkAXU1";
const CLIENT_SECRET: &str = "C1lpWCK45J9sOTJ0or5b4GBHcxX5Jbt4erLqwOpB3RP8it4Iy0";

//let mut outgoing_messages_queue: Vec<ProtoMessage> = vec![];
//outgoing_messages_queue.insert(0, outgoing_message);
//outgoing_messages_queue.pop();

pub fn start(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    let outgoing_message: ProtoOaApplicationAuthReq = ProtoOaApplicationAuthReq {
        payload_type: Some(2100),
        client_id: CLIENT_ID.to_string(),
        client_secret: CLIENT_SECRET.to_string(),
    };

    match socket.send(message::from(encode_proto_message_to_byte_vector(
        outgoing_message.payload_type.unwrap() as u32,
        outgoing_message.encode_to_vec(),
    ))) {
        Ok(()) => {
            println!("{:#?}", &outgoing_message);
            decode_incoming_message(socket)
        }
        Err(e) => println!("{:#?}: {:#?}", &outgoing_message, &e),
    }
}

pub fn decode_incoming_message(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    loop {
        match socket.read() {
            Ok(m) => match ProtoMessage::decode(m.into_data().as_slice()) {
                Ok(incoming_proto_message) => match incoming_proto_message.payload {
                    Some(x) => match &incoming_proto_message.payload_type {
                        5 => println!("5"),
                        50 => println!("50"),
                        51 => match ProtoHeartbeatEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!(
                                    "{:#?} {:#?}",
                                    chrono::offset::Local::now(),
                                    &incoming_message
                                );
                                let outgoing_message: ProtoHeartbeatEvent = ProtoHeartbeatEvent {
                                    payload_type: Some(51),
                                };
                                match socket.send(message::from(
                                    encode_proto_message_to_byte_vector(
                                        outgoing_message.payload_type.unwrap() as u32,
                                        outgoing_message.encode_to_vec(),
                                    ),
                                )) {
                                    Ok(()) => {
                                        println!(
                                            "{:#?} {:#?}",
                                            chrono::offset::Local::now(),
                                            &outgoing_message
                                        )
                                    }
                                    Err(e) => println!("{:#?}: {:#?}", &outgoing_message, &e),
                                }
                            }
                            Err(e) => println!("{:#?}", &e),
                        },
                        2101 => match ProtoOaApplicationAuthRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                                let outgoing_message = ProtoOaGetAccountListByAccessTokenReq {
                                    payload_type: Some(2149),
                                    access_token: ACCESS_TOKEN.to_string(),
                                };
                                match socket.send(message::from(
                                    encode_proto_message_to_byte_vector(
                                        outgoing_message.payload_type.unwrap() as u32,
                                        outgoing_message.encode_to_vec(),
                                    ),
                                )) {
                                    Ok(()) => println!("{:#?}", &outgoing_message),
                                    Err(e) => println!("{:#?}: {:#?}", &outgoing_message, &e),
                                }
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2103 => match ProtoOaAccountAuthRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);

                                let outgoing_message: ProtoOaReconcileReq = ProtoOaReconcileReq {
                                    payload_type: Some(2124),
                                    ctid_trader_account_id: incoming_message.ctid_trader_account_id,
                                    return_protection_orders: Some(true),
                                };
                                match socket.send(message::from(
                                    encode_proto_message_to_byte_vector(
                                        outgoing_message.payload_type.unwrap() as u32,
                                        outgoing_message.encode_to_vec(),
                                    ),
                                )) {
                                    Ok(..) => println!("{:#?}", &outgoing_message),
                                    Err(e) => println!("{:#?}: {:#?}", &outgoing_message, &e),
                                }
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2105 => match ProtoOaVersionRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2107 => match ProtoOaTrailingSlChangedEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2113 => match ProtoOaAssetListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2115 => match ProtoOaSymbolsListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2117 => match ProtoOaSymbolByIdRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2119 => match ProtoOaSymbolsForConversionRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2120 => match ProtoOaSymbolChangedEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2122 => match ProtoOaTraderRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2123 => match ProtoOaTraderUpdatedEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2125 => match ProtoOaReconcileRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                                for _position in &incoming_message.position {
                                    //println!("{:#?}", &position);
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
                                    Ok(..) => println!("{:#?}", &outgoing_message),
                                    Err(e) => println!("{:#?}: {:#?}", &outgoing_message, &e),
                                }
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2126 => match ProtoOaExecutionEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2128 => match ProtoOaSubscribeSpotsRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2130 => match ProtoOaUnsubscribeSpotsRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2131 => match ProtoOaSpotEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                                // TODO: process spot event
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2132 => match ProtoOaOrderErrorEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2134 => match ProtoOaDealListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2138 => match ProtoOaGetTrendbarsRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2140 => match ProtoOaExpectedMarginRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2141 => match ProtoOaMarginChangedEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2142 => match ProtoErrorRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                                // todo: match all error codes
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2144 => match ProtoOaGetTickDataRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2146 => match ProtoOaMarginChangedEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2147 => match ProtoOaAccountsTokenInvalidatedEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2148 => match ProtoOaClientDisconnectEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2150 => match ProtoOaGetAccountListByAccessTokenRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message);

                                for account in &incoming_message.ctid_trader_account {
                                    let outgoing_message = ProtoOaAccountAuthReq {
                                        payload_type: Some(2102),
                                        ctid_trader_account_id: account.ctid_trader_account_id
                                            as i64,
                                        access_token: ACCESS_TOKEN.to_string(),
                                    };

                                    match socket.send(message::from(
                                        encode_proto_message_to_byte_vector(
                                            outgoing_message.payload_type.unwrap() as u32,
                                            outgoing_message.encode_to_vec(),
                                        ),
                                    )) {
                                        Ok(()) => println!("{:#?}", &outgoing_message),
                                        Err(e) => {
                                            println!("{:#?}: {:#?}", &outgoing_message, &e)
                                        }
                                    }
                                }
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2152 => match ProtoOaGetCtidProfileByTokenRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2154 => match ProtoOaAssetClassListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2155 => match ProtoOaDepthEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2157 => match ProtoOaSubscribeDepthQuotesRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2159 => match ProtoOaUnsubscribeDepthQuotesRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2161 => match ProtoOaSymbolCategoryListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2163 => match ProtoOaAccountLogoutRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2164 => match ProtoOaAccountDisconnectEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2165 => match ProtoOaSubscribeLiveTrendbarRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2166 => match ProtoOaUnsubscribeLiveTrendbarRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2168 => match ProtoOaMarginCallListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2170 => match ProtoOaMarginCallUpdateRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2171 => match ProtoOaMarginCallUpdateEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2172 => match ProtoOaMarginCallTriggerEvent::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2174 => match ProtoOaRefreshTokenRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2176 => match ProtoOaOrderListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2178 => match ProtoOaGetDynamicLeverageByIdRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2180 => match ProtoOaDealListByPositionIdRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2182 => match ProtoOaOrderDetailsRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2184 => match ProtoOaOrderListByPositionIdRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2186 => match ProtoOaDealOffsetListRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        2188 => match ProtoOaGetPositionUnrealizedPnLRes::decode(x.as_slice()) {
                            Ok(incoming_message) => {
                                println!("{:#?}", &incoming_message)
                            }
                            Err(e) => println!("{e:#?}"),
                        },
                        _ => {
                            println!(
                                "not covered yet: {:#?}",
                                &incoming_proto_message.payload_type
                            )
                        }
                    },
                    None => {
                        println!("uh-oh, something went wrong here... :(")
                    }
                },
                Err(e) => {
                    println!("DecodeError: {:#?}", &e)
                }
            },
            Err(e) => {
                println!("Error :: {:?}", e);
            }
        }
    }
}

fn encode_proto_message_to_byte_vector(payoad_type: u32, payload: Vec<u8>) -> Vec<u8> {
    let message = ProtoMessage {
        payload_type: payoad_type,
        payload: Some(payload),
        client_msg_id: None,
    };

    message.encode_to_vec()
}

/*
fn send(payload: Vec<u8>, mut socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
    match &socket.send(message::from(payload)) {
        Ok(()) => {}
        Err(e) => println!("{:#?}: ", &e),
    }
}

*/
