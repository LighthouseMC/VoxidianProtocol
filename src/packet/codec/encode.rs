use super::*;


pub trait PacketEncode {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError>;
}

#[derive(Debug, Clone)]
pub enum EncodeError {

    /// Failed to send the packet to the client.
    SendFailed

}

impl<T : PacketEncode> PacketEncode for &T { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    (*self).encode(buf)
} }

macro packet_encode_num( $($types:ty),* $(,)? ) { $(
    impl PacketEncode for $types { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_u8s(&self.to_be_bytes());
        Ok(())
    } }
)* }
packet_encode_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl PacketEncode for bool { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.write_u8(if (*self) { 1 } else { 0 });
    Ok(())
} }

impl PacketEncode for Uuid { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    let (msb, lsb) = self.as_u64_pair();
    buf.encode_write(msb)?;
    buf.encode_write(lsb)?;
    Ok(())
} }

impl PacketEncode for String { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.encode_write(VarInt::from(self.len() as i32))?;
    buf.write_u8s(self.as_bytes());
    Ok(())
} }

impl<T : PacketEncode> PacketEncode for Option<T> { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    if let Some(value) = self {
        buf.write_u8(1);
        buf.encode_write(value)
    } else {
        buf.write_u8(0);
        Ok(())
    }
} }



pub trait PacketEncodeFull {
    /// Encode the full packet.
    /// This includes:
    /// - Packet length (VarInt)
    /// - Packet prefix/ID (VarInt)
    /// - Packet data (...)
    fn encode_full(&self, buf : &mut PacketBuf) -> Result<(), EncodeError>;
}

impl<T : PacketEncode + PacketMeta> PacketEncodeFull for T {
    fn encode_full(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        // Data
        let mut data_buf = PacketBuf::new();
        self.encode(&mut data_buf)?;
        // Packet ID
        let packetid = VarInt::from(Self::PREFIX as i32).as_bytes();
        // Packet Length
        let packetlen = VarInt::from((packetid.len() + data_buf.remaining()) as i32).as_bytes();
        // Write
        buf.write_u8s(&packetlen);
        buf.write_u8s(&packetid);
        buf.write_u8s(data_buf.as_slice());
        Ok(())
    }
}



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
