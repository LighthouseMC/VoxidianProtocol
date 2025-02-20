use super::*;


pub trait TryIntoC2SHandshakePackets {
    fn try_into_c2s_handshake(self) -> Option<C2SHandshakePackets>;
}


#[packet( "minecraft:c2s/handshake/intention" )]
pub struct IntentionC2SHandshakePacket {
    pub protocol_version : VarInt,
    pub address          : String,
    pub port             : u16,
    pub intended_stage   : IntendedStage
}

#[packet_part(VarInt)]
pub enum IntendedStage {
    Status   = 1,
    Login    = 2,
    Transfer = 3,
}
impl IntendedStage {
    pub fn into_stage(&self) -> Stage { match (self) {
        Self::Login    => Stage::Login,
        Self::Status   => Stage::Status,
        Self::Transfer => Stage::Transfer
    } }
}


packet_full_decode!{ C2S Handshake }
