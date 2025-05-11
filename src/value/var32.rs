use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Var32(i32);

impl Var32 {

    pub const fn new(from : i32) -> Self {
        Self(from)
    }

    pub fn as_i32(self) -> i32 { self.into() }

}
impl fmt::Debug for Var32 { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "VarInt({})", self.0)
} }

impl From<i32> for Var32 { fn from(value : i32) -> Self { Self(value) } }
impl From<Var32> for i32 { fn from(val: Var32) -> Self { val.0 } }
impl From<usize> for Var32 { fn from(value : usize) -> Self { Self(value as i32) } }
impl From<Var32> for usize { fn from(val: Var32) -> Self { val.0 as usize } }

impl PacketEncode for Var32 { fn encode(&self, writer : &mut PacketWriter) -> Result<(), EncodeError> {
    let mut buf = [0; 5];
    let     n   = self.bytes_buf(&mut buf);
    writer.write_u8s(&buf[0..n]);
    Ok(())
} }

impl PacketDecode for Var32 { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    Ok(Self::decode_iter(&mut buf.iter())?.0)
} }
impl Var32 {
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

        Ok((Var32::from(value), index + 1))
    }


    pub fn bytes_buf(&self, buf : &mut [u8]) -> usize {
        let mut data = self.0;
        let mut i    = 0;
        loop {
            if ((data & !(Self::SEGMENT_BITS)) == 0) {
                buf[i] = data as u8;
                return i + 1;
            }
            buf[i] = ((data & Self::SEGMENT_BITS) | Self::CONTINUE_BIT) as u8;
            i += 1;
            data = ((data as u32) >> 7) as i32;
        }
    }

    pub fn extend_bytes(&self, vec : &mut Vec<u8>) {
        let mut buf = [0; 5];
        let     n   = self.bytes_buf(&mut buf);
        vec.extend_from_slice(&buf[0..n]);
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Var32;

    #[test]
    fn varint_decode_iter() {
        let data = [
            16, 0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1, 1, 0,
        ];
        //          ^Packet length
        let Ok((len, consumed)) = Var32::decode_iter(&mut data.into_iter()) else {
            panic!("decode_iter was not a success");
        };
        assert_eq!(len.as_i32(), 16);
        assert_eq!(consumed, 1);
    }

    #[test]
    pub fn test_negative_varints() {
        assert_eq!(Var32::decode_iter(&mut [0x00].into_iter()).unwrap().0.as_i32(), 0);
        assert_eq!(Var32::decode_iter(&mut [0xff, 0xff, 0xff, 0xff, 0x0f].into_iter()).unwrap().0.as_i32(), -1);
        let mut buf = [0; 5];
        let     n   = Var32::from(-1).bytes_buf(&mut buf);
        assert_eq!(Var32::decode_iter(&mut buf[0..n].iter().cloned()).unwrap().0.as_i32(), -1);
    }
}
