use std::fmt::Debug;
use super::*;

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };


#[derive(Clone, PartialEq, Eq)]
pub struct LengthPrefixDoubleVec<
    Idx1 : From<usize> + Into<usize>,
    Idx2 : From<usize> + Into<usize>,
    T1,
    T2
> {
    vec1  : Vec<T1>,
    vec2  : Vec<T2>,
    _ph   : PhantomData<(Idx1, Idx2)>
}
impl<Idx1 : From<usize> + Into<usize>, Idx2 : From<usize> + Into<usize>, T1, T2> LengthPrefixDoubleVec<Idx1, Idx2, T1, T2> {

    pub fn new() -> Self { Self {
        vec1 : Vec::new(),
        vec2 : Vec::new(),
        _ph   : PhantomData
    } }

}
impl<Idx1 : From<usize> + Into<usize>, Idx2 : From<usize> + Into<usize>, T1: Debug, T2: Debug> Debug
    for LengthPrefixDoubleVec<Idx1, Idx2, T1, T2> { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LengthPrefixDoubleVec{:?}{:?}", self.vec1, self.vec2)
} }

impl<Idx1 : From<usize> + Into<usize>, Idx2 : From<usize> + Into<usize>, T1, T2> Default for LengthPrefixDoubleVec<Idx1, Idx2, T1, T2> { fn default() -> Self { Self::new() } }

impl<Idx1 : From<usize> + Into<usize> + PacketEncode, Idx2 : From<usize> + Into<usize> + PacketEncode, T1: PacketEncode, T2: PacketEncode> PacketEncode for LengthPrefixDoubleVec<Idx1, Idx2, T1, T2> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.encode_write(Idx1::from(self.vec1.len()))?;
        buf.encode_write(Idx2::from(self.vec2.len()))?;
        for item in &self.vec1 {
            buf.encode_write(item)?;
        }
        for item in &self.vec2 {
            buf.encode_write(item)?;
        }
        Ok(())
    }
}
impl<Idx1 : From<usize> + Into<usize> + PacketDecode, Idx2 : From<usize> + Into<usize> + PacketDecode, T1: PacketDecode, T2: PacketDecode> PacketDecode for LengthPrefixDoubleVec<Idx1, Idx2, T1, T2> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let len1 = buf.read_decode::<Idx1>()?.into();
        let len2 = buf.read_decode::<Idx2>()?.into();
        let mut vec1 = Vec::new();
        for _ in 0..len1 {
            vec1.push(buf.read_decode::<T1>()?);
        }
        let mut vec2 = Vec::new();
        for _ in 0..len2 {
            vec2.push(buf.read_decode::<T2>()?);
        }
        Ok(Self {
            vec1,
            vec2,
            _ph   : PhantomData
        })
    }
}
