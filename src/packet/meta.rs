pub trait PacketMeta {
    const PREFIX : usize;
    const BOUND  : Bound;
    const STAGE  : Stage;
}
impl<T : PacketMeta> PacketMeta for &T {
    const PREFIX : usize = Self::PREFIX;
    const BOUND  : Bound = Self::BOUND;
    const STAGE  : Stage = Self::STAGE;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bound {
    S2C,
    C2S
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Handshake,
    Status,
    Login,
    Config,
    Play,
    Transfer
}