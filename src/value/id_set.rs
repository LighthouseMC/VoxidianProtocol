use super::{DecodeError, EncodeError, Identifier, PacketBuf, PacketDecode, PacketEncode, RegEntry, RegValue, VarInt};

#[derive(Clone, Debug)]
pub enum IdSet<T: RegValue + PacketEncode + PacketDecode> {
    Tag(Identifier),
    Ids(Vec<RegEntry<T>>)
}

impl<T: RegValue + PacketEncode + PacketDecode> PacketEncode for IdSet<T> {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        match self {
            IdSet::Tag(tag) => {
                buf.encode_write::<VarInt>(VarInt::from(0))?;
                tag.encode(buf)?;
            },
            IdSet::Ids(ids) => {
                buf.encode_write::<VarInt>(VarInt::from(ids.len()))?;
                for id in ids {
                    id.encode(buf)?;
                }
            }
        }
        Ok(())
    }
}

impl<T: RegValue + PacketEncode + PacketDecode> PacketDecode for IdSet<T> {
    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        let amount = buf.read_decode::<VarInt>()?;
        if amount.as_i32() == 0 {
            let tag = Identifier::decode(buf)?;
            return Ok(IdSet::Tag(tag));
        } else {
            let mut ids = Vec::with_capacity(amount.as_i32() as usize);
            for _ in 0..amount.as_i32() {
                ids.push(RegEntry::decode(buf)?);
            }
            return Ok(IdSet::Ids(ids));
        }

    }
}