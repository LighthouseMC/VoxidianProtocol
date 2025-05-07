use super::*;


#[derive(Clone, Copy, Debug, Default)]
pub enum OptionVarInt {
    Some(VarInt),
    #[default]
    None
}
impl OptionVarInt {
    pub fn into_option(&self) -> Option<VarInt> { match (self) {
        Self::Some(value) => Some(*value),
        Self::None        => None
    } }
}
impl From<Option<VarInt>> for OptionVarInt { fn from(value : Option<VarInt>) -> Self { match (value) {
    Some(value) => Self::Some(value),
    None        => Self::None
} } }

impl PacketEncode for OptionVarInt { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    VarInt::from(match (self) {
        Self::Some(value) => { value.as_i32() + 1 },
        Self::None        => { 0 }
    }).encode(buf)
} }

impl PacketDecode for OptionVarInt { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let value_plus_1 = VarInt::decode(buf)?.as_i32();
    Ok(if (value_plus_1 == 0) {
        Self::None
    } else {
        Self::Some(VarInt::from(value_plus_1 - 1))
    })
} }
