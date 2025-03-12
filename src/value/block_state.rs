use std::collections::{BTreeMap};

use super::*;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct BlockState {
    pub id: Identifier,
    pub properties: BTreeMap<String, String>
}

impl std::hash::Hash for BlockState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        for property in &self.properties {
            property.0.hash(state);
            property.1.hash(state);
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockStateWithMetadata {
    pub block_state: BlockState,
    pub protocol_id: i32
}

