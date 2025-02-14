use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VarInt(i32);

impl VarInt {

    pub const fn new(from : i32) -> Self {
        Self(from)
    }

    pub fn as_i32(self) -> i32 { self.into() }

}
impl fmt::Debug for VarInt { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "VarInt({})", self.0)
} }

impl From<i32> for VarInt { fn from(value : i32) -> Self { Self(value) } }
impl Into<i32> for VarInt { fn into(self) -> i32 { self.0 } }
impl From<usize> for VarInt { fn from(value : usize) -> Self { Self(value as i32) } }
impl Into<usize> for VarInt { fn into(self) -> usize { self.0 as usize } }

impl PacketEncode for VarInt { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.write_u8s(&self.as_bytes());
    Ok(())
} }

impl PacketDecode for VarInt { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let ((out, consumed)) = Self::decode_iter(&mut buf.iter())?;
    buf.skip(consumed);
    Ok(out)
} }
impl VarInt {
    const SEGMENT_BITS: i32 = 0x7F;
    const CONTINUE_BIT: i32 = 0x80;

    /// Also returns the number of bytes that were consumed.
    pub fn decode_iter(iter : &mut impl Iterator<Item = u8>) -> Result<(Self, usize), DecodeError> {
        let mut value = 0;
        let mut position = 0;
        let mut index = 0;
        loop {
            let byte = iter.next().ok_or(DecodeError::EndOfBuffer)?;
            value |= ((byte & 0x7F) as i32) << position;

            if (byte & 0x80) == 0 {
                break;
            }

            position += 7;
            index += 1;

            if position >= 32 {
                return Err(DecodeError::InvalidData("VarInt is too long".to_string()));
            }
        }

        Ok((VarInt::from(value), index + 1))
    }



    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let mut data = self.0;
        loop {
            if (data & !(Self::SEGMENT_BITS)) == 0 {
                bytes.push(data as u8);
                return bytes;
            }

            bytes.push(((data & Self::SEGMENT_BITS) | Self::CONTINUE_BIT) as u8);
            data = ((data as u32) >> 7) as i32;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::value::VarInt;

    #[test]
    pub fn test_negative_varints() {
        assert_eq!(VarInt::decode_iter(&mut [0x00].into_iter()).unwrap().0.as_i32(), 0);
        assert_eq!(VarInt::decode_iter(&mut [0xff, 0xff, 0xff, 0xff, 0x0f].into_iter()).unwrap().0.as_i32(), -1);
        let encoded = VarInt::from(-1).as_bytes();
        assert_eq!(VarInt::decode_iter(&mut encoded.into_iter()).unwrap().0.as_i32(), -1);
    }
}