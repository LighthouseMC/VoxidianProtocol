use super::*;


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Var64(i64);

impl Var64 {

    pub fn as_i64(self) -> i64 { self.into() }

}
impl fmt::Debug for Var64 { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "VarLong({})", self.0)
} }

impl From<i64> for Var64 { fn from(value : i64) -> Self { Self(value) } }
impl From<Var64> for i64 { fn from(val: Var64) -> Self { val.0 } }
impl From<usize> for Var64 { fn from(value : usize) -> Self { Self(value as i64) } }
impl From<Var64> for usize { fn from(val: Var64) -> Self { val.0 as usize } }

impl PacketEncode for Var64 { fn encode(&self, writer : &mut PacketWriter) -> Result<(), EncodeError> {
    let mut buf = [0; 10];
    let     n   = self.bytes_buf(&mut buf);
    writer.write_u8s(&buf[0..n]);
    Ok(())
} }

impl PacketDecode for Var64 { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    Ok(Self::decode_iter(&mut buf.iter())?.0)
} }
impl Var64 {
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

        Ok((Var64::from(value), index + 1))
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
            data = ((data as u64) >> 7) as i64;
        }
    }
}
