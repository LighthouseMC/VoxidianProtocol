mod entry;
pub use entry::*;

use std::collections::HashMap;
use crate::value::Identifier;

pub enum RegistryInsertResult {
    Successful,
    IdentifierAlreadyOccupied
}

pub struct Registry<T> {
    keys: HashMap<Identifier, usize>,
    elements: Vec<T>
}

impl<T> Registry<T> {
    pub fn new() -> Registry<T> {
        Registry {
            keys: HashMap::new(),
            elements: Vec::new()
        }
    }

    pub fn get(&self, key: Identifier) -> Option<&T> {
        let Some(idx) = self.keys.get(&key) else {
            return None;
        };
        Some(&self.elements[*idx])
    }

    pub fn insert(&mut self, key: Identifier, value: T) -> RegistryInsertResult {
        if self.keys.contains_key(&key) {
            RegistryInsertResult::IdentifierAlreadyOccupied
        } else {
            self.elements.push(value);
            self.keys.insert(key, self.elements.len()-1);
            RegistryInsertResult::Successful
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::Registry;
    use crate::value::Identifier;

    #[test]
    pub fn simple_registry() {
        let mut registry = Registry::new();
        registry.insert(Identifier::new("voxidian_protocol", "a"), 10);
        registry.insert(Identifier::new("voxidian_protocol", "b"), 20);
        registry.insert(Identifier::new("voxidian_protocol", "c"), 30);

        assert_eq!(registry.get(Identifier::new("voxidian_protocol", "a")), Some(&10));
        assert_eq!(registry.get(Identifier::new("voxidian_protocol", "b")), Some(&20));
        assert_eq!(registry.get(Identifier::new("voxidian_protocol", "c")), Some(&30));
    }
}