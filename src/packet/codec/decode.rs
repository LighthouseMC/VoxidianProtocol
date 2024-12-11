use super::*;


pub trait PacketDecode : Sized {
    fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError>;
}


#[derive(Debug)]
pub enum DecodeError {

    /// The end of the buffer has been reached.
    EndOfBuffer,

    /// The data in the buffer could not be parsed properly.
    InvalidData,

}


macro packet_decode_int( $($types:ty),* $(,)? ) { $(
    impl PacketDecode for $types { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
        Ok(<$types>::from_be_bytes(buf.read_u8s_const()?))
    } }
)* }
packet_decode_int!(u8, i8, u16, i16, u32, i32, u64, i64);

impl PacketDecode for bool { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    Ok(buf.read_u8()? != 0)
} }

impl PacketDecode for String { fn decode(buf : &mut PacketBuf) -> Result<Self, DecodeError> {
    let len = buf.read_decode::<VarInt>()?.as_i32() as usize;
    Ok(String::from_utf8(buf.read_u8s(len)?).map_err(|_| DecodeError::InvalidData)?)
} }


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
