use super::*;

#[derive(Debug, Clone)]
pub struct ParticleInstance {
    pub base: RegEntry<ParticleType>,
    pub data: ParticleData,
}

impl PacketEncode for ParticleInstance {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(&self.base)?;
        buf.encode_write(&self.data)
    }
}

impl PacketDecode for ParticleInstance {
    fn decode<'l>(_buf: &mut PacketReader<'l>) -> Result<Self, crate::packet::DecodeError> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum ParticleData {
    None,
    Block(RegEntry<BlockState>),
    Dust {
        color: Colour,
        scale: f32,
    },
    DustColorTransition {
        from_color: Colour,
        to_color: Colour,
        scale: f32,
    },
    EntityEffect {
        color: Colour,
    },
    SculkCharge {
        roll: f32,
    },
    Item {
        item: SlotData,
    },
    Vibration {
        position_source_type: VibrationPositionSource,
        ticks: Var32,
    },
    Shriek {
        delay: Var32,
    },
}

impl PacketEncode for ParticleData {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), crate::packet::EncodeError> {
        match self {
            ParticleData::None => {}
            ParticleData::Block(reg_entry) => {
                buf.encode_write(reg_entry)?;
            }
            ParticleData::Dust { color, scale } => {
                buf.encode_write(color.to_int())?;
                buf.encode_write(scale)?;
            }
            ParticleData::DustColorTransition {
                from_color,
                to_color,
                scale,
            } => {
                buf.encode_write(from_color.to_int())?;
                buf.encode_write(to_color.to_int())?;
                buf.encode_write(scale)?;
            }
            ParticleData::EntityEffect { color } => {
                buf.encode_write(color.to_int())?;
            }
            ParticleData::SculkCharge { roll } => {
                buf.encode_write(roll)?;
            }
            ParticleData::Item { item } => {
                buf.encode_write(item)?;
            }
            ParticleData::Vibration {
                position_source_type,
                ticks,
            } => {
                buf.encode_write(position_source_type)?;
                buf.encode_write(ticks)?;
            }
            ParticleData::Shriek { delay } => {
                buf.encode_write(delay)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum VibrationPositionSource {
    BlockPosition(BlockPos),
    Entity { id: Var32, eye_height: f32 },
}

impl PacketEncode for VibrationPositionSource {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), crate::packet::EncodeError> {
        match self {
            VibrationPositionSource::BlockPosition(block_pos) => {
                buf.write_u8(0);
                buf.encode_write(block_pos)?;
            }
            VibrationPositionSource::Entity { id, eye_height } => {
                buf.write_u8(1);
                buf.encode_write(id)?;
                buf.encode_write(eye_height)?;
            }
        }
        Ok(())
    }
}
