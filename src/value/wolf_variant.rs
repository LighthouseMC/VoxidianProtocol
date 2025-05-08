use super::*;

#[derive(Serialize, Deserialize)]
pub struct WolfVariant {
    pub wild_texture  : Identifier,
    pub tame_texture  : Identifier,
    pub angry_texture : Identifier,
    pub biomes        : Cow<'static, [Identifier]>
}
impl RegValue for WolfVariant {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("wolf_variant");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("wild_texture"  , NbtElement::String(self.wild_texture.to_string()));
        nbt.insert("tame_texture"  , NbtElement::String(self.tame_texture.to_string()));
        nbt.insert("angry_texture" , NbtElement::String(self.angry_texture.to_string()));
        nbt.insert("biomes", NbtElement::List(self.biomes.iter().map(|biome| {
            NbtElement::String(biome.to_string())
        }).collect::<Vec<_>>()));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
