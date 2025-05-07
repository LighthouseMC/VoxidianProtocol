use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChunkSectionPosition {
    pub x : i32,
    pub y : i32,
    pub z : i32
}
impl ChunkSectionPosition {
    pub fn new(x : i32, y : i32, z : i32) -> Self { Self { x, y, z } }
}impl fmt::Debug for ChunkSectionPosition { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ChunkSection({}, {}, {})", self.x, self.y, self.z)
} }


impl PacketEncode for ChunkSectionPosition { fn encode(&self, buf : &mut PacketWriter) -> Result<(), super::EncodeError> {
    buf.encode_write(
        (((self.x as i64) & 0x3fffff) << 42)
        | (((self.z as i64) & 0x3ffff) << 20)
        | ((self.y as i64) & 0xfffff)
    )?;
    Ok(())
} }
impl<'l> PacketDecode<'l> for ChunkSectionPosition { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, super::DecodeError> {
    let long = buf.read_decode::<u64>()?;
    Ok(Self {
        x : (long >> 42) as i32,
        z : (long << 22 >> 42) as i32,
        y : (long << 44 >> 44) as i32
    })
} }
