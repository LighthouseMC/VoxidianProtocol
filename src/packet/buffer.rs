use super::*;


pub struct PacketBuf {
    inner    : Vec<u8>,
    read_idx : usize
}


/// Constructors
impl PacketBuf {

    pub fn new() -> Self { PacketBuf {
        inner    : Vec::new(),
        read_idx : 0
    } }

    pub fn of_encode<T : PacketEncode>(encodable : T) -> Result<Self, EncodeError> {
        let mut buf = Self::new();
        encodable.encode(&mut buf)?;
        Ok(buf)
    }

}


/// Basic Operations
impl PacketBuf {

    #[track_caller]
    pub fn seek(&mut self, idx : usize) {
        assert!(idx < self.inner.len(), "Seek index exceeded packet size");
        self.read_idx = idx;
    }

    pub fn write_u8(&mut self, byte : u8) -> () {
        self.inner.push(byte);
    }

    pub fn write_u8s(&mut self, data : &[u8]) -> () {
        self.inner.extend_from_slice(data);
    }

    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        let byte = self.inner.get(self.read_idx).ok_or(DecodeError::EndOfBuffer)?;
        self.read_idx += 1;
        Ok(*byte)
    }

    pub fn read_u8s_const<const BYTES : usize>(&mut self) -> Result<[u8; BYTES], DecodeError> {
        let data = self.inner.array_chunks().next().ok_or(DecodeError::EndOfBuffer)?;
        self.read_idx += BYTES;
        Ok(*data)
    }

    pub fn read_u8s(&mut self, bytes : usize) -> Result<&[u8], DecodeError> {
        let data = self.inner.chunks(bytes).next().ok_or(DecodeError::EndOfBuffer)?;
        self.read_idx += bytes;
        Ok(data)
    }

}


/// Encode & Decode
impl PacketBuf {

    pub fn encode_write<T : PacketEncode>(&mut self, encodable : T) -> Result<(), EncodeError> {
        encodable.encode(self)
    }

    pub fn read_decode<T : PacketDecode>(&mut self) -> Result<T, DecodeError> {
        T::decode(self)
    }

}
