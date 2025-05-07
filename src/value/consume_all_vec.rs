use super::*;

use std::ops::{ Deref, DerefMut };


#[derive(Clone, PartialEq, Eq)]
pub struct ConsumeAllVec<T> {
    inner : Vec<T>
}
impl<T> ConsumeAllVec<T> {

    pub fn new() -> Self { Self {
        inner : Vec::new()
    } }

    pub fn into_inner(self) -> Vec<T> { self.inner }

}
impl<T> From<Vec<T>> for ConsumeAllVec<T> {
    fn from(value : Vec<T>) -> Self { Self {
        inner : value
    } }
}
impl<T : fmt::Debug> fmt::Debug for ConsumeAllVec<T> { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ConsumeAllVec{:?}", self.inner)
} }
impl<T> Default for ConsumeAllVec<T> { fn default() -> Self { Self::new() } }

impl<T> Deref for ConsumeAllVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<T> DerefMut for ConsumeAllVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<T : PacketEncode> PacketEncode for ConsumeAllVec<T> {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        for item in &self.inner {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<'l, T : PacketDecode<'l>> PacketDecode<'l> for ConsumeAllVec<T> {
    fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        let mut items = Vec::new();
        loop { match (buf.read_decode::<T>()) {
            Ok(item) => { items.push(item); },
            Err(DecodeError::EndOfBuffer) => { break; }
            Err(e) => { return Err(e); }
        } }
        Ok(Self { inner : items })
    }
}
