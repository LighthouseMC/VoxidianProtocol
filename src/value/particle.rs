use super::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Particle {
    pub id : Identifier,
}
impl Particle {
    pub fn to_nbt(&self) -> NbtCompound {
        let mut nbt = NbtCompound::new();
        nbt.insert("type", NbtElement::String(self.id.to_string()));
        // TODO: Insert particle-specific data
        nbt
    }
}


impl PacketEncode for Particle { fn encode(&self, _buf : &mut PacketBuf) -> Result<(), EncodeError> {
    todo!("PacketEncode for Particle")
} }
impl PacketDecode for Particle { fn decode(_buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    todo!("PacketDecode for Particle")
} }
