use crate::value::NbtCompound;

pub struct DimType {
    pub fixed_time                      : Option<i64>,
    pub has_skylight                    : bool,
    pub has_ceiling                     : bool,
    pub ultrawarm                       : bool,
    pub natural                         : bool,
    pub coordinate_scale                : f64,
    pub bed_works                       : bool,
    pub respawn_anchor_works            : bool,
    pub min_y                           : i32,
    pub max_y                           : i32,
    pub logical_height                  : u32,
    pub infiniburn                      : String,
    pub effects                         : String,
    pub ambient_light                   : f32,
    pub piglin_safe                     : bool,
    pub has_raids                       : bool,
    pub monster_spawn_level             : i32,
    pub monster_spawn_block_light_limit : i32
}

impl Into<NbtCompound> for DimType {
    fn into(self) -> NbtCompound {
        todo!()
    }
}

impl TryFrom<NbtCompound> for DimType {
    type Error = String;

    fn try_from(_value: NbtCompound) -> Result<Self, Self::Error> {
        todo!()
    }
}