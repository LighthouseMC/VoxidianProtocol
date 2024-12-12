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
impl IntendedStage {
    pub fn into_stage(&self) -> Stage { match (self) {
        Self::Login    => Stage::Login,
        Self::Status   => Stage::Status,
        Self::Transfer => Stage::Transfer
    } }
}


packet_full_decode!( bound = C2S, stage = Handshake );
