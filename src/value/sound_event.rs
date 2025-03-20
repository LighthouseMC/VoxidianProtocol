use super::*;


#[packet_part]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct SoundEvent {
    pub name        : Identifier,
    pub fixed_range : Option<f32>
}

impl RegValue for SoundEvent {
    const REGISTRY_ID : Identifier = Identifier::new_const("minecraft", "sound_event");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("sound_id", self.name.to_string());
        if let Some(fixed_range) = self.fixed_range {
            nbt.insert("fixed_range", fixed_range);
        }
        Some(Nbt { name: "".into(), root: nbt })
    }
}