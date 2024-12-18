use std::ops::Deref;
use crate::registry::Registry;

pub struct Frozen<T>(Registry<T>);

impl<T> Deref for Frozen<T> {
    type Target = Registry<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

