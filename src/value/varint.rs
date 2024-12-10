use crate::packet::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt(i32);

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Into<i32> for VarInt {
    fn into(self) -> i32 {
        self.0
    }
}

impl PacketEncode for VarInt {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        let mut x = self.0 as u64;
        loop {
            let byte = (x & 0b01111111) as u8;
            x >>= 7;
            if (x == 0) {
                buf.write_u8(byte);
                break;
            }
            buf.write_u8(byte | 0b10000000);
        }
        Ok(())
    }
}

impl PacketDecode for VarInt {
    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        const MAX_BYTES: usize = 5;
        let mut x = 0;
        for i in 0..MAX_BYTES {
            let byte = buf.read_u8()?;
            x |= (i32::from(byte) & 0b01111111) << (i * 7);
            if (byte & 0b10000000 == 0) {
                return Ok(VarInt(x));
            }
        }
        Err(DecodeError::InvalidData)
    }
}
