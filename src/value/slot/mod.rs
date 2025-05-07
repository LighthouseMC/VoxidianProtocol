
mod components;
pub use components::*;

use crate::value::*;

#[derive(Debug, Clone)]
pub struct SlotData {
    pub id                 : RegEntry<Item>,
    pub count              : VarInt,
    pub components         : Vec<DataComponents>,
    pub removed_components : Vec<DataComponentTypes>
}

impl SlotData {

    pub const EMPTY : Self = Self {
        id                 : unsafe{ RegEntry::new_unchecked(0) },
        count              : VarInt::new(0),
        components         : Vec::new(),
        removed_components : Vec::new()
    };

}

impl PacketEncode for SlotData {

    fn encode(&self, buf: &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(self.count)?;
        if (self.count.as_i32() <= 0) {
            return Ok(());
        }

        buf.encode_write(self.id)?;

        buf.encode_write(VarInt::from(self.components.len()))?;
        buf.encode_write(VarInt::from(self.removed_components.len()))?;

        for component in &self.components {
            buf.encode_write(component)?;
        }

        for component in &self.removed_components {
            buf.encode_write(component)?;
        }

        Ok(())
    }
}

impl PacketDecode for SlotData {
    fn decode<'l>(buf: &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        let item_count = buf.read_decode::<VarInt>()?;
        if item_count.as_i32() == 0 {
            return Ok(SlotData {
                count: VarInt::from(0),
                id: unsafe { RegEntry::new_unchecked(0) },
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
            count: item_count,
            id: item_id,
            components,
            removed_components,
        })
    }
}

impl PartialEq for SlotData {
    fn eq(&self, other : &Self) -> bool {
        let self_empty  = (self  .id.id() == 0) || (self  .count.as_i32() == 0);
        let other_empty = (other .id.id() == 0) || (other .count.as_i32() == 0);
        match (self_empty, other_empty) {
            (true, true) => true,
            (false, false) => {
                if (self.id != other.id) { return false; }
                if (self.count != other.count) { return false; }
                if (self.components.len() != other.components.len()) { return false; }
                if (self.removed_components.len() != other.removed_components.len()) { return false; }
                for component in &self.components {
                    if (! other.components.contains(component)) { return false; }
                }
                for component in &self.removed_components {
                    if (! other.removed_components.contains(component)) { return false; }
                }
                true
            },
            _ => false
        }
    }
}
impl Eq for SlotData {}

pub trait ComponentData : PacketEncode + PacketDecode {
    const ID: u32;
}
