use std::{collections::HashMap, sync::LazyLock};

use crate::value::{BlockState, BlockStateWithMetadata};

pub static BLOCK_STATE_JSON: &'static str = include_str!("../../generated/block_states.json");

pub static BLOCK_STATES: LazyLock<Vec<BlockStateWithMetadata>> = LazyLock::new(|| {
    serde_json::from_str(&BLOCK_STATE_JSON).unwrap()
});



pub static ID_TO_BLOCK_STATE: LazyLock<HashMap<i32, BlockState>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    for state in BLOCK_STATES.iter() {
        let state2 = state.block_state.clone().sorted();
        map.insert(state.protocol_id, state2);
    }

    map
});

pub static BLOCK_STATE_TO_ID: LazyLock<HashMap<BlockState, i32>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    for state in BLOCK_STATES.iter() {
        map.insert(state.block_state.clone().sorted(), state.protocol_id);
    }

    map
});

impl BlockState {
    pub fn all_block_states() -> Vec<BlockState> {
        BLOCK_STATES.iter().map(|state| state.block_state.clone()).collect()
    }

    pub fn from_id(id: i32) -> Option<BlockState> {
        ID_TO_BLOCK_STATE.get(&id).cloned()
    }

    pub fn sort(&mut self) {
        self.properties.sort_by_cached_key(|x| x.0.clone());
    }

    pub fn sorted(mut self) -> Self {
        self.sort();
        self
    }

    pub fn to_id(&self) -> Option<i32> {
        let state = self.clone().sorted();
        BLOCK_STATE_TO_ID.get(&state).cloned()
    }

    pub fn to_id_mut(&mut self) -> Option<i32> {
        self.sort();
        BLOCK_STATE_TO_ID.get(self).cloned()
    }
}