#![feature(
    proc_macro_diagnostic,
    proc_macro_span,
    let_chains,
    decl_macro
)]

use std::{ fs, env };
use std::path::PathBuf;
use std::sync::Mutex;
use std::collections::HashMap;
use proc_macro::{ Span, TokenStream };
use proc_macro2::TokenStream as TokenStream2;
use quote::{ quote, quote_spanned };
use syn::{
    Field, Fields, FieldsNamed, FieldsUnnamed,
    Ident, Index, Item, ItemEnum, ItemStruct,
    LitStr, Meta, Type, Variant, Visibility,
    parse_macro_input, parse_str,
    spanned::Spanned,
    punctuated::Punctuated
};
use serde::Deserialize as Deser;
use inflector::Inflector;


include!("packet.rs");
include!("packet_part.rs");
include!("packet_full_decode.rs");


/// Cache
static PACKETS_DATA : Mutex<Option<PacketsData>> = Mutex::new(None);


#[derive(Deser)]
#[serde(transparent)]
struct PacketsData {
    packets : HashMap<String, PacketData>
}
#[derive(Deser, Debug, Clone, Copy, PartialEq, Eq)]
struct PacketData {
    prefix : u8,
    bound  : PacketBound,
    stage  : PacketStage
}
#[derive(Deser, Debug, Clone, Copy, PartialEq, Eq)]
enum PacketBound {
    S2C,
    C2S
}
#[derive(Deser, Debug, Clone, Copy, PartialEq, Eq)]
enum PacketStage {
    Handshake,
    Status,
    Login,
    Config,
    Play
}


macro get_packets_data(let $pat:pat) {
    PACKETS_DATA.clear_poison();
    let mut packets_data = match (PACKETS_DATA.lock()) {
        Ok  (guard ) => guard,
        Err (_     ) => { PACKETS_DATA.clear_poison(); PACKETS_DATA.lock().unwrap() }
    };
    let $pat = packets_data.get_or_insert_with(
        || {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("generated").join("packets.json");
            let Ok(file) = fs::read_to_string(&path) else { panic!("`packets.json` missing ({})", path.display()) };
            let Ok(data) = serde_json::from_str::<PacketsData>(&file) else { panic!("`packets.json` in invalid format") };
            data
        }
    );
}
