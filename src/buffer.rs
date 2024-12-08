use crate::codec::{DecodeError, EncodeError, PacketCodec};

#[repr(transparent)]
pub struct PacketBuf {
    inner: Vec<u8>,
    read_idx: usize
}

impl PacketBuf {
    pub fn new() -> Self {
        PacketBuf { inner: Vec::new(), read_idx: 0 }
    }

    pub fn reset_read(&mut self) {
        self.read_idx = 0;
    }

    pub fn write<T: PacketCodec>(&mut self, val: T) -> Result<(), EncodeError> {
        val.encode(self)
    }

    pub fn write_byte(&mut self, val: u8) -> Result<(), EncodeError> {
        self.inner.push(val);
        Ok(())
    }

    pub fn write_all(&mut self, slice: &[u8]) -> Result<(), EncodeError> {
        for byte in slice {
            self.inner.push(*byte);
        }
        Ok(())
    }

    pub fn read_byte(&mut self) -> Result<u8, DecodeError> {
        self.read_idx += 1;
        self.inner.get(self.read_idx - 1)
            .map(|x| *x)
            .ok_or(DecodeError::BufferOverflow)
    }
}