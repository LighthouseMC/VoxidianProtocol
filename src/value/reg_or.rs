use super::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegOr<T : RegValue, U> {
    Id(RegEntry<T>),
    Or(U)
}

impl<T : RegValue, U : PacketEncode> PacketEncode for RegOr<T, U> { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    match (self) {
        #[allow(deprecated)]
        Self::Id(entry) => { buf.encode_write(VarInt::from((entry.id() as i32) + 1)) }
        Self::Or(value) => {
            buf.encode_write(VarInt::from(0))?;
            buf.encode_write(value)
        }
    }
} }

impl<T : RegValue, U : PacketDecode> PacketDecode for RegOr<T, U> { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let id_plus_one = buf.read_decode::<VarInt>()?.as_i32() as u32;
    Ok(if (id_plus_one == 0) {
        Self::Id(unsafe{ RegEntry::new_unchecked(id_plus_one - 1) })
    } else {
        Self::Or(buf.read_decode::<U>()?)
    })
} }
