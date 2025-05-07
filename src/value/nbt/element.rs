use super::*;


#[derive(Debug, Clone, PartialEq)]
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
    /// You are responsible for guaranteeing that the elements of this list are all of the same type.
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
impl PacketEncode for NbtElement { fn encode(&self, buf : &mut PacketWriter) -> Result<(), EncodeError> {
    buf.write_u8(self.tag());
    self.encode_packet(buf);
    Ok(())
} }
impl NbtElement { pub(super) fn encode_packet(&self, buf : &mut PacketWriter) { match (self) {
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
        for e in values { e.encode_packet(buf); }
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
impl PacketDecode for NbtElement { fn decode<'l>(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let tag = buf.read_u8()?;
    Self::decode_packet(buf, tag)
} }
impl NbtElement {

    pub(super) fn decode_packet<'l>(buf : &mut PacketReader<'l>, tag : u8) -> Result<Self, DecodeError> { match (tag) {
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
        tag => Err(DecodeError::InvalidData(Cow::Owned(format!("Unknown nbt tag `{}`", tag))))
    } }

    pub(super) fn decode_string<'l>(buf : &mut PacketReader<'l>) -> Result<String, DecodeError> {
        let     len   = buf.read_decode::<u16>()? as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len { bytes.push(buf.read_u8()?); }
        let string = cesu8::from_java_cesu8(&bytes).map_err(|_| DecodeError::InvalidData(Cow::Borrowed("String data is not valid CESU8")))?;
        Ok(string.to_string())
    }

}
