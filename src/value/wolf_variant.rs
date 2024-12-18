use super::*;


pub struct WolfVariant {
    pub wild_tex  : Identifier,
    pub tame_tex  : Identifier,
    pub angry_tex : Identifier,
    pub biomes    : Vec<Identifier>
}
impl RegValue for WolfVariant {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("wolf_variant");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("wild_texture"  , NbtElement::String(self.wild_tex  .to_string()));
        nbt.insert("tame_texture"  , NbtElement::String(self.tame_tex  .to_string()));
        nbt.insert("angry_texture" , NbtElement::String(self.angry_tex .to_string()));
        nbt.insert("biomes", NbtElement::List(self.biomes.iter().map(|biome| {
            NbtElement::String(biome.to_string())
        }).collect::<Vec<_>>()));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
