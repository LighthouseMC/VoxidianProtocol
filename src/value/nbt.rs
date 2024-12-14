use super::*;

use std::collections::HashMap;


#[derive(Clone, PartialEq)]
pub struct Nbt {
    pub name : String,
    pub root : NbtCompound
}
impl fmt::Debug for Nbt { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Nbt({:?} -> Compound({:?}))", self.name, self.root)
} }

#[derive(Clone, PartialEq)]
pub struct NbtCompound(HashMap<String, NbtElement>);
impl NbtCompound {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&NbtElement> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: &str, value: NbtElement) {
        self.0.insert(key.to_string(), value);
    }
}
impl fmt::Debug for NbtCompound { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.0)
} }


#[derive(Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum NbtElement {
    Byte     (i8              ) = 1,
    Short    (i16             ) = 2,
    Int      (i32             ) = 3,
    Long     (i64             ) = 4,
    Float    (f32             ) = 5,
    Double   (f64             ) = 6,
    BArray   (Vec<i8>         ) = 7,
    String   (String          ) = 8,
    List     (Vec<NbtElement> ) = 9,
    Compound (NbtCompound     ) = 10,
    IArray   (Vec<i32>        ) = 11,
    LArray   (Vec<i64>        ) = 12
}
impl NbtElement {
    pub const TAG_END      : u8 = 0;
    pub const TAG_BYTE     : u8 = 1;
    pub const TAG_SHORT    : u8 = 2;
    pub const TAG_INT      : u8 = 3;
    pub const TAG_LONG     : u8 = 4;
    pub const TAG_FLOAT    : u8 = 5;
    pub const TAG_DOUBLE   : u8 = 6;
    pub const TAG_BARRAY   : u8 = 7;
    pub const TAG_STRING   : u8 = 8;
    pub const TAG_LIST     : u8 = 9;
    pub const TAG_COMPOUND : u8 = 10;
    pub const TAG_IARRAY   : u8 = 11;
    pub const TAG_LARRAY   : u8 = 12;
    pub fn tag(&self) -> u8 { match (self) {
        NbtElement::Byte     (_) => Self::TAG_BYTE,
        NbtElement::Short    (_) => Self::TAG_SHORT,
        NbtElement::Int      (_) => Self::TAG_INT,
        NbtElement::Long     (_) => Self::TAG_LONG,
        NbtElement::Float    (_) => Self::TAG_FLOAT,
        NbtElement::Double   (_) => Self::TAG_DOUBLE,
        NbtElement::BArray   (_) => Self::TAG_BARRAY,
        NbtElement::String   (_) => Self::TAG_STRING,
        NbtElement::List     (_) => Self::TAG_LIST,
        NbtElement::Compound (_) => Self::TAG_COMPOUND,
        NbtElement::IArray   (_) => Self::TAG_IARRAY,
        NbtElement::LArray   (_) => Self::TAG_LARRAY,
    } }
}


impl PacketEncode for Nbt { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.write_u8(NbtElement::TAG_COMPOUND);
    self.root.encode_packet(buf);
    Ok(())
} }
impl NbtCompound { fn encode_packet(&self, buf : &mut PacketBuf) {
    for (key, value) in &self.0 {
        buf.write_u8(value.tag());
        NbtElement::String(key.clone()).encode_packet(buf);
        value.encode_packet(buf);
    }
    buf.write_u8(NbtElement::TAG_END);
} }
impl NbtElement { fn encode_packet(&self, buf : &mut PacketBuf) { match (self) {
    Self::Byte   (value) => { buf.write_u8(*value as u8); },
    Self::Short  (value) => { let _ = buf.encode_write(value); },
    Self::Int    (value) => { let _ = buf.encode_write(value); },
    Self::Long   (value) => { let _ = buf.encode_write(value); },
    Self::Float  (value) => { let _ = buf.encode_write(value); },
    Self::Double (value) => { let _ = buf.encode_write(value); },
    Self::BArray(values) => {
        let _ = buf.encode_write(values.len() as i32);
        for byte in values { buf.write_u8(*byte as u8); }
    },
    Self::String (value) => {
        let jstring = cesu8::to_java_cesu8(value);
        let _ = buf.encode_write(jstring.len() as u16);
        for byte in jstring.as_ref() { buf.write_u8(*byte); }
    },
    Self::List (values) => {
        buf.write_u8(values.first().map_or(NbtElement::TAG_END, |e| e.tag()));
        let _ = buf.encode_write(values.len() as i32);
        for e in values { let _ = e.encode_packet(buf); }
    },
    Self::Compound (values) => { values.encode_packet(buf); },
    Self::IArray(values) => {
        let _ = buf.encode_write(values.len() as i32);
        for int in values { let _ = buf.encode_write(*int); }
    },
    Self::LArray(values) => {
        let _ = buf.encode_write(values.len() as i32);
        for long in values { let _ = buf.encode_write(*long); }
    }
} } }


