use super::{PacketDecode, PacketEncode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    pub x : i32,
    pub y : i32,
    pub z : i32
}
impl BlockPos {
    pub fn new(x : i32, y : i32, z : i32) -> Self { Self { x, y, z } }
}


impl PacketEncode for BlockPos { fn encode(&self, buf : &mut super::PacketBuf) -> Result<(), super::EncodeError> {
    buf.encode_write(
        (((self.x as i64) & 0x3ffffff) << 38)
        | (((self.z as i64) & 0x3fffff) << 12)
        | ((self.y as i64) & 0xfff)
    )?;
    Ok(())
} }
impl PacketDecode for BlockPos { fn decode(buf : &mut super::PacketBuf) -> Result<Self, super::DecodeError> {
    let long = buf.read_decode::<u64>()?;
    Ok(Self {
        x : (long >> 38) as i32,
        z : (long << 26 >> 38) as i32,
        y : (long << 52 >> 52) as i32
    })
} }