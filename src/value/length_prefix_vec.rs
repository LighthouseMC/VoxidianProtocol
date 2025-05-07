use super::*;

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };


#[derive(Clone)]
pub struct LengthPrefixVec<Idx : From<usize> + Into<usize>, T> {
    inner : Vec<T>,
    _ph   : PhantomData<Idx>
}
impl<Idx : From<usize> + Into<usize>, T> LengthPrefixVec<Idx, T> {

    pub fn new() -> Self { Self {
        inner : Vec::new(),
        _ph   : PhantomData
    } }

    pub fn into_inner(self) -> Vec<T> { self.inner }

}
impl<Idx : From<usize> + Into<usize>, T> From<Vec<T>> for LengthPrefixVec<Idx, T> {
    fn from(value : Vec<T>) -> Self { Self {
        inner : value,
        _ph   : PhantomData
    } }
}
impl<Idx : From<usize> + Into<usize>, T : fmt::Debug> fmt::Debug for LengthPrefixVec<Idx, T> { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LengthPrefixVec{:?}", self.inner)
} }

impl<Idx : From<usize> + Into<usize>, T> Default for LengthPrefixVec<Idx, T> { fn default() -> Self { Self::new() } }

impl<Idx : From<usize> + Into<usize>, T> Deref for LengthPrefixVec<Idx, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<Idx : From<usize> + Into<usize>, T> DerefMut for LengthPrefixVec<Idx, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<Idx : From<usize> + Into<usize> + PacketEncode, T : PacketEncode> PacketEncode for LengthPrefixVec<Idx, T> {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(Idx::from(self.inner.len()))?;
        for item in &self.inner {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<Idx : From<usize> + Into<usize> + PacketDecode, T : PacketDecode> PacketDecode for LengthPrefixVec<Idx, T> {
    fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        let len = buf.read_decode::<Idx>()?.into();
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

impl<Idx : From<usize> + Into<usize>, T : PartialEq> PartialEq for LengthPrefixVec<Idx, T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
