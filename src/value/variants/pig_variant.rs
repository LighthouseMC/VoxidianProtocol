use crate::packet::*;

#[derive(Serialize, Deserialize)]
pub struct PigVariant {
    pub asset_id: Identifier
}
impl RegValue for PigVariant {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("pig_variant");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("asset_id"  , NbtElement::String(self.asset_id.to_string()));
        nbt.insert("spawn_conditionx", NbtElement::List(Vec::new()));
        Some(Nbt { name : String::new(), root : nbt })
    }

}