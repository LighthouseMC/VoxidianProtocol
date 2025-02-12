use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use voxidian_protocol::value::{BlockState, BlockStateWithMetadata, Identifier};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct BlockStateRegistry {
    pub blocks: HashMap<Identifier, BlockStateMeta>
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockStateMeta {
    pub definition: BlockDefinition,
    pub states: Vec<BlockStateInformation>
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockDefinition {
    #[serde(rename = "type")]
    pub id: Identifier,
}

#[derive(Serialize, Deserialize, Debug)]
struct BlockStateInformation {
    pub id: i32,
    pub default: Option<bool>,
    pub properties: Option<HashMap<String, String>>
}

pub fn make_block_states() {
    println!("Generating block state data...");
    let states = generate_block_states();
    let jsonified = serde_json::to_string(&states).unwrap();

    std::fs::write("../generated/block_states.json", jsonified).unwrap();
}

fn generate_block_states() -> Vec<BlockStateWithMetadata> {
    let file = std::fs::read_to_string("../datagen/generated/reports/blocks.json").unwrap();
    let parsed = serde_json::from_str::<BlockStateRegistry>(&file).unwrap();
    let mut block_states = parsed
        .blocks
        .iter()
        .map(|(block_id, state_meta)| {
            let mut block_states = Vec::new();
            for state in &state_meta.states {
                let properties = state.properties.clone().unwrap_or_default();
                let properties = 
                    properties
                        .iter()
                        .map(|x| (x.0.clone(), x.1.clone()))
                        .collect::<Vec<_>>();
                let block_state = BlockState {
                    id: block_id.clone(),
                    properties
                };
                block_states.push(BlockStateWithMetadata {
                    block_state,
                    protocol_id: state.id
                });
            }
            block_states
        })
        .flatten()
        .collect::<Vec<_>>();

    block_states.sort_by_key(|x| x.protocol_id);

    block_states
}