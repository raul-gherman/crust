use log::info;

use crate::{
    ctrader_open_api::*, engagement::*, symbol::Symbol
};

pub fn process(
    incoming_message: &ProtoOaSpotEvent,
    symbol_1: &mut Symbol,
    symbol_2: &mut Symbol,
    symbol_3: &mut Symbol,
    threshold: &f64,
) {
    if incoming_message.symbol_id == symbol_1.id {
        if incoming_message.ask() > 0 {
            symbol_1.ask = incoming_message.ask() as f64
        };
        if incoming_message.bid() > 0 {
            symbol_1.bid = incoming_message.bid() as f64
        };
    }
    // info!("{:#?}", &symbol_1);

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
        // BBS=(1-(A.ask x B.ask) / C.Bid) x 10_000
        let bbs = (symbol_1.ask * symbol_2.ask / symbol_3.bid) - 100000.0;
        //info!("BBS: {:#?}", &bbs);
        if bbs < *threshold {
            info!("BBS: {:#?}", &bbs);
            info!("symbol_1: {:#?}", &symbol_1);
            info!("symbol_2: {:#?}", &symbol_2);
            info!("symbol_3: {:#?}", &symbol_3);
        }

        // SSB=(((A.bid x B.bid) / C.ask) -1) x 10_000
        let ssb = 100000.0 - ((symbol_1.bid * symbol_2.bid) / symbol_3.ask);
        //info!("SSB: {:#?}", &ssb);
        if ssb < *threshold {
            info!("SSB: {:#?}", &ssb);
            info!("symbol_1: {:#?}", &symbol_1);
            info!("symbol_2: {:#?}", &symbol_2);
            info!("symbol_3: {:#?}", &symbol_3);
        }
    }
}
