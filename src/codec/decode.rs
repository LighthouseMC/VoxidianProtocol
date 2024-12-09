use super::*;


pub trait PacketDecode : Sized {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError>;
}


pub enum DecodeError {

    /// The end of the buffer has been reached.
    EndOfBuffer,

    /// The data in the buffer could not be parsed properly.
    InvalidData

}


macro packet_decode_int( $($types:ty),* $(,)? ) { $(
    impl PacketDecode for $types { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(<$types>::from_be_bytes(buf.read_u8s_const()?))
    } }
)* }
packet_decode_int!( u8, i8, u16, i16, u32, i32, u64, i64 );

impl PacketDecode for String { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let len = Into::<i32>::into(buf.read_decode::<VarInt>()?) as usize;
    Ok(String::from_utf8(buf.read_u8s(len)?.to_vec()).map_err(|_| DecodeError::InvalidData)?)
} }