impl PacketDecode for Nbt { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let tag = buf.read_u8()?;
    if (tag != NbtElement::TAG_COMPOUND) {
        return Err(DecodeError::InvalidData);
    }
    Ok(Nbt {
        name : String::new(),
        root : NbtCompound::decode_packet(buf)?,
    })
} }
impl NbtCompound { fn decode_packet(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let mut compound = Self::new();
    while (buf.remaining() > 0) {
        let tag = buf.read_u8()?;
        if (tag == NbtElement::TAG_END) { break; }
        let key = NbtElement::decode_string(buf)?;
        let value = NbtElement::decode_packet(buf, tag)?;
        compound.0.insert(key, value);
    }
    Ok(compound)
} }
impl NbtElement {

    fn decode_packet(buf : &mut PacketBuf, tag : u8) -> Result<Self, DecodeError> { match (tag) {
        Self::TAG_BYTE   => Ok(Self::Byte   (buf.read_u8()? as i8)),
        Self::TAG_SHORT  => Ok(Self::Short  (buf.read_decode()?)),
        Self::TAG_INT    => Ok(Self::Int    (buf.read_decode()?)),
        Self::TAG_LONG   => Ok(Self::Long   (buf.read_decode()?)),
        Self::TAG_FLOAT  => Ok(Self::Float  (buf.read_decode()?)),
        Self::TAG_DOUBLE => Ok(Self::Double (buf.read_decode()?)),
        Self::TAG_BARRAY => {
            let     len = buf.read_decode::<i32>()? as usize;
            let mut out = Vec::with_capacity(len);
            for _ in 0..len { out.push(buf.read_decode()?); }
            Ok(Self::BArray(out))
        },
        Self::TAG_STRING => Ok(Self::String(Self::decode_string(buf)?)),
        Self::TAG_LIST => {
            let     tag = buf.read_u8()?;
            let     len = buf.read_decode::<i32>()? as usize;
            let mut out = Vec::with_capacity(len);
            for _ in 0..len { out.push(NbtElement::decode_packet(buf, tag)?); }
            Ok(Self::List(out))
        },
        Self::TAG_COMPOUND => Ok(Self::Compound(NbtCompound::decode_packet(buf)?)),
        Self::TAG_IARRAY => {
            let     len = buf.read_decode::<i32>()? as usize;
            let mut out = Vec::with_capacity(len);
            for _ in 0..len { out.push(buf.read_decode()?); }
            Ok(Self::IArray(out))
        },
        Self::TAG_LARRAY => {
            let     len = buf.read_decode::<i32>()? as usize;
            let mut out = Vec::with_capacity(len);
            for _ in 0..len { out.push(buf.read_decode()?); }
            Ok(Self::LArray(out))
        },
        _ => Err(DecodeError::InvalidData)
    } }

    fn decode_string(buf : &mut PacketBuf) -> Result<String, DecodeError> {
        let     len   = buf.read_decode::<u16>()? as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len { bytes.push(buf.read_u8()?); }
        let string = cesu8::from_java_cesu8(&bytes).map_err(|_| DecodeError::InvalidData)?;
        Ok(string.to_string())
    }

}
