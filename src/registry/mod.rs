mod entry;
pub use entry::*;
mod encode;
pub use encode::*;

use crate::value::Identifier;
use std::collections::HashMap;
use std::{ fmt, any };


pub struct Registry<T> {
    id_key   : HashMap<usize, Identifier>,
    id_value : HashMap<usize, T>,
    next_id  : usize
}

impl<T> Registry<T> {

    pub fn new() -> Self { Self {
        id_key   : HashMap::new(),
        id_value : HashMap::new(),
        next_id  : 0
    } }

    pub fn register(&mut self, key : Identifier, value : T) -> RegEntry<T> {
        if (self.id_key.values().any(|key1| key1 == &key)) {
            panic!("Key `{}` has already been registered to {:?}", key, self);
        }
        let id = self.next_id;
        self.next_id += 1;
        self.id_key.insert(id, key);
        self.id_value.insert(id, value);
        unsafe{ RegEntry::new_unchecked(id) }
    }

    pub fn key_by_entry(&self, entry : RegEntry<T>) -> Option<&Identifier> {
        #[allow(deprecated)]
        self.id_key.get(&entry.id())
    }

    pub fn entry_by_key(&self, key : &Identifier) -> Option<RegEntry<T>> {
        self.id_key.iter().find_map(|(id1, key1)| if (key == key1) { Some(unsafe{ RegEntry::new_unchecked(*id1) }) } else { None } )
    }

    pub fn value_by_entry(&self, entry : RegEntry<T>) -> Option<&T> {
        #[allow(deprecated)]
        self.id_value.get(&entry.id())
    }

    pub fn value_by_key(&self, key : &Identifier) -> Option<&T> { self.entry_by_key(key).map(|entry| self.value_by_entry(entry)).flatten() }

}
impl<T : PartialEq> Registry<T> {

    pub fn entry_by_value(&self, value : &T) -> Option<RegEntry<T>> {
        self.id_value.iter().find_map(|(id1, value1)| if (value == value1) { Some(unsafe{ RegEntry::new_unchecked(*id1) }) } else { None } )
    }

    pub fn key_by_value(&self, value : &T) -> Option<&Identifier> {
        self.id_value.iter().find_map(|(id1, value1)| if (value == value1) { self.id_key.get(id1) } else { None } )
    }

}
impl<T> fmt::Debug for Registry<T> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry<{}>({})", any::type_name::<T>(), self.next_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::registry::Registry;
    use crate::value::Identifier;

    #[test]
    pub fn simple_registry() {
        let mut registry = Registry::<usize>::new();
        registry.register(Identifier::new("test", "a"), 10);
        registry.register(Identifier::new("test", "b"), 20);
        registry.register(Identifier::new("test", "c"), 30);

        assert_eq!(registry.value_by_key(&Identifier::new("test", "a")), Some(&10));
        assert_eq!(registry.value_by_key(&Identifier::new("test", "b")), Some(&20));
        assert_eq!(registry.value_by_key(&Identifier::new("test", "c")), Some(&30));
    }
}