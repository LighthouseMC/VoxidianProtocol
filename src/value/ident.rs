use super::*;
use std::fmt;
use std::borrow::Cow;


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub namespace : Cow<'static, str>,
    pub path      : Cow<'static, str>
}
impl Identifier {

    pub fn new<N : Into<String>, P : Into<String>>(namespace : N, path : P) -> Self { Self {
        namespace : Cow::Owned(namespace .into()),
        path      : Cow::Owned(path      .into())
    } }

    pub fn vanilla<P : Into<String>>(path : P) -> Self { Self::new("minecraft", path) }

    pub const fn new_const(namespace : &'static str, path : &'static str) -> Self { Self {
        namespace : Cow::Borrowed(namespace ),
        path      : Cow::Borrowed(path      )
    } }

    pub const fn vanilla_const(path : &'static str) -> Self { Self::new_const("minecraft", path) }

}
impl fmt::Display for Identifier { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.namespace, self.path)
} }
impl fmt::Debug for Identifier { fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self.to_string())
} }
impl<T : AsRef<str>> From<T> for Identifier {
    fn from(value : T) -> Self {
        let value = value.as_ref();
        let Some(i) = value.find(':') else { return Identifier::vanilla(value) };
        let (namespace, colon_path) = value.split_at(i);
        Self::new(namespace, colon_path.chars().skip(1).collect::<String>())
    }
}
impl Ser for Identifier {
    fn serialize<S>(&self, ser : S) -> Result<S::Ok, S::Error> where S : Serer {
        self.to_string().serialize(ser)
    }
}
impl<'l> Deser<'l> for Identifier {
    fn deserialize<D>(deser : D) -> Result<Self, D::Error> where D : Deserer<'l> {
        Ok(Identifier::from(String::deserialize(deser)?))
    }
}


impl PacketEncode for Identifier { fn encode(&self, buf : &mut PacketBuf) -> Result<(), EncodeError> {
    buf.encode_write(self.to_string())
} }
impl PacketDecode for Identifier { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    Ok(Self::from(buf.read_decode::<String>()?))
} }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_identifier() {
        assert_eq!(Identifier::from("test:potato/hello"), Identifier::new("test", "potato/hello"));
        assert_eq!(Identifier::from("path/to/element"), Identifier::new("minecraft", "path/to/element"));
        assert_eq!(Identifier::from("voxidian-protocol:"), Identifier::new("voxidian-protocol", ""));
        assert_eq!(Identifier::from(""), Identifier::new("minecraft", ""));
    }

    #[test]
    fn identifier_to_string() {
        assert_eq!(Identifier::new("test", "potato/hello").to_string(), "test:potato/hello");
    }

}
