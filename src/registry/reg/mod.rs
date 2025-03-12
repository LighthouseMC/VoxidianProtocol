mod frozen;
use std::collections::HashSet;

pub use frozen::*;
use indexmap::IndexMap;

use crate::packet::s2c::config::RegistryDataS2CConfigPacket;
use crate::registry::RegEntry;
use crate::value::{Identifier, LengthPrefixVec, VarInt};
use super::*;

/// Represents a registry.
pub struct Registry<T> {
    map: IndexMap<Identifier, T>,
    tags: IndexMap<Identifier, HashSet<RegEntry<T>>>
}

pub type TagInformation = LengthPrefixVec<VarInt, (Identifier, LengthPrefixVec<VarInt, u32>)>;

impl<T> Registry<T> {

    pub fn new() -> Self {
        Self {
            map: IndexMap::new(),
            tags: IndexMap::new(),
        }
    }

    pub fn get(&self, key: &Identifier) -> Option<&T> {
        self.map.get(key)
    }

    pub fn get_entry(&self, key: &Identifier) -> Option<RegEntry<T>> {
        self.map
            .get_full(key)
            .map(|(index, _, _)| unsafe { RegEntry::new_unchecked(index as u32) })
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
        self.map.get_index(entry.id() as usize).map(|x| x.1)
    }

    pub fn lookup_ident(&self, entry: &RegEntry<T>) -> Option<&Identifier> {
        self.map.get_index(entry.id() as usize).map(|x| x.0)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn freeze(self) -> RegistryFrozen<T> {
        RegistryFrozen::freeze(self)
    }

    pub fn keys(&self) -> impl Iterator<Item = &Identifier> {
        self.map.keys()
    }

    pub fn entries(&self) -> impl Iterator<Item = (&Identifier, &T)> {
        self.map.keys().zip(self.map.values())
    }

    pub fn insert_tag(&mut self, tag: &Identifier, entry: &Identifier) {
        let entry = self.get_entry(entry);
        let tag_container = match self.tags.get_mut(&tag) {
            Some(container) => container,
            None => {
                self.tags.insert(tag.clone(), HashSet::new());
                self.tags.get_mut(&tag).unwrap()
            }
        };
        if let Some(entry) = entry {
            tag_container.insert(entry);
        }
        
    }

    pub fn get_tags(&self, tag: &Identifier) -> Option<&HashSet<RegEntry<T>>> {
        self.tags.get(&tag)
    }

    pub fn flatten_tags_for_packet(&self) -> TagInformation {
        let mut base_tags = LengthPrefixVec::new();
        for tag in &self.tags {
            let mut tag_container = LengthPrefixVec::new();
            for entry in tag.1 {
                tag_container.push(entry.id());
            }
            base_tags.push((tag.0.clone(), tag_container));
        }
        base_tags
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

    #[test]
    pub fn tags() {
        let mut rg = Registry::new();
        rg.insert(Identifier::new("test", "a"), 1);
        rg.insert(Identifier::new("test", "b"), 2);
        rg.insert(Identifier::new("test", "c"), 3);

        rg.insert_tag(&Identifier::new("test", "1_to_2"), &Identifier::new("test", "a"));
        rg.insert_tag(&Identifier::new("test", "1_to_2"), &Identifier::new("test", "b"));

        assert_eq!(rg.get_tags(&Identifier::new("test", "1_to_2")).unwrap().len(), 2);
    }
}
