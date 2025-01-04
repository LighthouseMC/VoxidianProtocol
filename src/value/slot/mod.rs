
mod components;
pub use components::*;

use crate::value::*;

#[derive(Debug, Clone)]
pub struct SlotData {
    pub item_count: VarInt,
    pub item_id: RegEntry<Item>,
    pub components: Vec<DataComponents>,
    pub removed_components: Vec<DataComponentTypes>
}

impl PacketEncode for SlotData {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.encode_write(self.item_count)?;
        if self.item_count.as_i32() < 0 {
            return Ok(());
        }

        buf.encode_write(self.item_id)?;

        buf.encode_write(VarInt::from(self.components.len()))?;
        buf.encode_write(VarInt::from(self.removed_components.len()))?;

        for component in &self.components {
            buf.encode_write(component)?;
        }

        for component in &self.removed_components {
            buf.encode_write(component)?;
        }
        todo!();

        #[allow(unreachable_code)]
        Ok(())
    }
}

impl PacketDecode for SlotData {
    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        let item_count = buf.read_decode::<VarInt>()?;
        if item_count.as_i32() == 0 {
            return Ok(SlotData {
                item_count: VarInt::from(0),
                item_id: unsafe { RegEntry::new_unchecked(0) },
                components: Vec::new(),
                removed_components: Vec::new(),
            });
        }

        let item_id = buf.read_decode::<RegEntry<Item>>()?;
        let components_len = buf.read_decode::<VarInt>()?;
        let removed_components_len = buf.read_decode::<VarInt>()?;

        let mut components = Vec::with_capacity(components_len.as_i32() as usize);
        for _ in 0..components_len.as_i32() {
            components.push(buf.read_decode::<DataComponents>()?);
        }

        let mut removed_components = Vec::with_capacity(removed_components_len.as_i32() as usize);
        for _ in 0..removed_components_len.as_i32() {
            removed_components.push(buf.read_decode::<DataComponentTypes>()?);
        }

        Ok(SlotData {
            item_count,
            item_id,
            components,
            removed_components,
        })
    }
}

pub trait ComponentData: PacketEncode + PacketDecode {
    const ID: u32;
}
