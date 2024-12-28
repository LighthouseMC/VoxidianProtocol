use super::*;

#[derive(Serialize, Deserialize)]
pub struct DimType {
    pub fixed_time                      : Option<u64>,
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
    pub height                          : u32,
    pub infiniburn                      : String, // TODO: Tag
    pub effects                         : DimEffects,
    pub ambient_light                   : f32,
    pub piglin_safe                     : bool,
    pub has_raids                       : bool,
    pub monster_spawn_light_level       : DimMonsterSpawnLightLevel, // TODO: Figure this out
    pub monster_spawn_block_light_limit : u32
}
impl RegValue for DimType {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("dimension_type");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        if let Some(fixed_time) = self.fixed_time {
            nbt.insert("fixed_time", NbtElement::Long(fixed_time as i64));
        }
        nbt.insert("has_skylight"         , NbtElement::Byte   (if (self.has_skylight         ) { 1 } else { 0 } ));
        nbt.insert("has_ceiling"          , NbtElement::Byte   (if (self.has_ceiling          ) { 1 } else { 0 } ));
        nbt.insert("ultrawarm"            , NbtElement::Byte   (if (self.ultrawarm            ) { 1 } else { 0 } ));
        nbt.insert("natural"              , NbtElement::Byte   (if (self.natural              ) { 1 } else { 0 } ));
        nbt.insert("coordinate_scale"     , NbtElement::Double (self.coordinate_scale                            ));
        nbt.insert("bed_works"            , NbtElement::Byte   (if (self.bed_works            ) { 1 } else { 0 } ));
        nbt.insert("respawn_anchor_works" , NbtElement::Byte   (if (self.respawn_anchor_works ) { 1 } else { 0 } ));
        nbt.insert("min_y"                , NbtElement::Int    (self.min_y                                       ));
        nbt.insert("max_y"                , NbtElement::Int    (self.max_y                                       ));
        nbt.insert("logical_height"       , NbtElement::Int    (self.logical_height as i32                       ));
        nbt.insert("height"               , NbtElement::Int    (self.height as i32                               ));
        nbt.insert("infiniburn"           , NbtElement::String (self.infiniburn.clone()                          ));
        nbt.insert("effects"              , NbtElement::String (self.effects.as_str().to_string()                ));
        nbt.insert("ambient_light"        , NbtElement::Float  (self.ambient_light                               ));
        nbt.insert("piglin_safe"          , NbtElement::Byte   (if (self.piglin_safe          ) { 1 } else { 0 } ));
        nbt.insert("has_raids"            , NbtElement::Byte   (if (self.has_raids            ) { 1 } else { 0 } ));
        nbt.insert("monster_spawn_light_level"       , NbtElement::Compound(self.monster_spawn_light_level.to_nbt()     ));
        nbt.insert("monster_spawn_block_light_limit" , NbtElement::Int     (self.monster_spawn_block_light_limit as i32 ));
        Some(Nbt { name : String::new(), root : nbt })
    }

}


#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum DimEffects {
    #[serde(rename = "minecraft:overworld")]
    Overworld,
    #[serde(rename = "minecraft:the_nether")]
    Nether,
    #[serde(rename = "minecraft:the_end")]
    End
}
impl DimEffects { fn as_str(&self) -> &'static str { match (self) {
    Self::Overworld => "minecraft:overworld",
    Self::Nether    => "minecraft:the_nether",
    Self::End       => "minecraft:the_end"
} } }

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DimMonsterSpawnLightLevel {
    Constant(u32),
    Uniform {
        min_inclusive : u32,
        max_inclusive : u32
    },
    BiasedToBottom {
        min_inclusive : u32,
        max_inclusive : u32
    },
    Clamped {
        min_inclusive : u32,
        max_inclusive : u32,
        source       : Box<DimMonsterSpawnLightLevel>
    },
    ClampedNormal {
        mean          : f32,
        deviation     : f32,
        min_inclusive : u32,
        max_inclusive : u32,
    },
    WeightedList {
        distribution : Vec<(DimMonsterSpawnLightLevel, i32)>
    }
}
impl DimMonsterSpawnLightLevel {
    pub fn to_nbt(&self) -> NbtCompound {
        let mut nbt = NbtCompound::new();
        match (self) {
            DimMonsterSpawnLightLevel::Constant(value) => {
                nbt.insert("type", NbtElement::String("constant".to_string()));
                nbt.insert("value", NbtElement::Int(*value as i32));
            },
            DimMonsterSpawnLightLevel::Uniform { min_inclusive, max_inclusive } => {
                nbt.insert("type", NbtElement::String("uniform".to_string()));
                nbt.insert("min_inclusive", NbtElement::Int(*min_inclusive as i32));
                nbt.insert("max_inclusive", NbtElement::Int(*max_inclusive as i32));
            },
            DimMonsterSpawnLightLevel::BiasedToBottom { min_inclusive, max_inclusive } => {
                nbt.insert("type", NbtElement::String("biased_to_bottom".to_string()));
                nbt.insert("min_inclusive", NbtElement::Int(*min_inclusive as i32));
                nbt.insert("max_inclusive", NbtElement::Int(*max_inclusive as i32));
            },
            DimMonsterSpawnLightLevel::Clamped { min_inclusive, max_inclusive, source } => {
                nbt.insert("type", NbtElement::String("clamped".to_string()));
                nbt.insert("min_inclusive", NbtElement::Int(*min_inclusive as i32));
                nbt.insert("max_inclusive", NbtElement::Int(*max_inclusive as i32));
                nbt.insert("source", NbtElement::Compound(source.to_nbt()))
            },
            DimMonsterSpawnLightLevel::ClampedNormal { mean, deviation, min_inclusive, max_inclusive } => {
                nbt.insert("type", NbtElement::String("clamped_normal".to_string()));
                nbt.insert("mean", NbtElement::Float(*mean));
                nbt.insert("deviation", NbtElement::Float(*deviation));
                nbt.insert("min_inclusive", NbtElement::Int(*min_inclusive as i32));
                nbt.insert("max_inclusive", NbtElement::Int(*max_inclusive as i32));
            },
            DimMonsterSpawnLightLevel::WeightedList { distribution } => {
                nbt.insert("type", NbtElement::String("weighted_list".to_string()));
                nbt.insert("distribution", NbtElement::List(distribution.iter().map(|(data, weight)| {
                    let mut nbt = NbtCompound::new();
                    nbt.insert("data", NbtElement::Compound(data.to_nbt()));
                    nbt.insert("weight", NbtElement::Int(*weight));
                    NbtElement::Compound(nbt)
                }).collect::<Vec<_>>()))
            },
        }
        nbt
    }
}
