use super::*;


#[derive(Clone, Debug)]
pub struct Particle {
}


impl PacketEncode for Particle { fn encode(&self, _buf : &mut PacketBuf) -> Result<(), EncodeError> {
    todo!("PacketEncode for Particle")
} }
impl PacketDecode for Particle { fn decode(_buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    todo!("PacketDecode for Particle")
} }
