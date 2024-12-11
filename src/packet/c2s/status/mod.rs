use super::*;


#[packet]
pub struct StatusRequestC2SPacket;


#[packet]
pub struct PingRequestC2SPacket(u64);
