use crate::buffer::PacketBuf;

pub trait PacketCodec {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError>;
    fn decode(buf: &PacketBuf) -> Result<Self, DecodeError>;
}

pub enum DecodeError {
    /// Indicates the end of the buffer has been reached.
    BufferOverflow
}

pub enum EncodeError {

}

impl PacketCodec for u8 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_byte(*self)
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        buf.read_byte()
    }
}

impl PacketCodec for i8 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_byte(*self as u8)
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        buf.read_byte().map(|x| x as i8)
    }
}

impl PacketCodec for u16 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_all(&self.to_be_bytes())
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(u16::from_be_bytes([
            buf.read_byte()?,
            buf.read_byte()?,
        ]))
    }
}

impl PacketCodec for i16 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_all(&self.to_be_bytes())
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(i16::from_be_bytes([
            buf.read_byte()?,
            buf.read_byte()?,
        ]))
    }
}

impl PacketCodec for u32 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_all(&self.to_be_bytes())
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(u32::from_be_bytes([
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
        ]))
    }
}

impl PacketCodec for i32 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_all(&self.to_be_bytes())
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(i32::from_be_bytes([
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
        ]))
    }
}


impl PacketCodec for u64 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_all(&self.to_be_bytes())
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(u64::from_be_bytes([
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
        ]))
    }
}

impl PacketCodec for i64 {
    fn encode(&self, buf: &mut PacketBuf) -> Result<(), EncodeError> {
        buf.write_all(&self.to_be_bytes())
    }

    fn decode(buf: &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(i64::from_be_bytes([
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
            buf.read_byte()?,
        ]))
    }
}

pub mod tests {
    use crate::buffer::PacketBuf;
    use crate::codec::PacketCodec;

    // TODO: write tests
}