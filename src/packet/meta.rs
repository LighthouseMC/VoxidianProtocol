use crate::packet::c2s::config::C2SConfigPackets;


pub trait PacketPrefix {
    const PREFIX : u8;
}
impl<T : PacketPrefix> PacketPrefix for &T {
    const PREFIX : u8 = T::PREFIX;
}

pub trait PacketMeta {
    const BOUND  : Bound;
    type  BoundT;
    const STAGE  : Stage;
    type  StageT;
}
impl<T : PacketMeta> PacketMeta for &T {
    const BOUND  : Bound = T::BOUND;
    type  BoundT         = T::BoundT;
    const STAGE  : Stage = T::STAGE;
    type  StageT         = T::StageT;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bound {
    S2C,
    C2S
}
pub struct BoundS2C;
pub struct BoundC2S;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Handshake,
    Status,
    Login,
    Config,
    Play
}
pub struct StageHandshake;
pub struct StageStatus;
pub struct StageLogin;
pub struct StageConfig;
pub struct StagePlay;

pub trait IsConfigStage { }
impl<T : Into<C2SConfigPackets>> IsConfigStage for T {}
