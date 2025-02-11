use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct NbtText(pub(super) NbtElement);
impl NbtText {
    pub fn into_inner(self) -> NbtElement { self.0 }
}
impl PacketEncode for NbtText { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { self.0.encode(buf) } }
impl PacketDecode for NbtText { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { Ok(Self(NbtElement::decode(buf)?)) } }
