mod block;
pub use block::*;
mod block_state;
pub use block_state::*;
mod dimension_type;
pub use dimension_type::*;
mod entity_type;
pub use entity_type::*;
mod item;
pub use item::*;


use crate::packet::*;
use std::marker::PhantomData;
use std::{ fmt, any };


#[derive()]
pub struct RegEntry<T> {
    id  : usize,
    _ph : PhantomData<T>
}
impl<T> RegEntry<T> {

    /// Seriously consider what you're doing before using this function.
    pub unsafe fn new_unchecked(id : usize) -> Self { Self {
        id, _ph : PhantomData
    } }

    #[deprecated = "Chances are, you shouldn't be using the underlying raw id of a RegEntry"]
    pub fn id(&self) -> usize { self.id }

}
impl<T> Clone for RegEntry<T> {
    fn clone(&self) -> Self { Self {
        id  : self.id,
        _ph : PhantomData
    } }
}
impl<T> Copy for RegEntry<T> { }
impl<T> fmt::Debug for RegEntry<T> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegEntry<{}>({})", any::type_name::<T>(), self.id)
    }
}


impl<T> PacketEncode for RegEntry<T> { fn encode(&self, buf : &mut crate::packet::PacketBuf) -> Result<(), crate::packet::EncodeError> {
    buf.encode_write(VarInt::from(self.id as i32))
} }
impl<T> PacketDecode for RegEntry<T> { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    Ok(unsafe{ Self::new_unchecked(buf.read_decode::<VarInt>()?.as_i32() as usize) })
} }
