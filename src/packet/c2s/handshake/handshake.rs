use super::*;

#[packet]
pub struct HandshakeC2SPacket {
    pub protocol_version: VarInt,
    pub address: String,
    pub port: u16,
    pub intended_state: ConnectionIntent,
}

#[packet(VarInt)]
pub enum ConnectionIntent {
    Status = 0,
    Login = 1,
    Transfer = 2,
}
