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
            Ok(IdSet::Tag(tag))
        } else {
            let mut ids = Vec::with_capacity(amount.as_i32() as usize);
            for _ in 0..amount.as_i32() {
                ids.push(RegEntry::decode(buf)?);
            }
            Ok(IdSet::Ids(ids))
        }

    }
}

impl<T: RegValue + PacketEncode + PacketDecode> PartialEq for IdSet<T> {
    fn eq(&self, other : &Self) -> bool {
        match ((self, other)) {
            (IdSet::Tag(id0), IdSet::Tag(id1)) => id0 == id1,
            (IdSet::Ids(ids0), IdSet::Ids(ids1)) => {
                if (ids0.len() != ids1.len()) { return false; }
                for id in ids0 {
                    if (! ids1.contains(id)) { return false; }
                }
                true
            },
            _ => false
        }
    }
}
