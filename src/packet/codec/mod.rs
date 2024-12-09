mod encode;
pub use encode::*;
mod decode;
pub use decode::*;


use super::{ PacketBuf, VarInt };


pub trait PacketEncodeDecode : Sized {

    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError>;

    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError>;

}

impl<T : PacketEncode + PacketDecode> PacketEncodeDecode for T {

    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> { PacketEncode::encode(self, buf) }

    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> { PacketDecode::decode(buf) }

}
