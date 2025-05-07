mod encode;
pub use encode::*;
mod decode;
pub use decode::*;
mod ordinal;
pub use ordinal::*;

use super::*;

pub trait PacketEncodeDecode<'l> : Sized + PacketEncode + PacketDecode<'l> { }
