use super::*;

#[packet]
#[derive(Debug)]
pub struct HandshakeC2SPacket {
    pub protocol_version : VarInt,
    pub address          : String,
    pub port             : u16,
    pub intended_state   : ConnectionIntent,
}

#[packet(VarInt)]
#[derive(Debug)]
pub enum ConnectionIntent {
    Status   = 0,
    Login    = 1,
    Transfer = 2,
}
