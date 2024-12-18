mod codec;
pub use codec::*;
mod buffer;
pub use buffer::*;
mod meta;
pub use meta::*;
pub mod processing;
pub(crate) use processing::*;

pub mod c2s;
pub mod s2c;

pub(crate) use crate::MINECRAFT_VERSION;
pub(crate) use crate::value::*;
pub(crate) use crate::registry::*;
pub(crate) use voxidian_protocol_macros::{ packet, packet_part, packet_full_decode };
pub(crate) use std::fmt;
pub(crate) use serde::{ Serialize as Ser, Deserialize as Deser, Serializer as Serer, Deserializer as Deserer };
pub(crate) use serde_json::ser::to_string as to_json_string;
pub(crate) use serde_json::de::from_str as from_json_str;
pub(crate) use uuid::Uuid;


#[derive(Debug)]
struct TODO(());
impl PacketEncode for TODO { fn encode(&self, _ : &mut PacketBuf) -> Result<(), EncodeError> {
    todo!("Packet field TODO");
} }
impl PacketDecode for TODO { fn decode(_ : &mut PacketBuf) -> Result<Self, DecodeError> {
    todo!("Packet field TODO");
} }


pub(crate) macro packet_flags(
    $vis:vis struct $ident:ident { $(
        $fieldvis:vis $fieldident:ident : $bitmask:tt
    ),* $(,)? }
) {

    #[derive(Debug, Clone, PartialEq, Eq)]
    $vis struct $ident { $(
        $fieldvis $fieldident : bool
    ),* }
    impl PacketEncode for $ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_u8( $(
            (if (self.$fieldident) { $bitmask } else { 0b00000000 })
        )|* );
        Ok(())
    } }
    impl PacketDecode for $ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let bits = buf.read_u8()?;
        Ok(Self { $(
            $fieldident : (bits & $bitmask) != 0
        ),* })
    } }

}
