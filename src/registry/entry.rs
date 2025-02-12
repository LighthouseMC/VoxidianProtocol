use crate::packet::*;
use std::marker::PhantomData;
use std::{ fmt, any };


pub struct RegEntry<T> {
    id  : usize,
    _ph : PhantomData<T>
}
impl<T> RegEntry<T> {

    /// Creates a new RegEntry from a numeric ID into a registry.
    ///
    /// # Safety
    /// This function may do undefined behavior if not checked that the RegEntry
    /// is a valid entry into the provided registry when sending it to a client.
    /// Worst case scenario, sending a client an invalid registry entry may
    /// kick them from the server unexpectedly.
    /// This can also lead to invalid references to the packet registry.
    /// While misusing this function won't lead to any memory safety errors, it can lead to
    /// undesired behavior very easily.
    pub unsafe fn new_unchecked(id : usize) -> Self { Self {
        id, _ph : PhantomData
    } }

    /// Obtains the raw numeric ID this RegEntry would link to.
    pub fn id(&self) -> usize { self.id }

    /// Looks up this RegEntry in a provided registry.
    pub fn lookup<'r>(&self, registry : &'r Registry<T>) -> Option<&'r T> {
        registry.lookup(self)
    }
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
