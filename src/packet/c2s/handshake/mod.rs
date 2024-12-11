use crate::packet::meta::{Direction, PacketMeta, State};
use super::*;

#[packet]
pub struct HandshakeC2SPacket {
    pub protocol_version : VarInt,
    pub address          : String,
    pub port             : u16,
    pub intended_state   : ConnectionIntent
}

// TODO: Add this as a part of #[packet] for structs
// for example: #[packet(id = 0x01, direction = Direction::Serverbound, state = State::Handshaking)]
impl PacketMeta for HandshakeC2SPacket {
    const ID: i32 = 0x01;
    const DIRECTION: Direction = Direction::Serverbound;
    const STATE: State = State::Handshaking;
}

#[packet(VarInt)]
pub enum ConnectionIntent {
    Login    = 0,
    Status   = 1,
    Transfer = 2,
}
