
use crate::value::{PacketEncode, VarInt};
use crate::value::PacketBuf;
use crate::value::PacketDecode;
use crate::value::EncodeError;
use crate::value::DecodeError;
use voxidian_protocol_macros::{component, packet_part};

#[component("minecraft:max_stack_size")]
pub struct MaxStackSize {
    pub max_stack_size: VarInt
}