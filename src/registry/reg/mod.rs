mod frozen;

use std::collections::HashMap;
pub use frozen::*;
use crate::registry::RegEntry;
use crate::value::Identifier;

/// Represents a registry.
pub struct Registry<T> {
    // key = Identifier
    // id = RegEntry<T> / usize
    // value = T
    values: Vec<T>,
    keys: Vec<Identifier>,
    key_to_value: HashMap<Identifier, usize>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            keys: Vec::new(),
            key_to_value: HashMap::new(),
        }
    }

    pub fn get(&self, key: &Identifier) -> Option<&T> {
        self.values.get(self.key_to_value.get(key))
    }

    pub fn insert(&mut self, key: Identifier, value: T) {
        self.keys.push(key.clone());
        self.values.push(value);
        self.key_to_value.insert(key, self.values.len()-1);
    }

    pub fn map<F: FnOnce(&T) -> T>(&mut self, key: Identifier, func: F) {
        let Some(idx) = self.key_to_value.get(&key) else {
            return;
        };

        let value: &T = self.values.get(&key).expect("infallible");
        let new_value = func(value);

        self.values[key] = new_value;
    }

    pub fn lookup(&self, entry: &RegEntry<T>) -> Option<&T> {
        self.values[entry]
    }

    pub fn make_entry(&self, identifier: &Identifier) -> Option<RegEntry<T>> {
        // Safety: Since the RegEntry is only obtainable if the Identifier is a valid entry,
        // this code will not cause any issues with the client
        self.key_to_value.get(&identifier).map(|x| unsafe { RegEntry::new_unchecked(*x) })
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
        self.key_to_value.clear();
    }

    pub fn freeze(self) -> Frozen<T> {
        Frozen(self)
    }
}

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