use super::*;


pub struct BannerPattern {
    pub asset     : Identifier,
    pub translate : String
}
impl RegValue for BannerPattern {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("banner_pattern");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("asset_id"        , NbtElement::String (self.asset.to_string() ));
        nbt.insert("translation_key" , NbtElement::String (self.translate.clone() ));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
