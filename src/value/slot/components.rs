
use crate::value::VarInt;
use voxidian_protocol_macros::component;

#[component("minecraft:max_stack_size")]
pub struct MaxStackSize {
    pub max_stack_size: VarInt
}