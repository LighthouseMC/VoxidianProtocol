use super::*;


#[packet_part]
pub struct SoundEvent {
    pub name        : Identifier,
    pub fixed_range : Option<f32>
}
