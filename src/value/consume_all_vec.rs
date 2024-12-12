use super::*;

use std::ops::{ Deref, DerefMut };


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsumeAllVec<T> {
    inner : Vec<T>
}
impl<T> ConsumeAllVec<T> {

    pub fn new(self) -> Self { Self {
        inner : Vec::new()
    } }

    pub fn into_inner(self) -> Vec<T> { self.inner }

}
impl<T> From<Vec<T>> for ConsumeAllVec<T> {
    fn from(value : Vec<T>) -> Self { Self {
        inner : value
    } }
}

impl<T> Deref for ConsumeAllVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<T> DerefMut for ConsumeAllVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<T : PacketEncode> PacketEncode for ConsumeAllVec<T> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        for item in &self.inner {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<T : PacketDecode> PacketDecode for ConsumeAllVec<T> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let mut items = Vec::new();
        loop { match (buf.read_decode::<T>()) {
            Ok(item) => { items.push(item); },
            Err(DecodeError::EndOfBuffer) => { break; }
            Err(e) => { return Err(e); }
        } }
        Ok(Self { inner : items })
    }
}
