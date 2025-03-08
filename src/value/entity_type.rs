use super::*;

#[packet_part]
#[derive(PartialEq)]

pub struct EntityType {
    pub id : Identifier
}

impl RegValue for EntityType {
    const REGISTRY_ID : Identifier = Identifier::new_const("minecraft", "entity_type");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        todo!()
    }
}