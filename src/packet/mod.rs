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


#[derive(Debug, Clone)]
pub struct TODO(());
impl PacketEncode for TODO { #[track_caller] fn encode(&self, _ : &mut PacketBuf) -> Result<(), EncodeError> {
    todo!("Packet field TODO");
} }
impl PacketDecode for TODO { #[track_caller] fn decode(_ : &mut PacketBuf) -> Result<Self, DecodeError> {
    todo!("Packet field TODO");
} }


pub(crate) macro packet_flags(
    $vis:vis struct $ident:ident<$type:ty> { $(
        $fieldvis:vis $fieldident:ident : $bitmask:tt
    ),* $(,)? }
) {

    #[derive(Debug, Clone, PartialEq, Eq)]
    $vis struct $ident { $(
        $fieldvis $fieldident : bool
    ),* }
    impl PacketEncode for $ident { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        <$type as PacketEncode>::encode(
            &($(
                (if (self.$fieldident) { $bitmask } else { 0b00000000 })
            )|*),
            buf
        )?;
        Ok(())
    } }
    impl PacketDecode for $ident { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let bits = <$type as PacketDecode>::decode(buf)?;
        Ok(Self { $(
            $fieldident : (bits & $bitmask) != 0
        ),* })
    } }
}

#[cfg(test)]
mod tests {
    use crate::packet::codec::{PacketEncode, PacketDecode};
    use super::{packet_flags, PacketBuf};


    packet_flags! {
        struct FlagsWithU8<u8> {
            a: 0b001,
            b: 0b010,
            c: 0b100
        }
    }

    packet_flags! {
        struct FlagsWithI32<i32> {
            a: 0b001,
            b: 0b010,
            c: 0b100
        }
    }

    #[test]
    pub fn test_u8_packet_flags() {
        let flags = FlagsWithU8 {
            a: true,
            b: false,
            c: true
        };
        let mut buf = PacketBuf::new();
        flags.encode(&mut buf).unwrap();
        let new_flags = FlagsWithU8::decode(&mut buf).unwrap();
        assert_eq!(flags, new_flags);
    }

    #[test]
    pub fn test_i32_packet_flags() {
        let flags = FlagsWithI32 {
            a: true,
            b: false,
            c: true
        };
        let mut buf = PacketBuf::new();
        flags.encode(&mut buf).unwrap();
        let new_flags = FlagsWithI32::decode(&mut buf).unwrap();
        assert_eq!(flags, new_flags);
    }
}
