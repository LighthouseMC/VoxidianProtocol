use super::*;

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Clone, PartialEq, Eq)]
pub struct LengthPrefixHashMap<Idx : From<usize> + Into<usize>, K : Eq + Hash, V> {
    inner : HashMap<K, V>,
    _ph   : PhantomData<Idx>
}
impl<Idx : From<usize> + Into<usize>, K : Eq + Hash, V> LengthPrefixHashMap<Idx, K, V> {

    pub fn new() -> Self { Self {
        inner : HashMap::new(),
        _ph   : PhantomData
    } }

    pub fn into_inner(self) -> HashMap<K, V> { self.inner }

}
impl<Idx : From<usize> + Into<usize>, K : Eq + Hash, V> From<HashMap<K, V>> for LengthPrefixHashMap<Idx, K, V> {
    fn from(value : HashMap<K, V>) -> Self { Self {
        inner : value,
        _ph   : PhantomData
    } }
}
impl<Idx : From<usize> + Into<usize>, K : Eq + Hash + fmt::Debug, V : fmt::Debug> fmt::Debug for LengthPrefixHashMap<Idx, K, V> { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LengthPrefixHashMap{:?}", self.inner)
} }
impl<Idx : From<usize> + Into<usize>, K : Eq + Hash, V> Default for LengthPrefixHashMap<Idx, K, V> { fn default() -> Self { Self::new() } }

impl<Idx : From<usize> + Into<usize>, K : Eq + Hash, V> Deref for LengthPrefixHashMap<Idx, K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<Idx : From<usize> + Into<usize>, K : Eq + Hash, V> DerefMut for LengthPrefixHashMap<Idx, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<Idx : From<usize> + Into<usize> + PacketEncode, K : Eq + Hash + PacketEncode, V : PacketEncode> PacketEncode for LengthPrefixHashMap<Idx, K, V> {
    fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
        buf.encode_write(Idx::from(self.inner.len()))?;
        for (key, value) in &self.inner {
            buf.encode_write(key)?;
            buf.encode_write(value)?;
        }
        Ok(())
    }
}
impl<Idx : From<usize> + Into<usize> + PacketDecode, K : Eq + Hash + PacketDecode, V : PacketDecode> PacketDecode for LengthPrefixHashMap<Idx, K, V> {
    fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        let len = buf.read_decode::<Idx>()?.into();
        let mut items = HashMap::new();
        for _ in 0..len {
            items.insert(buf.read_decode::<K>()?, buf.read_decode::<V>()?);
        }
        Ok(Self {
            inner : items,
            _ph   : PhantomData
        })
    }
}
