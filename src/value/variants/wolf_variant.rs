use crate::packet::*;

#[derive(Serialize, Deserialize)]
pub struct WolfVariant {
    pub wild_texture  : Identifier,
    pub tame_texture  : Identifier,
    pub angry_texture : Identifier,
}
impl RegValue for WolfVariant {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("wolf_variant");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        let mut assets = NbtCompound::new();
        assets.insert("wild"  , NbtElement::String(self.wild_texture.to_string()));
        assets.insert("tame"  , NbtElement::String(self.tame_texture.to_string()));
        assets.insert("angry" , NbtElement::String(self.angry_texture.to_string()));
        nbt.insert("spawn_condition", NbtElement::List(Vec::new()));
        nbt.insert("assets", NbtElement::Compound(assets));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
