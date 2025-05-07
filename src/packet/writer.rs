use super::*;
use std::io::{self, Write};
use std::fmt;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct PacketWriter {
    bytes : Vec<u8>
}

/// Constructors
impl Default for PacketWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketWriter {

    pub fn new() -> Self {
        Self { bytes : Vec::new() }
    }

    pub fn of_encode<T: PacketEncode>(encodable: T) -> Result<Self, EncodeError> {
        let mut buf = Self::new();
        encodable.encode(&mut buf)?;
        Ok(buf)
    }

}
impl<'l> From<PacketReader<'l>> for PacketWriter {
    fn from(value : PacketReader<'l>) -> Self {
        Self { bytes : value.clone_inner() }
    }
}
impl From<Vec<u8>> for PacketWriter {
    fn from(bytes : Vec<u8>) -> Self {
        Self { bytes }
    }
}

/// Destructures
impl PacketWriter {
    pub fn into_inner(self) -> Vec<u8> {
        self.bytes
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize { self.bytes.len() }
}

impl Write for PacketWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for element in buf {
            self.write_u8(*element);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Basic Operations
impl PacketWriter {

    pub fn write_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    pub fn write_u8s(&mut self, data: &[u8]) {
        self.bytes.extend_from_slice(data);
    }

    #[deprecated = "Are you sure?"]
    pub(crate) fn insert(&mut self, idx: usize, bytes: &[u8]) {
        self.bytes.splice(idx..idx, bytes.iter().cloned());
    }
}

/// Encode
impl PacketWriter {
    pub fn encode_write<T: PacketEncode>(&mut self, encodable: T) -> Result<(), EncodeError> {
        encodable.encode(self)
    }
}

impl fmt::Debug for PacketWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PacketWriter[")?;
        for byte in self.bytes.iter() {
            write!(f, "{:02X} ", byte)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
}
