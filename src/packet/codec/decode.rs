use super::*;
use std::mem::MaybeUninit;


pub trait PacketDecode : Sized {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError>;
}


#[derive(Debug)]
pub enum DecodeError {

    /// The end of the buffer has been reached.
    EndOfBuffer,

    /// The data in the buffer could not be parsed properly.
    InvalidData,

    /// The packet decoder did not consume the length specified in the previously received header.
    UnconsumedBuffer,

    /// The received packet ID did not match any registered packet.
    UnknownPacketPrefix

}


macro packet_decode_num( $($types:ty),* $(,)? ) { $(
    impl PacketDecode for $types { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(<$types>::from_be_bytes(buf.read_u8s_const()?))
    } }
)* }
packet_decode_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl PacketDecode for bool { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    Ok(buf.read_u8()? != 0)
} }

impl PacketDecode for Uuid { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let msb = buf.read_decode::<u64>()?;
    let lsb = buf.read_decode::<u64>()?;
    Ok(Self::from_u64_pair(msb, lsb))
} }

impl PacketDecode for String { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let len = buf.read_decode::<VarInt>()?.as_i32() as usize;
    Ok(String::from_utf8(buf.read_u8s(len)?).map_err(|_| DecodeError::InvalidData)?)
} }

impl<T : PacketDecode> PacketDecode for Option<T> { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let is_some = buf.read_u8()? != 0;
    Ok(if (is_some) { Some(buf.read_decode::<T>()?) } else { None })
} }

impl<T : PacketDecode, const LEN : usize> PacketDecode for [T; LEN] { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let mut out : Self = unsafe{ MaybeUninit::uninit().assume_init() };
    for i in 0..LEN { out[i] = buf.read_decode::<T>()?; }
    Ok(out)
} }



pub trait PacketDecodeFull : Sized {

    /// Decode part of the packet.
    /// This includes:
    /// - Packet prefix/ID (VarInt)
    /// - Packet data (...)
    /// This **DOES NOT** include the packet length.
    /// 
    /// See `decode_full_from_queue`.
    fn decode_partial(buf : &mut PacketBuf) -> Result<Self, DecodeError>;

    /// Decode the full packet, reading from a queue of bytes.
    /// This includes:
    /// - Packet length (VarInt)
    /// - Packet prefix/ID (VarInt)
    /// - Packet data (...)
    /// 
    /// Also returns the number of bytes that were consumed.
    fn decode_full_from_queue(queue : impl Iterator<Item = u8>) -> Result<(Self, usize), DecodeError> {
        let (mut buf, consumed) = PacketBuf::from_raw_queue(queue)?;
        let out = Self::decode_partial(&mut buf)?;
        if (buf.remaining() != 0) {
            Err(DecodeError::UnconsumedBuffer)
        } else {
            Ok((out, consumed))
        }
    }

}



#[cfg(test)]
mod tests {
    use crate::packet::c2s::handshake::{ HandshakeC2SPacket, IntendedStage };

    use super::*;

    #[test]
    fn basic_decoding() {
        let data = [16, 0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1, 1, 0];
        //          |   |  |-----  |  |----------------------------------------  |------  |  |  ^Status request packet
        //          |   |  |       |  |                                          |        |  ^Packet length
        //          |   |  |       |  |                                          |        ^Intended state
        //          |   |  |       |  ^Address UTF8 data                         ^Port    |
        //          |   |  |       ^Address length                                        |
        //          |   |  ^Protocol version                                              |
        //          |   |-----------------------------------------------------------------'
        //          |   ^Handshake packet
        //          ^Packet length
        let Ok((mut packetbuf, consumed)) = PacketBuf::from_raw_queue(data.into_iter()) else { panic!("from_raw_queue was not a success") };
        assert_eq!(packetbuf.as_slice(), [0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1]);
        assert_eq!(consumed, 17);

        let Ok((packet_id, consumed)) = VarInt::decode_iter(&mut packetbuf.iter()) else { panic!("decode_iter was not a success") };
        assert_eq!(packet_id.as_i32(), 0);
        packetbuf.skip(consumed);

        let packet = match (HandshakeC2SPacket::decode(&mut packetbuf)) {
            Ok(packet) => packet,
            Err(err) => { panic!("decode was not a success: {:?}", err); }
        };
        assert_eq!(packet.protocol_version.as_i32(), 769);
        assert_eq!(packet.address, "localhost");
        assert_eq!(packet.port, 25565);
        assert_eq!(packet.intended_stage, IntendedStage::Status);
    }

}
