mod frozen;
pub use frozen::*;
use indexmap::IndexMap;

use crate::packet::s2c::config::RegistryDataS2CConfigPacket;
use crate::registry::RegEntry;
use crate::value::{Identifier, LengthPrefixVec};
use super::*;

/// Represents a registry.
pub struct Registry<T> {
    map: IndexMap<Identifier, T>,
}

impl<T> Registry<T> {

    pub fn new() -> Self {
        Self {
            map: IndexMap::new()
        }
    }

    pub fn get(&self, key: &Identifier) -> Option<&T> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: Identifier, value: T) {
        self.map.insert(key, value);
    }

    pub fn map<F: FnOnce(&T) -> T>(&mut self, key: Identifier, func: F) {
        let value: &T = self.get(&key).expect("infallible");
        let new_value = func(value);
        self.insert(key, new_value);
    }

    pub fn lookup(&self, entry: &RegEntry<T>) -> Option<&T> {
        self.map.get_index(entry.id()).map(|x| x.1)
    }

    pub fn make_entry(&self, identifier: &Identifier) -> Option<RegEntry<T>> {
        self.map
            .get_full(&identifier)
            .map(|(index, _, _)| unsafe { RegEntry::new_unchecked(index) })
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn freeze(self) -> RegistryFrozen<T> {
        RegistryFrozen::freeze(self)
    }

}

impl<T : RegValue> Registry<T> {
    pub fn to_registry_data_packet(&self) -> RegistryDataS2CConfigPacket {
        let mut entries = LengthPrefixVec::new();
        for entry in self.map.iter() {
            entries.push((entry.0.clone(), entry.1.to_registry_data_packet()));
        }
        RegistryDataS2CConfigPacket {
            registry: T::REGISTRY_ID,
            entries
        }
    }
}

impl<T> Default for Registry<T> { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use crate::registry::Registry;
    use crate::value::Identifier;

    #[test]
    pub fn simple_registry() {
        let mut rg = Registry::new();
        rg.insert(Identifier::new("test", "a"), 10);
        rg.insert(Identifier::new("test", "b"), 20);
        rg.insert(Identifier::new("test", "c"), 30);

        assert_eq!(rg.get(&Identifier::new("test", "b")), Some(&20));
    }

    #[test]
    pub fn registries_can_freeze() {
        let mut rg = Registry::new();
        rg.insert(Identifier::new("test", "a"), 10);
        rg.insert(Identifier::new("test", "b"), 20);
        rg.insert(Identifier::new("test", "c"), 30);

        let rg = rg.freeze();
        assert_eq!(rg.get(&Identifier::new("test", "b")), Some(&20));
    }
}