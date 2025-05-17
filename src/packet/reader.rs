use super::*;
use flate2::read::GzDecoder;
use std::io::{ self, Read };
use std::fmt;
use std::borrow::Cow;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct PacketReader<'l> {
    bytes : Cow<'l, [u8]>,
    head  : usize
}

/// Constructors
impl PacketReader<'static> {

    pub const EMPTY : Self = Self { bytes : Cow::Borrowed(&[]), head : 0 };

    /// Also returns the number of bytes that were consumed.
    pub fn from_raw_queue(
        mut queue: impl Iterator<Item = u8>,
    ) -> Result<(Self, usize), DecodeError> {
        let (size, consumed) = Var32::decode_iter(&mut queue)?;
        let size = size.as_i32() as usize;
        // TODO: Max packet size.
        let mut bytes = Vec::with_capacity(size);
        for _ in 0..size {
            let Some(byte) = queue.next() else {
                return Err(DecodeError::EndOfBuffer);
            };
            bytes.push(byte);
        }
        Ok((Self { bytes : Cow::Owned(bytes), head : 0 }, consumed + size,))
    }

    /// Reads all bytes from the stream, returning it as a `PacketBuf`.
    pub fn read(f: &mut impl Read) -> io::Result<Self> {
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(Self { bytes : Cow::Owned(bytes), head : 0 })
    }

}
impl<'l> PacketReader<'l> {

    pub fn inflate_gzip(&self) -> io::Result<PacketReader<'static>> {
        let mut gz = GzDecoder::new(&*self.bytes);
        PacketReader::read(&mut gz)
    }

}
impl From<PacketWriter> for PacketReader<'static> {
    fn from(value : PacketWriter) -> Self {
        Self { bytes : Cow::Owned(value.into_inner()), head : 0 }
    }
}
impl From<Vec<u8>> for PacketReader<'static> {
    fn from(value : Vec<u8>) -> Self {
        Self { bytes : Cow::Owned(value), head : 0 }
    }
}
impl<'l> From<&'l [u8]> for PacketReader<'l> {
    fn from(value : &'l [u8]) -> Self {
        Self { bytes : Cow::Borrowed(value), head : 0 }
    }
}
impl<'l> From<Cow<'l, [u8]>> for PacketReader<'l> {
    fn from(value : Cow<'l, [u8]>) -> Self {
        Self { bytes : value, head : 0 }
    }
}

/// Destructures
impl<'l> PacketReader<'l> {

    pub fn clone_inner(self) -> Vec<u8> {
        self.bytes
            .into_iter()
            .skip(self.head)
            .cloned()
            .collect()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.bytes.get(self.head..).unwrap_or(&[])
    }

    pub fn head(&self) -> usize { self.head }

}

impl<'l> Read for PacketReader<'l> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = buf.len();
        for i in 0..n {
            match (self.read_u8()) {
                Ok(b) => { buf[i] = b; },
                Err(DecodeError::EndOfBuffer) => { return Ok(i); },
                Err(e) => { return Err(io::Error::new(io::ErrorKind::Other, e.to_string())); }
            }
        }
        Ok(n)
    }
}

/// Basic Operations
impl<'l> PacketReader<'l> {

    #[track_caller]
    pub fn seek(&mut self, idx: usize) {
        assert!(idx <= self.bytes.len(), "Seek index exceeded packet size");
        self.head = idx;
    }

    #[track_caller]
    pub fn skip(&mut self, count: usize) {
        self.seek(self.head + count);
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.head
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let byte = self
            .bytes
            .get(self.head)
            .ok_or(DecodeError::EndOfBuffer)?;
        self.head += 1;
        Ok(*byte)
    }

    pub fn read_u8s_const<const BYTES: usize>(&mut self) -> Result<[u8; BYTES], DecodeError> {
        if (BYTES == 0) {
            return Ok([0; BYTES]);
        }
        let out = self
            .bytes
            .iter()
            .skip(self.head)
            .cloned()
            .array_chunks()
            .next()
            .ok_or(DecodeError::EndOfBuffer)?;
        self.head += BYTES;
        Ok(out)
    }

    pub fn read_u8s(&mut self, bytes: usize) -> Result<Vec<u8>, DecodeError> {
        if (bytes == 0) {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(bytes);
        let mut data = self.bytes.iter().skip(self.head);
        for _ in 0..bytes {
            out.push(*data.next().ok_or(DecodeError::EndOfBuffer)?);
        }
        self.head += bytes;
        Ok(out)
    }

}

// Iterator

impl<'l, 'k> IntoIterator for &'k mut PacketReader<'l> {
    type Item = u8;
    type IntoIter = PacketReaderIter<'l, 'k>;
    fn into_iter(self) -> Self::IntoIter {
        PacketReaderIter(self)
    }
}
pub struct PacketReaderIter<'l, 'k>(&'k mut PacketReader<'l>);
impl<'l, 'k> Iterator for PacketReaderIter<'l, 'k> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let b = self.0.bytes.get(self.0.head)?;
        self.0.head += 1;
        Some(*b)
    }
}

impl<'l, 'k> PacketReader<'l> {
    pub fn iter(&'k mut self) -> PacketReaderIter<'l, 'k> {
        self.into_iter()
    }
}

/// Decode
impl<'l> PacketReader<'l> {
    pub fn read_decode<T: PacketDecode>(&mut self) -> Result<T, DecodeError> {
        T::decode(self)
    }
}

impl<'l> fmt::Debug for PacketReader<'l> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PacketReader[")?;
        for byte in self.bytes.iter().skip(self.head) {
            write!(f, "{:02X} ", byte)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint_decode_iter() {
        let data = [
            16, 0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1, 1, 0,
        ];
        //          ^Packet length
        let Ok((len, consumed)) = Var32::decode_iter(&mut data.into_iter()) else {
            panic!("decode_iter was not a success");
        };
        assert_eq!(len.as_i32(), 16);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn packetbuf_from_raw_queue() {
        let data = [
            16, 0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1, 1, 0,
        ];
        //          |   |------------------------------------------------------------------  |  ^Status request packet
        //          |   ^Handshake packet                                                    ^Packet length
        //          ^Packet length
        let Ok((packetbuf, consumed)) = PacketReader::from_raw_queue(data.into_iter()) else {
            panic!("from_raw_queue was not a success")
        };
        assert_eq!(&*packetbuf.bytes, [
            0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1
        ]);
        assert_eq!(consumed, 17);
    }
}
