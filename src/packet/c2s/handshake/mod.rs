use super::*;


#[packet( prefix = 0x00, bound = C2S, stage = Handshake )]
pub struct HandshakeC2SPacket {
    pub protocol_version : VarInt,
    pub address          : String,
    pub port             : u16,
    pub intended_stage   : IntendedStage
}

#[packet_enum(VarInt)]
pub enum IntendedStage {
    Login    = 0,
    Status   = 1,
    Transfer = 2,
}


packet_full_decode!( bound = C2S, stage = Handshake );
