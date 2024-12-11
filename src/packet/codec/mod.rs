mod encode;
pub use encode::*;
mod decode;
pub use decode::*;

use super::{PacketBuf, VarInt};

pub trait PacketEncodeDecode: Sized + PacketEncode + PacketDecode {
}
