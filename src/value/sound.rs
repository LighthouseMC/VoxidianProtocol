use super::*;


#[derive(Debug, Clone)]
pub enum Sound {
    Registry(RegEntry<SoundEvent>),
    Inline(SoundEvent)
}

impl PacketEncode for Sound {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        match (self) {
            Sound::Registry(entry) => {
                buf.encode_write(VarInt::from((entry.id() as i32) + 1))?;
            },
            Sound::Inline(sound) => {
                buf.encode_write(VarInt::from(0))?;
                buf.encode_write(sound)?;
            }
        }
        Ok(())
    }
}
impl PacketDecode for Sound {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let id = buf.read_decode::<VarInt>()?.as_i32() as u32;
        if (id == 0) {
            Ok(Self::Inline(buf.read_decode::<SoundEvent>()?))
        } else {
            Ok(Self::Registry(unsafe{ RegEntry::new_unchecked(id - 1) }))
        }
    }
}
