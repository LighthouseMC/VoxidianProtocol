use crate::value::nbt::into_element::IntoNbtElement;
use super::*;


#[derive(Clone, PartialEq, Default)]
pub struct NbtCompound(HashMap<String, NbtElement>);
impl NbtCompound {

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get<S : AsRef<String>>(&self, key : S) -> Option<&NbtElement> {
        self.0.get(key.as_ref())
    }

    pub fn insert<S : Into<String>, E : IntoNbtElement>(&mut self, key : S, value : E) {
        self.0.insert(key.into(), value.into_nbt());
    }

    pub fn extend(&mut self, other : NbtCompound) {
        self.0.extend(other.0.into_iter());
    }

}
impl fmt::Debug for NbtCompound { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.0)
} }
impl NbtCompound { pub(super) fn encode_packet(&self, buf : &mut PacketBuf) {
    for (key, value) in &self.0 {
        buf.write_u8(value.tag());
        NbtElement::String(key.clone()).encode_packet(buf);
        value.encode_packet(buf);
    }
    buf.write_u8(NbtElement::TAG_END);
} }
impl NbtCompound { pub(super) fn decode_packet(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let mut compound = Self::new();
    while (buf.remaining() > 0) {
        let tag = buf.read_u8()?;
        if (tag == NbtElement::TAG_END) { break; }
        let key = NbtElement::decode_string(buf)?;
        let value = NbtElement::decode_packet(buf, tag)?;
        compound.0.insert(key, value);
    }
    Ok(compound)
} }
