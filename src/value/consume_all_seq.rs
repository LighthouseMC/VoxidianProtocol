use super::*;

use std::ops::{ Deref, DerefMut };


#[derive(Clone, PartialEq, Eq)]
pub struct ConsumeAllSeq<'l, T> {
    inner : Cow<'l, [T]>
}
impl<'l, T> ConsumeAllSeq<'l, T> {

    pub fn new() -> Self { Self {
        inner : Vec::new()
    } }

    pub fn into_inner(self) -> Vec<T> { self.inner }

}
impl<'l, T> From<Vec<T>> for ConsumeAllSeq<'l, T> {
    fn from(value : Vec<T>) -> Self { Self {
        inner : value
    } }
}
impl<'l, T : fmt::Debug> fmt::Debug for ConsumeAllSeq<'l, T> { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ConsumeAllSeq{:?}", self.inner)
} }
impl<'l, T> Default for ConsumeAllSeq<'l, T> { fn default() -> Self { Self::new() } }

impl<'l, T> Deref for ConsumeAllSeq<'l, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<'l, T> DerefMut for ConsumeAllSeq<'l, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<'l, T : PacketEncode> PacketEncode for ConsumeAllSeq<'l, T> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        for item in &self.inner {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<'l, T : PacketDecode> PacketDecode for ConsumeAllSeq<'l, T> {
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
