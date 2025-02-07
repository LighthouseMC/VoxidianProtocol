use super::*;


#[packet_part]
#[derive(Clone)]
pub struct SoundEvent {
    pub name        : Identifier,
    pub fixed_range : Option<f32>
}
