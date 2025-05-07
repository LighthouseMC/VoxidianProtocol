use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct JsonText(pub(super) String);
impl JsonText {
    pub fn into_inner(self) -> String { self.0 }
}
impl PacketEncode for JsonText { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    buf.encode_write(&self.0)
} }
impl PacketDecode for JsonText { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    Ok(JsonText(buf.read_decode::<String>()?))
} }
