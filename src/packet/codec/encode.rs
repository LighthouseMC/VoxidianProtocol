use super::*;

pub trait PacketEncode {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError>;
}

#[derive(Debug, Clone)]
pub enum EncodeError {
    /// Failed to send the packet to the client.
    SendFailed,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            Self::SendFailed => write!(f, "failed to send")
        }
    }
}

impl<T: PacketEncode> PacketEncode for &T {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        (*self).encode(buf)
    }
}

macro packet_encode_num( $($types:ty),* $(,)? ) { $(
    impl PacketEncode for $types { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        buf.write_u8s(&self.to_be_bytes());
        Ok(())
    } }
)* }
packet_encode_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl PacketEncode for bool {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        buf.write_u8(if (*self) { 1 } else { 0 });
        Ok(())
    }
}

impl PacketEncode for Uuid {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        let (msb, lsb) = self.as_u64_pair();
        buf.encode_write(msb)?;
        buf.encode_write(lsb)?;
        Ok(())
    }
}

impl PacketEncode for &str {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(VarInt::from(self.len() as i32))?;
        buf.write_u8s(self.as_bytes());
        Ok(())
    }
}

impl PacketEncode for String {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        self.as_str().encode(buf)
    }
}

impl<T: PacketEncode> PacketEncode for Option<T> {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        if let Some(value) = self {
            buf.write_u8(1);
            buf.encode_write(value)
        } else {
            buf.write_u8(0);
            Ok(())
        }
    }
}

impl<T: PacketEncode, const LEN: usize> PacketEncode for [T; LEN] {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        for item in self {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}

pub trait PrefixedPacketEncode {
    /// Encode the full packet.
    /// This includes the packet ID and packet data, but **does not include the full packet length**
    fn encode_prefixed(&self, buf: &mut PacketWriter) -> Result<(), EncodeError>;
}

impl<T: PacketEncode + PacketMeta> PrefixedPacketEncode for T {
    fn encode_prefixed(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(VarInt::from(Self::PREFIX as i32))?;
        self.encode(buf)?;
        Ok(())
    }
}

impl<A: PacketEncode, B: PacketEncode> PacketEncode for (A, B) {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        self.0.encode(buf)?;
        self.1.encode(buf)
    }
}

impl<A: PacketEncode, B: PacketEncode, C: PacketEncode> PacketEncode for (A, B, C) {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        self.0.encode(buf)?;
        self.1.encode(buf)?;
        self.2.encode(buf)
    }
}

impl<A: PacketEncode, B: PacketEncode, C: PacketEncode, D: PacketEncode> PacketEncode
    for (A, B, C, D)
{
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        self.0.encode(buf)?;
        self.1.encode(buf)?;
        self.2.encode(buf)?;
        self.3.encode(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::c2s::handshake::{IntendedStage, IntentionC2SHandshakePacket};

    #[test]
    fn basic_encoding() {
        let packet = IntentionC2SHandshakePacket {
            protocol_version: VarInt::from(823),
            address: "127.0.0.1".to_string(),
            port: 25565,
            intended_stage: IntendedStage::Status,
        };
        let mut buf = PacketWriter::new();
        packet.encode(&mut buf).unwrap();
        assert_eq!(buf.into_inner(), vec![
            183, 6, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 1
        ]);
    }
}
