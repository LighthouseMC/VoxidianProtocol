use super::*;


#[derive(Clone, Copy, PartialEq)]
pub struct Vec3d {
    pub x : f64,
    pub y : f64,
    pub z : f64
}
impl Vec3d {
    pub fn new(x : f64, y : f64, z : f64) -> Self { Self { x, y, z } }
}
impl fmt::Debug for Vec3d { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Vec3d({}, {}, {})", self.x, self.y, self.z)
} }


impl PacketEncode for Vec3d { fn encode(&self, buf : &mut super::PacketBuf) -> Result<(), super::EncodeError> {
    buf.encode_write(self.x)?;
    buf.encode_write(self.y)?;
    buf.encode_write(self.z)?;
    Ok(())
} }
impl PacketDecode for Vec3d { fn decode(buf : &mut super::PacketBuf) -> Result<Self, super::DecodeError> {
    Ok(Self {
        x : buf.read_decode::<f64>()?,
        y : buf.read_decode::<f64>()?,
        z : buf.read_decode::<f64>()?
    })
} }
