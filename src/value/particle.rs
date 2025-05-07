use super::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Particle {
    #[serde(rename = "type")]
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


impl PacketEncode for Particle {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(&self.id)?;
        Ok(())
    }
}
impl<'l> PacketDecode<'l> for Particle { fn decode(_buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    todo!("PacketDecode for Particle")
} }
