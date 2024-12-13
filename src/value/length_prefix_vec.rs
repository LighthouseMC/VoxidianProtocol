use super::*;

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LengthPrefixVec<Index : From<usize> + Into<usize>, T> {
    inner : Vec<T>,
    _ph   : PhantomData<Index>
}
impl<Index : From<usize> + Into<usize>, T> LengthPrefixVec<Index, T> {

    pub fn new() -> Self { Self {
        inner : Vec::new(),
        _ph   : PhantomData
    } }

    pub fn into_inner(self) -> Vec<T> { self.inner }

}
impl<Index : From<usize> + Into<usize>, T> From<Vec<T>> for LengthPrefixVec<Index, T> {
    fn from(value : Vec<T>) -> Self { Self {
        inner : value,
        _ph   : PhantomData
    } }
}

impl<Index : From<usize> + Into<usize>, T> Deref for LengthPrefixVec<Index, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<Index : From<usize> + Into<usize>, T> DerefMut for LengthPrefixVec<Index, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<Index : From<usize> + Into<usize> + PacketEncode, T : PacketEncode> PacketEncode for LengthPrefixVec<Index, T> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.encode_write(Index::from(self.inner.len()))?;
        for item in &self.inner {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<Index : From<usize> + Into<usize> + PacketDecode, T : PacketDecode> PacketDecode for LengthPrefixVec<Index, T> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let len = buf.read_decode::<Index>()?.into();
        let mut items = Vec::new();
        for _ in 0..len {
            items.push(buf.read_decode::<T>()?);
        }
        Ok(Self {
            inner : items,
            _ph   : PhantomData
        })
    }
}
