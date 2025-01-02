use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub(crate) struct RegistryReport {
    pub(crate) map: HashMap<String, RegistryEntries>
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RegistryEntries {
    pub(crate) entries: HashMap<String, RegistryEntry>
}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct RegistryEntry {
    pub(crate) protocol_id: u32
}

pub(crate) fn make_components() {
    let registries = get_reports_json();
    let data_component_types = registries.map.get("minecraft:data_component_type").unwrap();
    let out_string = serde_json::to_string(&data_component_types.entries).unwrap();
    std::fs::write("../generated/data_component_type.json", out_string).unwrap();
}

pub(crate) fn get_reports_json() -> RegistryReport {
    let file = std::fs::read_to_string("../datagen/generated/reports/registries.json").unwrap();
    let json = serde_json::from_str::<RegistryReport>(&file).unwrap();
    json
}