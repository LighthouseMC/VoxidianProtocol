use super::*;
use std::ops::Deref;


pub struct RegistryFrozen<T>(Registry<T>);

impl<T> RegistryFrozen<T> {
    pub fn freeze(reg: Registry<T>) -> Self {
        RegistryFrozen(reg)
    }
}
impl<T> Deref for RegistryFrozen<T> {
    type Target = Registry<T>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

