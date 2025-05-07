use super::*;


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Either<A, B> {
    True(A),
    False(B)
}

impl<A : PacketEncode, B : PacketEncode> PacketEncode for Either<A, B> {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        match (self) {
            Self::True(value) => {
                buf.write_u8(1);
                buf.encode_write(value)
            },
            Self::False(value) => {
                buf.write_u8(0);
                buf.encode_write(value)
            }
        }
    }
}
impl<A : PacketDecode, B : PacketDecode> PacketDecode for Either<A, B> {
    fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        let is_true = buf.read_u8()? != 0;
        Ok(if (is_true) {
            Self::True(buf.read_decode::<A>()?)
        } else {
            Self::False(buf.read_decode::<B>()?)
        })
    }
}
