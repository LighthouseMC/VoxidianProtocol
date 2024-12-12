use crate::value::{NbtCompound, NbtElement};

pub struct DimensionType {
    fixed_time: Option<i64>,
    has_skylight: bool,
    has_ceiling: bool,
    ultrawarm: bool,
    natural: bool,
    coordinate_scale: f64,
    bed_works: bool,
    respawn_anchor_works: bool,
    min_y: i32,
    max_y: i32,
    logical_height: u32,
    infiniburn: String,
    effects: String,
    ambient_light: f32,
    piglin_safe: bool,
    has_raids: bool,
    monster_spawn_level: i32,
    monster_spawn_block_light_limit: i32
}

impl Into<NbtCompound> for DimensionType {
    fn into(self) -> NbtCompound {
        todo!()
    }
}

impl TryFrom<NbtCompound> for DimensionType {
    type Error = String;

    fn try_from(value: NbtCompound) -> Result<Self, Self::Error> {
        todo!()
    }
}