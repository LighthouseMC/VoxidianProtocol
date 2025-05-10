use crate::packet::*;
use std::hash::Hash;
use std::marker::PhantomData;
use std::{ fmt, any };


pub struct RegEntry<T> {
    id  : u32,
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
    pub const unsafe fn new_unchecked(id : u32) -> Self { Self {
        id, _ph : PhantomData
    } }

    /// Obtains the raw numeric ID this RegEntry would link to.
    pub fn id(&self) -> u32 { self.id }

    /// Looks up this RegEntry in a provided registry.
    pub fn lookup<'r>(&self, registry : &'r Registry<T>) -> Option<&'r T> {
        registry.lookup(self)
    }
}
impl<T> Clone for RegEntry<T> {
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self { Self {
        id  : self.id,
        _ph : PhantomData
    } }
}
impl<T> Hash for RegEntry<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl<T> Copy for RegEntry<T> { }
impl<T> PartialEq for RegEntry<T> {
    fn eq(&self, other : &Self) -> bool {
        self.id == other.id
    }
}
impl<T> Eq for RegEntry<T> { }
impl<T> fmt::Debug for RegEntry<T> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegEntry<{}>({})", any::type_name::<T>(), self.id)
    }
}


impl<T> PacketEncode for RegEntry<T> { fn encode(&self, buf : &mut PacketWriter) -> Result<(), crate::packet::EncodeError> {
    buf.encode_write(Var32::from(self.id as i32))
} }
impl<T> PacketDecode for RegEntry<T> { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    Ok(unsafe{ Self::new_unchecked(buf.read_decode::<Var32>()?.as_i32() as u32) })
} }
