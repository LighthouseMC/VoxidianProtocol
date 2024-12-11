mod encode;
pub use encode::*;
mod decode;
pub use decode::*;

use super::*;

pub trait PacketEncodeDecode: Sized + PacketEncode + PacketDecode { }
