use crate::value::PacketEncode;
use crate::value::PacketDecode;
use crate::value::PacketBuf;
use crate::value::EncodeError;
use crate::value::DecodeError;
use voxidian_protocol_macros::packet_part;
use crate::value::{OptionVarInt, VarInt};

mod components;

#[packet_part]
pub struct SlotData {
    pub item_count: VarInt,
    pub item_id: Option<VarInt>
}

pub trait ComponentData {
    const ID: u32;
}