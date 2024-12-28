use super::*;

#[derive(Debug)]
pub struct Item {
    pub id : Identifier
}

impl Item {
    pub fn from_identifier(id: Identifier) -> Item {
        Item { id }
    }
}
