use super::*;


#[derive(Clone, Copy, Debug, Default)]
pub enum OptionVarInt {
    Some(Var32),
    #[default]
    None
}
impl OptionVarInt {
    pub fn into_option(&self) -> Option<Var32> { match (self) {
        Self::Some(value) => Some(*value),
        Self::None        => None
    } }
}
impl From<Option<Var32>> for OptionVarInt { fn from(value : Option<Var32>) -> Self { match (value) {
    Some(value) => Self::Some(value),
    None        => Self::None
} } }

impl PacketEncode for OptionVarInt { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    Var32::from(match (self) {
        Self::Some(value) => { value.as_i32() + 1 },
        Self::None        => { 0 }
    }).encode(buf)
} }

impl PacketDecode for OptionVarInt { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let value_plus_1 = Var32::decode(buf)?.as_i32();
    Ok(if (value_plus_1 == 0) {
        Self::None
    } else {
        Self::Some(Var32::from(value_plus_1 - 1))
    })
} }
