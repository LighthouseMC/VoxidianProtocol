use super::*;


pub trait TryIntoC2SStatusPackets : Sized {
    fn try_into_c2s_status(self) -> Result<C2SStatusPackets, Self>;
}


#[packet( "minecraft:c2s/status/status_request" )]
pub struct StatusRequestC2SStatusPacket;


#[packet( "minecraft:c2s/status/ping_request" )]
pub struct PingRequestC2SStatusPacket {
    pub timestamp : u64
}


packet_full_decode!{ C2S Status }
