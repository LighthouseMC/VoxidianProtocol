use crate::packet::*;

#[derive(Serialize, Deserialize)]
pub struct ChickenVariant {
    pub model: EntityModelType,
    pub asset_id: Identifier,
}
impl RegValue for ChickenVariant {
    const REGISTRY_ID: Identifier = Identifier::vanilla_const("chicken_variant");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("model_type", self.model.to_nbt());
        nbt.insert("asset_id", NbtElement::String(self.asset_id.to_string()));
        nbt.insert("spawn_condition", NbtElement::List(Vec::new()));
        Some(Nbt {
            name: String::new(),
            root: nbt,
        })
    }
}
