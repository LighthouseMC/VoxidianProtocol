use super::*;


pub struct PaintingVariant {
    pub asset  : Identifier,
    pub height : u32,
    pub width  : u32
}
impl RegValue for PaintingVariant {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("painting_variant");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("asset"  , NbtElement::String (self.asset.to_string() ));
        nbt.insert("height" , NbtElement::Int    (self.height as i32     ));
        nbt.insert("width"  , NbtElement::Int    (self.width  as i32     ));
        Some(Nbt { name : String::new(), root : nbt })
    }

}
