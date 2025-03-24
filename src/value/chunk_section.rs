use super::*;
use voxidian_protocol_macros::packet_part;

#[derive(Debug, Clone)]
pub struct ChunkSectionData {
    pub sections: Vec<ChunkSection>,
}

impl PacketEncode for ChunkSectionData {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
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
    fn decode(_buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}

#[packet_part]
pub struct ChunkSection {
    pub block_count: i16,
    pub block_states: PalettedContainer<BlockState, 4096>,
    pub biomes: PalettedContainer<Biome, 64>,
}

#[derive(Debug, Clone)]
pub struct PalettedContainer<T, const E: usize> {
    pub bits_per_entry: u8,
    pub format: PaletteFormat<T, E>,
}

impl<T, const E: usize> PacketEncode for PalettedContainer<T, E> {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        self.bits_per_entry.encode(buf)?;
        self.format.encode(buf)?;
        let PaletteFormat::RawDirect { data: _ } = self.format else {
            let arr = self.format.to_data_array(self.bits_per_entry);
            for item in arr.inner().iter() {
                buf.encode_write(item)?;
            }

            return Ok(());
        };

        Ok(())
    }
}

impl<T, const E: usize> PacketDecode for PalettedContainer<T, E> {
    fn decode(_buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub enum PaletteFormat<T, const E: usize> {
    SingleValued {
        entry: RegEntry<T>,
    },
    Indirect {
        mappings: Vec<RegEntry<T>>,
        data: [usize; E],
    },
    Direct {
        data: [RegEntry<T>; E],
    },

    RawDirect {
        data: RawDataArray,
    },
}

impl<T, const E: usize> PaletteFormat<T, E> {
    pub fn to_data_array(&self, bits_per_entry: u8) -> RawDataArray {
        match self {
            PaletteFormat::SingleValued { entry: _entry } => {
                assert!(bits_per_entry == 0);
                RawDataArray::new(0)
            }
            PaletteFormat::Indirect {
                mappings: _,
                data: _,
            } => {
                assert!(bits_per_entry >= 1);
                todo!()
            }
            PaletteFormat::Direct { data } => {
                let mut arr = RawDataArray::new(15);
                for entry in data {
                    arr.push(entry.id() as u64);
                }
                arr
            }
            PaletteFormat::RawDirect { data: _ } => {
                panic!("use encode directly please")
            }
        }
    }
}

impl<T, const E: usize> PacketEncode for PaletteFormat<T, E> {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        match self {
            PaletteFormat::SingleValued { entry } => {
                VarInt::from(entry.id() as i32).encode(buf)?;
            }
            PaletteFormat::Indirect { mappings, data: _ } => {
                VarInt::from(mappings.len()).encode(buf)?;
                for entry in mappings {
                    VarInt::from(entry.id() as i32).encode(buf)?;
                }
            }
            PaletteFormat::Direct { data: _ } => {}
            PaletteFormat::RawDirect { data } => {
                for item in data.inner() {
                    buf.encode_write(item)?;
                }
            }
        }
        Ok(())
    }
}

impl<T, const E: usize> PacketDecode for PaletteFormat<T, E> {
    fn decode(_buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        unimplemented!()
    }
}
