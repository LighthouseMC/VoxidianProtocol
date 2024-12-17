mod compound;
pub use compound::*;
mod element;
pub use element::*;


use super::*;

use std::collections::HashMap;


#[derive(Clone, PartialEq, Default)]
pub struct Nbt {
    pub name : String,
    pub root : NbtCompound
}
impl fmt::Debug for Nbt { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Nbt({:?} -> Compound({:?}))", self.name, self.root)
} }
impl PacketEncode for Nbt { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.write_u8(NbtElement::TAG_COMPOUND);
    self.root.encode_packet(buf);
    Ok(())
} }
impl PacketDecode for Nbt { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let tag = buf.read_u8()?;
    if (tag != NbtElement::TAG_COMPOUND) {
        return Err(DecodeError::InvalidData("Nbt root is not a compound".to_string()));
    }
    Ok(Nbt {
        name : String::new(),
        root : NbtCompound::decode_packet(buf)?,
    })
} }
