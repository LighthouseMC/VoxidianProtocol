use super::*;


#[packet( prefix = 0x00, bound = C2S, stage = Handshake )]
pub struct HandshakeC2SPacket {
    pub protocol_version : VarInt,
    pub address          : String,
    pub port             : u16,
    pub intended_stage   : IntendedStage
}

// TODO: Add this as a part of #[packet] for structs
// for example: #[packet(id = 0x01, direction = Direction::Serverbound, state = State::Handshaking)]
/*impl PacketMeta for HandshakeC2SPacket {
    const ID: i32 = 0x01;
    const DIRECTION: Direction = Direction::Serverbound;
    const STATE: State = State::Handshaking;
}*/

#[packet_enum(VarInt)]
pub enum IntendedStage {
    Login    = 0,
    Status   = 1,
    Transfer = 2,
}
