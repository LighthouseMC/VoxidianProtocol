use std::{collections::HashMap, fmt::format};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PacketPhases {
    handshake: PacketFlow,
    login: PacketFlow,
    configuration: PacketFlow,
    status: PacketFlow,
    play: PacketFlow
}

#[derive(Serialize, Deserialize, Debug)]
struct PacketFlow {
    clientbound: Option<HashMap<String, PacketEntry>>,
    serverbound: HashMap<String, PacketEntry>
}

#[derive(Serialize, Deserialize, Debug)]
struct PacketEntry {
    protocol_id: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct PacketOutput {
    map: HashMap<String, PacketOutputMeta>
}

#[derive(Serialize, Deserialize, Debug)]
struct PacketOutputMeta {
    prefix: u32,
    bound: Bound,
    stage: Stage
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum Bound {
    S2C,
    C2S
}

impl ToString for Bound {
    fn to_string(&self) -> String {
        match self {
            Bound::S2C => "s2c",
            Bound::C2S => "c2s",
        }.to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum Stage {
    Handshake,
    Login,
    Status,
    Config,
    Play
}

impl ToString for Stage {
    fn to_string(&self) -> String {
        match self {
            Stage::Handshake => "handshake",
            Stage::Login => "login",
            Stage::Status => "status",
            Stage::Config => "config",
            Stage::Play => "play",
        }.to_string()
    }
}

pub(crate) fn make_packets() {
    let packets = get_packets_json();
    let mut output = PacketOutput {
        map: HashMap::new(),
    };

    transform_entry(&packets.handshake, &mut output, Stage::Handshake);
    transform_entry(&packets.login, &mut output, Stage::Login);
    transform_entry(&packets.status, &mut output, Stage::Status);
    transform_entry(&packets.configuration, &mut output, Stage::Config);
    transform_entry(&packets.play, &mut output, Stage::Play);

    std::fs::write("../generated/packets.json", serde_json::to_string(&output).unwrap()).unwrap();
}

fn transform_entry(entry: &PacketFlow, output: &mut PacketOutput, stage: Stage) {
    if let Some(ec) = &entry.clientbound {
        transform_flow(ec, output, stage, Bound::S2C);
    }
    transform_flow(&entry.serverbound, output, stage, Bound::C2S);
}

fn transform_flow(entry: &HashMap<String, PacketEntry>, output: &mut PacketOutput, stage: Stage, bound: Bound) {
    for (name, entry) in entry.iter() {
        let name = name.replace("minecraft:", "");
        let name = format!("minecraft:{}/{}/{}", bound.to_string(), stage.to_string(), name);
        output.map.insert(
            name,
            PacketOutputMeta { prefix: entry.protocol_id, bound, stage }
        );
    }
}

fn get_packets_json() -> PacketPhases {
    let file = std::fs::read_to_string("../datagen/generated/reports/packets.json").unwrap();
    let json = serde_json::from_str::<PacketPhases>(&file).unwrap();
    json
}