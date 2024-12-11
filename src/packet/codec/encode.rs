use super::*;

pub trait PacketEncode {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError>;
}

#[derive(Debug, Clone)]
pub enum EncodeError {}

impl<T: PacketEncode> PacketEncode for &T {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        (*self).encode(buf)
    }
}

macro packet_encode_int( $($types:ty),* $(,)? ) { $(
    impl PacketEncode for $types { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_u8s(&self.to_be_bytes());
        Ok(())
    } }
)* }
packet_encode_int!(u8, i8, u16, i16, u32, i32, u64, i64);

impl PacketEncode for bool { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.write_u8(if (*self) { 1 } else { 0 });
    Ok(())
} }

impl PacketEncode for String { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.encode_write(VarInt::from(self.len() as i32))?;
    buf.write_u8s(self.as_bytes());
    Ok(())
} }


#[cfg(test)]
mod tests {
    use crate::packet::c2s::handshake::{ HandshakeC2SPacket, IntendedStage };
    use super::*;

    #[test]
    fn basic_encoding() {
        let packet = HandshakeC2SPacket {
            protocol_version : VarInt::from(823),
            address          : "127.0.0.1".to_string(),
            port             : 25565,
            intended_stage   : IntendedStage::Status,
        };
        let mut buf = PacketBuf::new();
        packet.encode(&mut buf).unwrap();
        assert_eq!(buf.into_inner(), vec![183, 6, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 1]);
    }
}
