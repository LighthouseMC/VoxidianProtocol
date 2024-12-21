
use crate::value::PacketEncode;
use crate::value::PacketDecode;
use crate::value::PacketBuf;
use crate::value::EncodeError;
use crate::value::DecodeError;
use crate::value::VarInt;
use voxidian_protocol_macros::{component, packet_part};

#[derive(Clone)]
#[component("minecraft:max_stack_size")]
#[packet_part]
pub struct MaxStackSize {
    pub max_stack_size: VarInt
}

#[derive(Debug, Clone)]
pub enum DataComponent {
    MaxStackSize(MaxStackSize)
}

#[derive(Debug, Clone)]
pub enum DataComponentType {
    MaxStackSize,
}