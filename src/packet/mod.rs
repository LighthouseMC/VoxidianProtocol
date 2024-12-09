mod codec;
pub use codec::*;
mod buffer;
pub use buffer::*;

pub mod c2s;
pub mod s2c;


use crate::value::VarInt;
use voxidian_protocol_macros::packet;
