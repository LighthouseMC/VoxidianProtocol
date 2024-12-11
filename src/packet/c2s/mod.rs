use super::*;

pub mod handshake;
pub mod status;


pub enum HandshakeC2SPackets {
    Handshake(handshake::HandshakeC2SPacket)
}


pub enum StatusC2SPackets {
    StatusRequest(status::StatusRequestC2SPacket),
    PingRequest(status::PingRequestC2SPacket)
}
