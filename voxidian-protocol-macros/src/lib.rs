#![feature(
    proc_macro_diagnostic,
    proc_macro_span,
    let_chains,
    decl_macro
)]

mod component;
mod packet;
mod packet_full_decode;
mod packet_part;
mod reg_from_file;

use std::env;
use std::sync::LazyLock;
use std::collections::HashMap;
use proc_macro::TokenStream;
use serde::Deserialize as Deser;


/// Cache
static PACKETS_DATA : LazyLock<PacketsData> = LazyLock::new(|| {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("generated").join("packets.json");
    let Ok(file) = std::fs::read_to_string(&path) else { panic!("`packets.json` missing ({})", path.display()) };
    let Ok(data) = serde_json::from_str::<PacketsData>(&file) else { panic!("`packets.json` in invalid format") };
    data
});
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

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::component::component_impl(attr, item)
}

#[proc_macro]
pub fn component_enum(_input: TokenStream) -> TokenStream {
    crate::component::component_enum_impl()
}

#[proc_macro_attribute]
pub fn packet(attr : TokenStream, item : TokenStream) -> TokenStream {
    crate::packet::packet_impl(attr, item) 
}

#[proc_macro]
pub fn packet_full_decode(input : TokenStream) -> TokenStream {
    crate::packet_full_decode::packet_full_decode_impl(input)
}

#[proc_macro_attribute]
pub fn packet_part(attr : TokenStream, item : TokenStream) -> TokenStream {
    crate::packet_part::packet_part_impl(attr, item)
}

#[proc_macro]
pub fn import_item_registry_from_file(input: TokenStream) -> TokenStream {
    reg_from_file::item_reg_from_file_impl(input)
}

#[proc_macro]
pub fn import_damage_type_registry_from_file(input: TokenStream) -> TokenStream {
    reg_from_file::damage_types_from_file_impl(input)
}