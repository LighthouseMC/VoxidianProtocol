use super::*;
use std::f32::consts::TAU;


#[derive(Clone, Copy, PartialEq)]
pub struct Angle {
    frac : f32
}
impl Angle {

    pub fn of_frac(frac : f32) -> Self { Self { frac : frac.rem_euclid(1.0) } }
    pub fn as_frac(&self) -> f32 { self.frac }

    pub fn of_rad(rad : f32) -> Self { Self { frac : rad / TAU } }
    pub fn as_rad(&self) -> f32 { self.frac * TAU }

    pub fn of_deg(deg : f32) -> Self { Self { frac : deg / 360.0 } }
    pub fn as_deg(&self) -> f32 { self.frac * 360.0 }

}
impl fmt::Debug for Angle { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Angle({}/1.0)", self.frac)
} }


impl PacketEncode for Angle { fn encode(&self, buf : &mut super::PacketBuf) -> Result<(), super::EncodeError> {
    buf.write_u8((self.frac * 256.0) as u8);
    Ok(())
} }
impl PacketDecode for Angle { fn decode(buf : &mut super::PacketBuf) -> Result<Self, super::DecodeError> {
    Ok(Self { frac : (buf.read_u8()? as f32) / 256.0 })
} }
