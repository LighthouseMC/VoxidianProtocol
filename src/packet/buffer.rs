use super::*;
use flate2::read::GzDecoder;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::{fmt, iter, slice, vec};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct PacketBuf {
    inner: Vec<u8>,
    read_idx: usize,
}

/// Constructors
impl Default for PacketBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketBuf {
    pub fn new() -> Self {
        PacketBuf {
            inner: Vec::new(),
            read_idx: 0,
        }
    }

    pub fn of_encode<T: PacketEncode>(encodable: T) -> Result<Self, EncodeError> {
        let mut buf = Self::new();
        encodable.encode(&mut buf)?;
        Ok(buf)
    }

    /// Also returns the number of bytes that were consumed.
    pub fn from_raw_queue(
        mut queue: impl Iterator<Item = u8>,
    ) -> Result<(Self, usize), DecodeError> {
        let (size, consumed) = VarInt::decode_iter(&mut queue)?;
        let size = size.as_i32() as usize;
        let mut bytes = Vec::with_capacity(size);
        for _ in 0..size {
            let Some(byte) = queue.next() else {
                return Err(DecodeError::EndOfBuffer);
            };
            bytes.push(byte);
        }
        Ok((
            Self {
                inner: bytes,
                read_idx: 0,
            },
            consumed + size,
        ))
    }

    pub fn inflate_gzip(&self) -> io::Result<Self> {
        let mut gz = GzDecoder::new(self.inner.as_slice());
        Self::read(&mut gz)
    }

    /// Reads all bytes from the stream, returning it as a `PacketBuf`.
    pub fn read(f: &mut impl Read) -> io::Result<Self> {
        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes)?;
        Ok(Self {
            inner: bytes,
            read_idx: 0,
        })
    }
}
impl From<Vec<u8>> for PacketBuf {
    fn from(value: Vec<u8>) -> Self {
        Self {
            inner: value,
            read_idx: 0,
        }
    }
}

/// Deconstructors
impl PacketBuf {
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
            .into_iter()
            .skip(self.read_idx)
            .collect::<Vec<_>>()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.inner.as_slice().get(self.read_idx..).unwrap_or(&[])
    }
}

impl Read for PacketBuf {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        #[allow(clippy::needless_range_loop)]
        for idx in 0..buf.len() {
            buf[idx] = self
                .read_u8()
                .map_err(|_| Error::from(ErrorKind::UnexpectedEof))?;
        }
        Ok(buf.len())
    }
}

impl Write for PacketBuf {
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
impl PacketBuf {
    #[track_caller]
    pub fn seek(&mut self, idx: usize) {
        assert!(idx <= self.inner.len(), "Seek index exceeded packet size");
        self.read_idx = idx;
    }

    #[track_caller]
    pub fn skip(&mut self, count: usize) {
        self.seek(self.read_idx + count);
    }

    pub fn remaining(&self) -> usize {
        self.inner.len() - self.read_idx
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.inner.push(byte);
    }

    pub fn write_u8s(&mut self, data: &[u8]) {
        self.inner.extend_from_slice(data);
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let byte = self
            .inner
            .get(self.read_idx)
            .ok_or(DecodeError::EndOfBuffer)?;
        self.read_idx += 1;
        Ok(*byte)
    }

    pub fn read_u8s_const<const BYTES: usize>(&mut self) -> Result<[u8; BYTES], DecodeError> {
        if (BYTES == 0) {
            return Ok([0; BYTES]);
        }
        let out = self
            .inner
            .iter()
            .skip(self.read_idx)
            .cloned()
            .array_chunks()
            .next()
            .ok_or(DecodeError::EndOfBuffer)?;
        self.read_idx += BYTES;
        Ok(out)
    }

    pub fn read_u8s(&mut self, bytes: usize) -> Result<Vec<u8>, DecodeError> {
        if (bytes == 0) {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(bytes);
        let mut data = self.inner.iter().skip(self.read_idx);
        for _ in 0..bytes {
            out.push(*data.next().ok_or(DecodeError::EndOfBuffer)?);
        }
        self.read_idx += bytes;
        Ok(out)
    }

    #[deprecated = "Are you sure?"]
    pub(crate) fn insert(&mut self, idx: usize, bytes: &[u8]) {
        self.inner.splice(idx..idx, bytes.iter().cloned());
    }
}

impl<'l> IntoIterator for &'l PacketBuf {
    type Item = &'l u8;
    type IntoIter = iter::Skip<slice::Iter<'l, u8>>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter().skip(self.read_idx)
    }
}
impl IntoIterator for PacketBuf {
    type Item = u8;
    type IntoIter = iter::Skip<vec::IntoIter<u8>>;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter().skip(self.read_idx)
    }
}
/// Iterator
impl PacketBuf {
    pub fn iter(&self) -> impl Iterator<Item = u8> {
        self.into_iter().copied()
    }
}

/// Encode & Decode
impl PacketBuf {
    pub fn encode_write<T: PacketEncode>(&mut self, encodable: T) -> Result<(), EncodeError> {
        encodable.encode(self)
    }

    pub fn read_decode<T: PacketDecode>(&mut self) -> Result<T, DecodeError> {
        T::decode(self)
    }
}

impl fmt::Debug for PacketBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PacketBuf[")?;
        for byte in self.inner.iter().skip(self.read_idx) {
            write!(f, "{} ", byte)?;
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
        let Ok((len, consumed)) = VarInt::decode_iter(&mut data.into_iter()) else {
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
        let Ok((packetbuf, consumed)) = PacketBuf::from_raw_queue(data.into_iter()) else {
            panic!("from_raw_queue was not a success")
        };
        assert_eq!(packetbuf.inner, [
            0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1
        ]);
        assert_eq!(consumed, 17);
    }
}
