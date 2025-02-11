use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct JsonText(pub(super) String);
impl JsonText {
    pub fn into_inner(self) -> String { self.0 }
}
impl PacketEncode for JsonText { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.encode_write(to_json_string(&self.0).unwrap())
} }
impl PacketDecode for JsonText { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    Ok(JsonText(from_json_str(&buf.read_decode::<String>()?).map_err(|_| DecodeError::InvalidData("Text data is not valid JSON".to_string()))?))
} }
