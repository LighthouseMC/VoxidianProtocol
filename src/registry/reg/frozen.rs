use std::ops::Deref;
use crate::registry::Registry;

pub struct Frozen<T>(Registry<T>);

impl<T> Frozen<T> {
    pub fn freeze(reg: Registry<T>) -> Self {
        Frozen(reg)
    }
}
impl<T> Deref for Frozen<T> {
    type Target = Registry<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

