use crate::value::*;
use voxidian_protocol_macros::packet_part;

mod components;
pub use components::*;

#[packet_part]
pub struct SlotData {
    pub item_count: VarInt,
    pub item_id: Option<VarInt>
}

pub trait ComponentData {
    const ID: u32;
}