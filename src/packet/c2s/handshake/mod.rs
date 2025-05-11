use super::*;


pub trait TryIntoC2SHandshakePackets {
    fn try_into_c2s_handshake(self) -> Option<C2SHandshakePackets>;
}


#[packet( "minecraft:c2s/handshake/intention" )]
pub struct IntentionC2SHandshakePacket {
    pub protocol_version : Var32,
    pub address          : String,
    pub port             : u16,
    pub intended_stage   : IntendedStage
}

#[packet_part(Var32)]
pub enum IntendedStage {
    Status   = 1,
    Login    = 2,
    Transfer = 3,
}
impl IntendedStage {
    pub fn into_stage(&self) -> Stage { match (self) {
        Self::Login    => Stage::Login,
        Self::Status   => Stage::Status,
        Self::Transfer => Stage::Login
    } }
}


packet_full_decode!{ C2S Handshake }
