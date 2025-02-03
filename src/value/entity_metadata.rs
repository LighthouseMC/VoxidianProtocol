use std::collections::HashMap;

use crate::packet::{PacketEncode, RegEntry, TODO};

use super::{
    BlockState, Identifier, Nbt, OptionVarInt, PaintingVariant, RegOr, SlotData, TextComponent,
    VarInt, VarLong, WolfVariant,
};

pub struct EntityMetadata {
    inner: HashMap<u8, MetadataEntry>,
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
    fn encode(&self, buf: &mut crate::packet::PacketBuf) -> Result<(), crate::packet::EncodeError> {
        for (entry, value) in self.inner.iter() {
            buf.write_u8(*entry);
            buf.encode_write(value)?;
        }
        buf.write_u8(0xff);
        Ok(())
    }
}

pub enum MetadataEntry {
    Byte(u8),
    VarInt(VarInt),
    VarLong(VarLong),
    Float(f32),
    String(String),
    TextComponent(TextComponent),
    OptionalTextComponent(Option<TextComponent>),
    SlotData(SlotData),
    Rotations(f32, f32, f32),
    Position(TODO),
    OptionalPosition(Option<TODO>),
    Direction(VarInt),
    BlockState(BlockState),
    OptionalBlockState(Option<BlockState>),
    Nbt(Nbt),
    Particle(VarInt, TODO),
    Particles(TODO),
    OptionalVarInt(OptionVarInt),
    Pose(VarInt),
    CatVariant(RegEntry<TODO>),
    WolfVariant(RegEntry<WolfVariant>),
    FrogVariant(RegEntry<TODO>),
    OptionalGlobalPosition(Option<(Identifier, TODO)>),
    PaintingVariant(RegOr<PaintingVariant, PaintingVariant>),
    SnifferState(VarInt),
    ArmadilloState(VarInt),
    Vector3(f32, f32, f32),
    Quaternion(f32, f32, f32, f32),
}

impl PacketEncode for MetadataEntry {
    fn encode(&self, buf: &mut crate::packet::PacketBuf) -> Result<(), crate::packet::EncodeError> {
        todo!()
    }
}
