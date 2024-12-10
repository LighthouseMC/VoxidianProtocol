use super::*;

pub trait PacketEncode {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError>;
}

pub enum EncodeError {}

impl<T: PacketEncode> PacketEncode for &T {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        // FIXME: recursion limit reached here while instantiating
        buf.encode_write(self)
    }
}

macro packet_encode_int( $($types:ty),* $(,)? ) { $(
    impl PacketEncode for $types { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_u8s(&self.to_be_bytes());
        Ok(())
    } }
)* }
packet_encode_int!(u8, i8, u16, i16, u32, i32, u64, i64);

impl PacketEncode for String {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.encode_write(VarInt::from(self.len() as i32))?;
        buf.write_u8s(self.as_bytes());
        Ok(())
    }
}
