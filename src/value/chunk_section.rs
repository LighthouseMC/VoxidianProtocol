use std::vec;

use voxidian_protocol_macros::packet_part;
use super::*;

#[derive(Debug)]
pub struct DataArray {
    pub bits_per_entry: u8,
    pub input_data: Vec<u32>,
}

impl DataArray {
    pub fn to_bit_stream(&self) -> Vec<u64> {
        todo!()
    }
}

impl PacketEncode for DataArray {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        let compressed = self.to_bit_stream();
        VarInt::from(compressed.len()).encode(buf)?;
        for long in &compressed {
            buf.encode_write(*long)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ChunkSectionData {
    pub sections: Vec<ChunkSection>
}

impl PacketEncode for ChunkSectionData {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        let mut subbuf = PacketBuf::new();
        for element in &self.sections {
            element.encode(&mut subbuf)?;
        }
        let vector = subbuf.iter().collect::<Vec<u8>>();
        LengthPrefixVec::<VarInt, _>::from(vector).encode(buf)?;
        Ok(())
    }
}

impl PacketDecode for ChunkSectionData {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}

#[packet_part]
pub struct ChunkSection {
    pub block_count: i16,
    pub block_states: PalettedContainer<BlockState, 4096>,
    pub biomes: PalettedContainer<Biome, 64>,
}

#[derive(Debug)]
pub struct PalettedContainer<T, const E: usize> {
    pub bits_per_entry: u8,
    pub format: PaletteFormat<T, E>
}

impl<T, const E: usize> PacketEncode for PalettedContainer<T, E> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        self.bits_per_entry.encode(buf)?;
        self.format.encode(buf)?;
        self.format.to_data_array(self.bits_per_entry).encode(buf)?;
        Ok(())
    }
}

impl<T, const E: usize> PacketDecode for PalettedContainer<T, E> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum PaletteFormat<T, const E: usize> {
    SingleValued { entry: RegEntry<T> },
    Indirect { mappings: Vec<RegEntry<T>>, data: [usize; E] },
    Direct { data: [RegEntry<T>; E] }
}

impl<T, const E: usize> PaletteFormat<T, E> {
    pub fn to_data_array(&self, bits_per_entry: u8) -> DataArray {
        match self {
            PaletteFormat::SingleValued { entry } => {
                assert!(bits_per_entry == 0);
                DataArray {
                    bits_per_entry,
                    input_data: vec![]
                }
            },
            PaletteFormat::Indirect { mappings, data } => {
                assert!(bits_per_entry >= 1 && bits_per_entry < 15);
                todo!()
            },
            PaletteFormat::Direct { data } => {
                assert!(bits_per_entry == 15);
                let input_data = data.iter().map(|entry| entry.id() as u32).collect();
                DataArray {
                    bits_per_entry,
                    input_data
                }
            },
        }
    }
}

impl<T, const E: usize> PacketEncode for PaletteFormat<T, E> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        match self {
            PaletteFormat::SingleValued { entry } => {
                VarInt::from(entry.id()).encode(buf)?;
                Ok(())
            },
            PaletteFormat::Indirect { mappings, data } => {
                VarInt::from(mappings.len() as i32).encode(buf)?;
                for entry in mappings {
                    VarInt::from(entry.id()).encode(buf)?;
                }
                Ok(())
            },
            PaletteFormat::Direct { data } => { Ok(()) },
        }
    }
}

impl<T, const E: usize> PacketDecode for PaletteFormat<T, E> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}