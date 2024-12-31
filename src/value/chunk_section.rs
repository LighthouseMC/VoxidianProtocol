use std::vec;

use voxidian_protocol_macros::packet_part;
use super::*;

#[derive(Debug)]
pub struct DataArray {
    pub bits_per_entry: usize,
    pub input_data: Vec<u64>
}

impl DataArray {
    pub fn to_bit_stream(&self) -> Vec<u64> {
        let entries_per_long = 64 / self.bits_per_entry;
        let needed_longs = (self.input_data.len() + entries_per_long - 1) / entries_per_long;

        let mut output_data = vec![0; needed_longs];
        
        for (idx, value) in self.input_data.iter().enumerate() {
            let u64_index = idx / entries_per_long;
            let bit_index = (idx % entries_per_long) * self.bits_per_entry;

            output_data[u64_index] &= !(((1 << self.bits_per_entry) - 1) << bit_index);
            output_data[u64_index] |= value << bit_index;
        }
        output_data
    }
}

#[cfg(test)]
mod test {
    use super::DataArray;

    #[test]
    pub fn check_data_array() {
        let arr = DataArray {
            bits_per_entry: 5,
            input_data: vec![1,2,2,3,4,4,5,6,6,4,8,0,7,4,3,13,15,16,9,14,10,12,0,2],
        };
        assert_eq!(arr.to_bit_stream(), vec![0x0020863148418841, 0x01018A7260F68C87])
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
                    bits_per_entry: bits_per_entry.into(),
                    input_data: vec![]
                }
            },
            PaletteFormat::Indirect { mappings, data } => {
                assert!(bits_per_entry >= 1 && bits_per_entry < 15);
                todo!()
            },
            PaletteFormat::Direct { data } => {
                assert!(bits_per_entry == 15);
                let input_data = data.iter().map(|entry| entry.id() as u64).collect();
                DataArray {
                    bits_per_entry: bits_per_entry.into(),
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