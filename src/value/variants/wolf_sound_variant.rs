use crate::packet::*;

#[derive(Serialize, Deserialize)]
pub struct WolfSoundVariant {
    pub ambient: SoundEvent,
    pub death: SoundEvent,
    pub growl: SoundEvent,
    pub hurt: SoundEvent,
    pub pant: SoundEvent,
    pub whine: SoundEvent,
}
impl RegValue for WolfSoundVariant {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("wolf_sound_variant");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("ambient_sound", NbtElement::Compound(self.ambient.to_registry_data_packet()?.root));
        nbt.insert("death_sound", NbtElement::Compound(self.death.to_registry_data_packet()?.root));
        nbt.insert("growl_sound", NbtElement::Compound(self.growl.to_registry_data_packet()?.root));
        nbt.insert("hurt_sound", NbtElement::Compound(self.hurt.to_registry_data_packet()?.root));
        nbt.insert("pant_sound", NbtElement::Compound(self.pant.to_registry_data_packet()?.root));
        nbt.insert("whine_sound", NbtElement::Compound(self.whine.to_registry_data_packet()?.root));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
