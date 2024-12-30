use super::*;

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct BlockState {
    pub id: Identifier,
    pub properties: Vec<(String, String)>
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockStateWithMetadata {
    pub block_state: BlockState,
    pub protocol_id: i32
}