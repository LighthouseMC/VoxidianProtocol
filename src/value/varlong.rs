use super::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarLong(i64);

impl VarLong {

    pub fn as_i64(self) -> i64 { self.into() }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut x = self.0 as u64;
        loop {
            let byte = (x & 0b01111111) as u8;
            x >>= 7;
            if (x == 0) {
                out.push(byte);
                break;
            }
            out.push(byte | 0b10000000);
        }
        out
    }

}

impl From<i64> for VarLong { fn from(value : i64) -> Self { Self(value) } }
impl Into<i64> for VarLong { fn into(self) -> i64 { self.0 } }
impl From<usize> for VarLong { fn from(value : usize) -> Self { Self(value as i64) } }
impl Into<usize> for VarLong { fn into(self) -> usize { self.0 as usize } }

impl PacketEncode for VarLong { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.write_u8s(&self.as_bytes());
    Ok(())
} }

impl PacketDecode for VarLong { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let ((out, consumed)) = Self::decode_iter(&mut buf.iter())?;
    buf.skip(consumed);
    Ok(out)
} }
impl VarLong {
    /// Also returns the number of bytes that were consumed.
    pub fn decode_iter(iter : &mut impl Iterator<Item = u8>) -> Result<(Self, usize), DecodeError> {
        const MAX_BYTES: usize = 10;
        let mut x = 0;
        let mut consumed = 0;
        for i in 0..MAX_BYTES {
            let byte = iter.next().ok_or(DecodeError::EndOfBuffer)?;
            consumed += 1;
            x |= (i64::from(byte) & 0b01111111) << (i * 7);
            if (byte & 0b10000000 == 0) {
                return Ok((VarLong(x), consumed));
            }
        }
        Err(DecodeError::InvalidData)
    }
}
