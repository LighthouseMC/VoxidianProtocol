use super::*;

#[packet_part]
#[derive(Clone)]
pub struct Block {
    pub id : Identifier
}

impl RegValue for Block {
    const REGISTRY_ID : Identifier = Identifier::new_const("minecraft", "block");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        unimplemented!()
    }
}