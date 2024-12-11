pub trait PacketMeta {
    const ID: i32;
    const DIRECTION: Direction;
    const STATE: State;
}

pub enum Direction {
    Clientbound,
    Serverbound
}

pub enum State {
    Handshaking,
    Login,
    Configuration,
    Play,
    Transfer
}