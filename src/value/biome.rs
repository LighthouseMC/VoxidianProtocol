use std::vec;

use super::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    pub has_precipitation      : bool,
    pub temperature            : f32,
    pub temperature_modifier   : Option<BiomeTempModifier>,
    pub downfall               : f32,
    pub effects                : BiomeEffects
}
impl RegValue for Biome {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("worldgen/biome");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("has_precipitation" , NbtElement::Byte  (if (self.has_precipitation) { 1 } else { 0 }));
        nbt.insert("temperature"       , NbtElement::Float (self.temperature));
        if let Some(temp_modifier) = self.temperature_modifier {
            nbt.insert("temperature_modifier", NbtElement::String(temp_modifier.as_str().to_string()));
        }
        nbt.insert("downfall", NbtElement::Float(self.downfall));
        nbt.insert("effects", {
            let mut nbt = NbtCompound::new();
            nbt.insert("fog_color"       , NbtElement::Int(self.effects.fog_color       .to_int()));
            nbt.insert("water_color"     , NbtElement::Int(self.effects.water_color     .to_int()));
            nbt.insert("water_fog_color" , NbtElement::Int(self.effects.water_fog_color .to_int()));
            nbt.insert("sky_color"       , NbtElement::Int(self.effects.sky_color       .to_int()));
            if let Some(foliage_color) = &self.effects.foliage_color {
                nbt.insert("foliage_color"   , NbtElement::Int(foliage_color.to_int()));
            }
            if let Some(grass_color) = &self.effects.grass_color {
                nbt.insert("grass_color"     , NbtElement::Int(grass_color.to_int()));
            }
            if let Some(grass_colour_modifier) = &self.effects.grass_color_modifier {
                nbt.insert("grass_color_modifier", NbtElement::String(grass_colour_modifier.as_str().to_string()))
            }
            if let Some(particle) = &self.effects.particle {
                nbt.insert("particle", {
                    let mut nbt = NbtCompound::new();
                    nbt.insert("options"     , NbtElement::Compound (particle.options.to_nbt() ));
                    nbt.insert("probability" , NbtElement::Float    (particle.probability      ));
                    NbtElement::Compound(nbt)
                });
            }
            if let Some(ambient_sound) = &self.effects.ambient_sound {
                match ambient_sound {
                    BiomeAmbientSound::Id(identifier) => {
                        nbt.insert("ambient_sound", NbtElement::String(identifier.to_string()));
                    },
                    BiomeAmbientSound::Ranged { sound, range } => {
                        nbt.insert("ambient_sound", {
                            let mut nbt = NbtCompound::new();
                            nbt.insert("sound", NbtElement::String(sound.to_string()));
                            if let Some(range) = range {
                                nbt.insert("range", NbtElement::Float(*range));
                            }
                            NbtElement::Compound(nbt)
                        });
                    },
                };
            }
            if let Some(mood_sound) = &self.effects.mood_sound {
                nbt.insert("mood_sound", {
                    let mut nbt = NbtCompound::new();
                    nbt.insert("sound"               , NbtElement::String (mood_sound.sound.to_string() ));
                    nbt.insert("tick_delay"          , NbtElement::Int    (mood_sound.tick_delay as i32 ));
                    nbt.insert("block_search_extent" , NbtElement::Int    (mood_sound.block_search_extent as i32 ));
                    nbt.insert("offset"              , NbtElement::Double (mood_sound.offset            ));
                    NbtElement::Compound(nbt)
                });
            }
            if let Some(additions_sound) = &self.effects.additions_sound {
                nbt.insert("additions_sound", {
                    let mut nbt = NbtCompound::new();
                    nbt.insert("sound"       , NbtElement::String (additions_sound.sound.to_string() ));
                    nbt.insert("tick_chance" , NbtElement::Double (additions_sound.tick_chance       ));
                    NbtElement::Compound(nbt)
                });
            }
            if let Some(music) = &self.effects.music {
                nbt.insert("music", {
                    let mut arr = vec![];
                    for song in music {
                        let mut nbt = NbtCompound::new();
                        nbt.insert("data", {
                            let mut nbt = NbtCompound::new();
                            nbt.insert("sound"                 , NbtElement::String (song.data.sound.to_string()             ));
                            nbt.insert("min_delay"             , NbtElement::Int    (song.data.min_delay as i32              ));
                            nbt.insert("max_delay"             , NbtElement::Int    (song.data.max_delay as i32              ));
                            nbt.insert("replace_current_music" , NbtElement::Byte   (if (song.data.replace_current_music) { 1 } else { 0 } ));
                            NbtElement::Compound(nbt)
                        });
                        nbt.insert("weight", NbtElement::Int(song.weight as i32));
                        arr.push(NbtElement::Compound(nbt));
                    }
                    NbtElement::List(arr)
                });
            }
            NbtElement::Compound(nbt)
        });
        Some(Nbt { name : String::new(), root : nbt })
    }

}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum BiomeTempModifier {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "frozen")]
    Frozen
}
impl BiomeTempModifier { fn as_str(&self) -> &'static str { match (self) {
    BiomeTempModifier::None   => "none",
    BiomeTempModifier::Frozen => "frozen"
} } }


#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeEffects {
    pub fog_color            : Colour,
    pub water_color          : Colour,
    pub water_fog_color      : Colour,
    pub sky_color            : Colour,
    pub foliage_color        : Option<Colour>,
    pub grass_color          : Option<Colour>,
    pub grass_color_modifier : Option<BiomeColourModifier>,
    pub particle              : Option<BiomeParticle>,
    pub ambient_sound         : Option<BiomeAmbientSound>,
    pub mood_sound            : Option<BiomeMoodSound>,
    pub additions_sound       : Option<BiomeAdditionsSound>,
    pub music                 : Option<Vec<BiomeMusicWeights>>
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[derive(Serialize, Deserialize)]
pub enum BiomeColourModifier {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "dark_forest")]
    DarkForest,
    #[serde(rename = "swamp")]
    Swamp
}
impl BiomeColourModifier { fn as_str(&self) -> &'static str { match (self) {
    Self::None       => "none",
    Self::DarkForest => "dark_forest",
    Self::Swamp      => "swamp"
} } }

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeParticle {
    pub options     : Particle,
    pub probability : f32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BiomeAmbientSound {
    #[serde(untagged)]
    Id(Identifier),
    #[serde(untagged)]
    Ranged { sound : Identifier, range : Option<f32>}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeMoodSound {
    pub sound               : Identifier,
    pub tick_delay          : u32,
    pub block_search_extent : u32,
    pub offset              : f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeAdditionsSound {
    pub sound       : Identifier,
    pub tick_chance : f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeMusicWeights {
    pub data: BiomeMusic,
    pub weight: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiomeMusic {
    pub sound     : Identifier,
    pub min_delay : u32,
    pub max_delay : u32,
    pub replace_current_music   : bool
}
