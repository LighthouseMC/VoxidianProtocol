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
impl From<VarInt> for i32 { fn from(val: VarInt) -> Self { val.0 } }
impl From<usize> for VarInt { fn from(value : usize) -> Self { Self(value as i32) } }
impl From<VarInt> for usize { fn from(val: VarInt) -> Self { val.0 as usize } }

impl PacketEncode for VarInt { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    buf.write_u8s(&self.as_bytes());
    Ok(())
} }

impl<'l> PacketDecode<'l> for VarInt { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
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
                return Err(DecodeError::InvalidData(Cow::Borrowed("VarInt is too long")));
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
    fn varint_decode_iter() {
        let data = [
            16, 0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1, 1, 0,
        ];
        //          ^Packet length
        let Ok((len, consumed)) = VarInt::decode_iter(&mut data.into_iter()) else {
            panic!("decode_iter was not a success");
        };
        assert_eq!(len.as_i32(), 16);
        assert_eq!(consumed, 1);
    }

    #[test]
    pub fn test_negative_varints() {
        assert_eq!(VarInt::decode_iter(&mut [0x00].into_iter()).unwrap().0.as_i32(), 0);
        assert_eq!(VarInt::decode_iter(&mut [0xff, 0xff, 0xff, 0xff, 0x0f].into_iter()).unwrap().0.as_i32(), -1);
        let encoded = VarInt::from(-1).as_bytes();
        assert_eq!(VarInt::decode_iter(&mut encoded.into_iter()).unwrap().0.as_i32(), -1);
    }
}
