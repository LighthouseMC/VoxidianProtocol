use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VarLong(i64);

impl VarLong {

    pub fn as_i64(self) -> i64 { self.into() }

}
impl fmt::Debug for VarLong { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "VarLong({})", self.0)
} }

impl From<i64> for VarLong { fn from(value : i64) -> Self { Self(value) } }
impl From<VarLong> for i64 { fn from(val: VarLong) -> Self { val.0 } }
impl From<usize> for VarLong { fn from(value : usize) -> Self { Self(value as i64) } }
impl From<VarLong> for usize { fn from(val: VarLong) -> Self { val.0 as usize } }

impl PacketEncode for VarLong { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    buf.write_u8s(&self.as_bytes());
    Ok(())
} }

impl PacketDecode for VarLong { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    Ok(Self::decode_iter(&mut buf.iter())?.0)
} }
impl VarLong {
    const SEGMENT_BITS: i64 = 0x7F;
    const CONTINUE_BIT: i64 = 0x80;

    /// Also returns the number of bytes that were consumed.
    pub fn decode_iter(iter : &mut impl Iterator<Item = u8>) -> Result<(Self, usize), DecodeError> {
        let mut value: i64 = 0;
        let mut position = 0;
        let mut index = 0;
        loop {
            let byte = iter.next().ok_or(DecodeError::EndOfBuffer)?;
            value |= ((byte & 0x7F) as i64) << position;

            if (byte & 0x80) == 0 {
                break;
            }

            position += 7;
            index += 1;

            if position >= 64 {
                return Err(DecodeError::InvalidData(Cow::Borrowed("VarLong is too long")));
            }
        }

        Ok((VarLong::from(value), index + 1))
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
            data = ((data as u64) >> 7) as i64;
        }
    }
}
