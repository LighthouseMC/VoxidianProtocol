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

pub enum Bound {
    S2C,
    C2S
}

pub enum Stage {
    Handshake,
    Status,
    Login,
    Config,
    Play,
    Transfer
}