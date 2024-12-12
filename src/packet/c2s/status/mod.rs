use super::*;


#[packet( prefix = 0x00, bound = C2S, stage = Status )]
pub struct StatusRequestC2SPacket;


#[packet( prefix = 0x01, bound = C2S, stage = Status )]
pub struct PingRequestC2SPacket {
    pub timestamp : u64
}


packet_full_decode!( bound = C2S, stage = Status );
