use super::*;


#[packet_part]
#[derive(PartialEq)]
pub struct SoundEvent {
    pub name        : Identifier,
    pub fixed_range : Option<f32>
}

impl RegValue for SoundEvent {
    const REGISTRY_ID : Identifier = Identifier::new_const("minecraft", "sound_event");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        todo!()
    }
}