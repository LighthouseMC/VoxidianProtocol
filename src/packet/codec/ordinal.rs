use crate::value::*;


pub trait PacketEnumOrdinal : Copy {
    fn into_usize(self) -> usize;
    fn from_usize(from : usize) -> Self;
}

impl PacketEnumOrdinal for Var32 {
    fn into_usize(self) -> usize { self.as_i32() as usize }
    fn from_usize(from : usize) -> Self { Self::from(from as i32) }
}
impl PacketEnumOrdinal for u8 {
    fn into_usize(self) -> usize { self as usize }
    fn from_usize(from : usize) -> Self { from as u8 }
}
