use crate::value::*;
use voxidian_protocol_macros::{component_enum, packet_part};

mod components;
pub use components::*;

#[derive(Debug)]
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
            // TODO: write component
        }

        for component in &self.removed_components {
            // TODO: write removed component
        }
        todo!();

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

        todo!()
    }
}

pub trait ComponentData: PacketEncode + PacketDecode {
    const ID: u32;
}

component_enum!();