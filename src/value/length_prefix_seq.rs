use super::*;

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };


#[derive(Clone, PartialEq, Eq)]
pub struct LengthPrefixSeq<'l, Idx : From<usize> + Into<usize>, T> {
    inner : Cow<'l, [T]>,
    _ph   : PhantomData<Idx>
}
impl<'l, Idx : From<usize> + Into<usize>, T> LengthPrefixSeq<'l, Idx, T> {

    pub fn new() -> Self { Self {
        inner : Vec::new(),
        _ph   : PhantomData
    } }

    pub fn into_inner(self) -> Vec<T> { self.inner }

}
impl<'l, Idx : From<usize> + Into<usize>, T> From<Vec<T>> for LengthPrefixSeq<'l, Idx, T> {
    fn from(value : Vec<T>) -> Self { Self {
        inner : Cow::Owned(value),
        _ph   : PhantomData
    } }
}
impl<'l, Idx : From<usize> + Into<usize>, T> From<&'l [T]> for LengthPrefixSeq<'l, Idx, T> {
    fn from(value : &'l [T]) -> Self { Self {
        inner : Cow::Borrowed(value),
        _ph   : PhantomData
    } }
}
impl<'l, Idx : From<usize> + Into<usize>, T : fmt::Debug> fmt::Debug for LengthPrefixSeq<'l, Idx, T> { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LengthPrefixSeq{:?}", self.inner.borrow())
} }

impl<'l, Idx : From<usize> + Into<usize>, T> Default for LengthPrefixSeq<'l, Idx, T> { fn default() -> Self { Self::new() } }

impl<'l, Idx : From<usize> + Into<usize>, T> Deref for LengthPrefixSeq<'l, Idx, T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<'l, Idx : From<usize> + Into<usize>, T> DerefMut for LengthPrefixSeq<'l, Idx, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<'l, Idx : From<usize> + Into<usize> + PacketEncode, T : PacketEncode> PacketEncode for LengthPrefixSeq<'l, Idx, T> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.encode_write(Idx::from(self.inner.len()))?;
        for item in &self.inner {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<'l, Idx : From<usize> + Into<usize> + PacketDecode, T : PacketDecode> PacketDecode for LengthPrefixSeq<'l, Idx, T> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
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
