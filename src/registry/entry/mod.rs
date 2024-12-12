mod block;
pub use block::*;
mod block_state;
pub use block_state::*;


use crate::value::*;
use std::marker::PhantomData;
use std::fmt;


pub struct RegEntry<T> {
    id  : usize,
    _ph : PhantomData<T>
}
impl<T> Clone for RegEntry<T> {
    fn clone(&self) -> Self { Self {
        id  : self.id,
        _ph : PhantomData
    } }
}
impl<T> Copy for RegEntry<T> { }
impl<T> fmt::Debug for RegEntry<T> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RegEntry(")?;
        write!(f, "{}", self.id)?;
        write!(f, ")")?;
        Ok(())
    }
}
