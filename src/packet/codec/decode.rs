use super::*;
use std::mem::MaybeUninit;
use std::borrow::Cow;


pub trait PacketDecode<'l> : Sized {
    fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError>;
}


#[derive(Debug, Clone)]
pub enum DecodeError {

    /// The end of the buffer has been reached.
    EndOfBuffer,

    /// The data in the buffer could not be parsed properly.
    ///
    /// Includes a message.
    InvalidData(Cow<'static, str>),

    /// The packet decoder did not consume the length specified in the previously received header.
    UnconsumedBuffer,

    /// The received packet ID did not match any registered packet.
    ///
    /// Includes the ID that wasn't recognised.
    UnknownPacketPrefix(u8)

}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            Self::EndOfBuffer                 => write!(f, "end of buffer"),
            Self::InvalidData(err)            => write!(f, "invalid data: {}", err),
            Self::UnconsumedBuffer            => write!(f, "unconsumed buffer"),
            Self::UnknownPacketPrefix(prefix) => write!(f, "unknown packet prefix {:#04x}", prefix)
        }
    }
}


macro packet_decode_num( $($types:ty),* $(,)? ) { $(
    impl<'l> PacketDecode<'l> for $types { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        Ok(<$types>::from_be_bytes(buf.read_u8s_const()?))
    } }
)* }
packet_decode_num!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

impl<'l> PacketDecode<'l> for bool { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    Ok(buf.read_u8()? != 0)
} }

impl<'l> PacketDecode<'l> for Uuid { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let msb = buf.read_decode::<u64>()?;
    let lsb = buf.read_decode::<u64>()?;
    Ok(Self::from_u64_pair(msb, lsb))
} }

impl<'l> PacketDecode<'l> for String { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let len = buf.read_decode::<VarInt>()?.as_i32() as usize;
    String::from_utf8(buf.read_u8s(len)?).map_err(|_| DecodeError::InvalidData(Cow::Borrowed("String data is not valid UTF8")))
} }

impl<'l, T : PacketDecode<'l>> PacketDecode<'l> for Option<T> { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let is_some = buf.read_u8()? != 0;
    Ok(if (is_some) { Some(buf.read_decode::<T>()?) } else { None })
} }

impl<'l, T : PacketDecode<'l>, const LEN : usize> PacketDecode<'l> for [T; LEN] { fn decode(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
    let mut out: [MaybeUninit<T>; LEN] = unsafe{ MaybeUninit::uninit().assume_init() };
    for item in &mut out { *item = MaybeUninit::new(buf.read_decode::<T>()?); }
    Ok(unsafe { std::mem::transmute_copy(&out) })
} }

impl<'l, A: PacketDecode<'l>, B: PacketDecode<'l>> PacketDecode<'l> for (A, B) {
    fn decode(buf: &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        Ok((A::decode(buf)?, B::decode(buf)?))
    }
}

impl<'l, A: PacketDecode<'l>, B: PacketDecode<'l>, C: PacketDecode<'l>> PacketDecode<'l> for (A, B, C) {
    fn decode(buf: &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        Ok((A::decode(buf)?, B::decode(buf)?, C::decode(buf)?))
    }
}

impl<'l, A: PacketDecode<'l>, B: PacketDecode<'l>, C: PacketDecode<'l>, D: PacketDecode<'l>> PacketDecode<'l> for (A, B, C, D) {
    fn decode(buf: &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        Ok((A::decode(buf)?, B::decode(buf)?, C::decode(buf)?, D::decode(buf)?))
    }
}



pub trait PrefixedPacketDecode<'l> : Sized {
    /// This includes the packet ID and packet data, but **does not include the full packet length**.
    fn decode_prefixed(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError>;
}

impl<'l, T : PacketDecode<'l> + PacketMeta> PrefixedPacketDecode<'l> for T {
    fn decode_prefixed(buf : &mut PacketReader<'l>) -> Result<Self, DecodeError> {
        let packet_id = buf.read_decode::<VarInt>()?.as_i32() as u8;
        if (packet_id != Self::PREFIX) {
            return Err(DecodeError::UnknownPacketPrefix(packet_id));
        }
        Self::decode(buf)
    }
}



#[cfg(test)]
mod tests {
    use crate::packet::c2s::handshake::{ IntentionC2SHandshakePacket, IntendedStage };

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
        let Ok((mut packetbuf, consumed)) = PacketReader::from_raw_queue(data.into_iter()) else { panic!("from_raw_queue was not a success") };
        assert_eq!(packetbuf.as_slice(), [0, 129, 6, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 99, 221, 1]);
        assert_eq!(consumed, 17);

        let Ok((packet_id, consumed)) = VarInt::decode_iter(&mut packetbuf.iter()) else { panic!("decode_iter was not a success") };
        assert_eq!(packet_id.as_i32(), 0);
        packetbuf.skip(consumed);

        let packet = match (IntentionC2SHandshakePacket::decode(&mut packetbuf)) {
            Ok(packet) => packet,
            Err(err) => { panic!("decode was not a success: {:?}", err); }
        };
        assert_eq!(packet.protocol_version.as_i32(), 769);
        assert_eq!(packet.address, "localhost");
        assert_eq!(packet.port, 25565);
        assert_eq!(packet.intended_stage, IntendedStage::Status);
    }

}
