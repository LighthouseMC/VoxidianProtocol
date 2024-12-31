use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct RegistryReport {
    map: HashMap<String, RegistryEntries>
}

#[derive(Serialize, Deserialize)]
struct RegistryEntries {
    entries: HashMap<String, RegistryEntry>
}

#[derive(Serialize, Deserialize)]
struct RegistryEntry {
    protocol_id: u32
}

pub(crate) fn make_components() {
    let registries = get_components_json();
    let data_component_types = registries.map.get("minecraft:data_component_type").unwrap();
    let out_string = serde_json::to_string(&data_component_types.entries).unwrap();
    std::fs::write("../generated/data_component_type.json", out_string).unwrap();
}

fn get_components_json() -> RegistryReport {
    let file = std::fs::read_to_string("../datagen/generated/reports/registries.json").unwrap();
    let json = serde_json::from_str::<RegistryReport>(&file).unwrap();
    json
}