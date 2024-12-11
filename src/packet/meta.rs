pub trait PacketMeta {
    const PREFIX : usize;
    const BOUND  : Bound;
    const STAGE  : Stage;
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