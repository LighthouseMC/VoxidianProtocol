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
    /// Also returns the number of bytes that were consumed.
    pub fn decode_iter(iter : &mut impl Iterator<Item = u8>) -> Result<(Self, usize), DecodeError> {
        let mut value: i32 = 0;
        let mut position: u32 = 0;
        let mut consumed: usize = 0;

        loop {
            let current_byte = iter.next().ok_or(DecodeError::EndOfBuffer)?;
            consumed += 1;

            let segment = (current_byte & 0x7F) as u32;
            value |= (segment << position) as i32;

            if (current_byte & 0x80) == 0 {
                return Ok((VarInt(value), consumed));
            }

            position += 7;
            if position >= 32 {
                return Err(DecodeError::InvalidData("VarInt data exceeded max size".to_string(),));
            }
        }
    }



    pub fn as_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut value = self.0 as u32; // Treat as unsigned for right shifts

        loop {
            if value <= 0x7F {
                out.push(value as u8);
                break;
            } else {
                out.push(((value & 0x7F) as u8) | 0x80);
                value >>= 7;
            }
        }

        out
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