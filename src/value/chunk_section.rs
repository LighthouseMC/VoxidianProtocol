use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChunkSection {
    pub x : i32,
    pub y : i32,
    pub z : i32
}
impl ChunkSection {
    pub fn new(x : i32, y : i32, z : i32) -> Self { Self { x, y, z } }
}impl fmt::Debug for ChunkSection { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ChunkSection({}, {}, {})", self.x, self.y, self.z)
} }


impl PacketEncode for ChunkSection { fn encode(&self, buf : &mut super::PacketBuf) -> Result<(), super::EncodeError> {
    buf.encode_write(
        (((self.x as i64) & 0x3fffff) << 42)
        | (((self.z as i64) & 0x3ffff) << 20)
        | ((self.y as i64) & 0xfffff)
    )?;
    Ok(())
} }
impl PacketDecode for ChunkSection { fn decode(buf : &mut super::PacketBuf) -> Result<Self, super::DecodeError> {
    let long = buf.read_decode::<u64>()?;
    Ok(Self {
        x : (long >> 42) as i32,
        z : (long << 22 >> 42) as i32,
        y : (long << 44 >> 44) as i32
    })
} }
