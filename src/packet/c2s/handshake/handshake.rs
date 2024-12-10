use super::*;

#[packet]
pub struct HandshakeC2SPacket {
    protocol_version: VarInt,
    address: String,
    port: u16,
    intended_state: ConnectionIntent,
}

#[packet(VarInt)]
pub enum ConnectionIntent {
    Status = 0,
    Login = 1,
    Transfer = 2,
}
