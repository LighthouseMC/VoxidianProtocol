use super::*;
use std::collections::HashMap;

use super::{
    BlockState, Identifier, Nbt, OptionVarInt, PaintingVariant, SlotData, TextComponent, Var32,
    Var64, WolfVariant,
};

#[derive(Debug, Clone)]
pub struct EntityMetadata {
    inner: HashMap<u8, MetadataEntry>,
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityMetadata {
    pub fn new() -> EntityMetadata {
        EntityMetadata {
            inner: HashMap::new(),
        }
    }

    pub fn insert_raw_entry(&mut self, index: u8, entry: MetadataEntry) {
        self.inner.insert(index, entry);
    }
}

impl PacketEncode for EntityMetadata {
    fn encode(&self, buf: &mut PacketWriter) -> Result<(), crate::packet::EncodeError> {
        for (entry, value) in self.inner.iter() {
            buf.write_u8(*entry);
            buf.encode_write(value)?;
        }
        buf.write_u8(0xff);
        Ok(())
    }
}

macro_rules! create_metadata_entries {
    (
        $($index:expr => $name:ident($type:ty)),*$(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum MetadataEntry {
            $($name($type)),*
        }

        impl PacketEncode for MetadataEntry {
            fn encode(&self, buf: &mut PacketWriter) -> Result<(), crate::packet::EncodeError> {
                match self {
                    $(
                        MetadataEntry::$name( value ) => {
                            buf.write_u8($index);
                            buf.encode_write(value)?;
                        }
                    )*
                }
                Ok(())
            }
        }

    };
}

impl PacketDecode for MetadataEntry {
    fn decode<'l>(_buf: &mut PacketReader<'l>) -> Result<Self, crate::packet::DecodeError> {
        todo!()
    }
}

impl PacketDecode for EntityMetadata {
    fn decode<'l>(_buf: &mut PacketReader<'l>) -> Result<Self, crate::packet::DecodeError> {
        todo!()
    }
}

create_metadata_entries! {
    0x00 => Byte(u8),
    0x01 => VarInt(Var32),
    0x02 => VarLong(Var64),
    0x03 => Float(f32),
    0x04 => String(String),
    0x05 => TextComponent(TextComponent),
    0x06 => OptionalTextComponent(Option<TextComponent>),
    0x07 => SlotData(SlotData),
    0x08 => Rotations((f32, f32, f32)),
    0x09 => Position(TODO),
    0x0A => OptionalPosition(Option<TODO>),
    0x0B => Direction(Var32),
    0x0C => BlockState(RegEntry<BlockState>),
    0x0D => OptionalBlockState(OptionVarInt),
    0x0E => Nbt(Nbt),
    0x0F => Particle((Var32, TODO)),
    0x10 => Particles(TODO),
    0x11 => OptionalVarInt(OptionVarInt),
    0x12 => Pose(Var32),
    0x13 => CatVariant(RegEntry<TODO>),
    0x14 => WolfVariant(RegEntry<WolfVariant>),
    0x15 => FrogVariant(RegEntry<TODO>),
    0x16 => OptionalGlobalPosition(Option<(Identifier, TODO)>),
    0x17 => PaintingVariant(RegEntry<PaintingVariant>),
    0x18 => SnifferState(Var32),
    0x19 => ArmadilloState(Var32),
    0x1A => Vector3((f32, f32, f32)),
    0x1B => Quaternion((f32, f32, f32, f32)),
}
