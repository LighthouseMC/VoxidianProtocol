use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BannerPattern {
    pub asset_id        : Identifier,
    pub translation_key : String
}
impl RegValue for BannerPattern {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("banner_pattern");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("asset_id"        , NbtElement::String (self.asset_id.to_string() ));
        nbt.insert("translation_key" , NbtElement::String (self.translation_key.clone() ));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
