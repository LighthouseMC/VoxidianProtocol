use super::*;

use std::marker::PhantomData;
use std::ops::{ Deref, DerefMut };
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LengthPrefixHashMap<Index : From<usize> + Into<usize>, K : Eq + Hash, V> {
    inner : HashMap<K, V>,
    _ph   : PhantomData<Index>
}
impl<Index : From<usize> + Into<usize>, K : Eq + Hash, V> LengthPrefixHashMap<Index, K, V> {

    pub fn new() -> Self { Self {
        inner : HashMap::new(),
        _ph   : PhantomData
    } }

    pub fn into_inner(self) -> HashMap<K, V> { self.inner }

}
impl<Index : From<usize> + Into<usize>, K : Eq + Hash, V> From<HashMap<K, V>> for LengthPrefixHashMap<Index, K, V> {
    fn from(value : HashMap<K, V>) -> Self { Self {
        inner : value,
        _ph   : PhantomData
    } }
}

impl<Index : From<usize> + Into<usize>, K : Eq + Hash, V> Deref for LengthPrefixHashMap<Index, K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target { &self.inner }
}
impl<Index : From<usize> + Into<usize>, K : Eq + Hash, V> DerefMut for LengthPrefixHashMap<Index, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}

impl<Index : From<usize> + Into<usize> + PacketEncode, K : Eq + Hash + PacketEncode, V : PacketEncode> PacketEncode for LengthPrefixHashMap<Index, K, V> {
    fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
        buf.encode_write(Index::from(self.inner.len()))?;
        for (key, value) in &self.inner {
            buf.encode_write(key)?;
            buf.encode_write(value)?;
        }
        Ok(())
    }
}
impl<Index : From<usize> + Into<usize> + PacketDecode, K : Eq + Hash + PacketDecode, V : PacketDecode> PacketDecode for LengthPrefixHashMap<Index, K, V> {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        let len = buf.read_decode::<Index>()?.into();
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
