use super::*;


pub struct Biome {
    pub has_rain      : bool,
    pub temp          : f32,
    pub temp_modifier : Option<BiomeTempModifier>,
    pub downfall      : f32,
    pub effects       : BiomeEffects
}
impl RegValue for Biome {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("worldgen/biome");

    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("has_precipitation" , NbtElement::Byte  (if (self.has_rain) { 1 } else { 0 }));
        nbt.insert("temperature"       , NbtElement::Float (self.temp));
        if let Some(temp_modifier) = self.temp_modifier {
            nbt.insert("temperature_modifier", NbtElement::String(temp_modifier.as_str().to_string()));
        }
        nbt.insert("downfall", NbtElement::Float(self.downfall));
        nbt.insert("effects", {
            let mut nbt = NbtCompound::new();
            nbt.insert("fog_color"       , NbtElement::Int(self.effects.fog_colour       .to_int()));
            nbt.insert("water_color"     , NbtElement::Int(self.effects.water_colour     .to_int()));
            nbt.insert("water_fog_color" , NbtElement::Int(self.effects.water_fog_colour .to_int()));
            nbt.insert("sky_color"       , NbtElement::Int(self.effects.sky_colour       .to_int()));
            nbt.insert("foliage_color"   , NbtElement::Int(self.effects.foliage_colour   .to_int()));
            nbt.insert("grass_color"     , NbtElement::Int(self.effects.grass_colour     .to_int()));
            if let Some(grass_colour_modifier) = &self.effects.grass_colour_modifier {
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
                nbt.insert("ambient_sound", {
                    let mut nbt = NbtCompound::new();
                    nbt.insert("sound_id", NbtElement::String(ambient_sound.sound.to_string()));
                    if let Some(range) = ambient_sound.range {
                        nbt.insert("range", NbtElement::Float(range));
                    }
                    NbtElement::Compound(nbt)
                });
            }
            if let Some(mood_sound) = &self.effects.mood_sound {
                nbt.insert("mood_sound", {
                    let mut nbt = NbtCompound::new();
                    nbt.insert("sound"               , NbtElement::String (mood_sound.sound.to_string() ));
                    nbt.insert("tick_delay"          , NbtElement::Int    (mood_sound.tick_delay as i32 ));
                    nbt.insert("block_search_extent" , NbtElement::Int    (mood_sound.tick_delay as i32 ));
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
                    let mut nbt = NbtCompound::new();
                    nbt.insert("sound"                 , NbtElement::String (music.sound.to_string()             ));
                    nbt.insert("min_delay"             , NbtElement::Int    (music.min_delay as i32              ));
                    nbt.insert("max_delay"             , NbtElement::Int    (music.max_delay as i32              ));
                    nbt.insert("replace_current_music" , NbtElement::Byte   (if (music.replace) { 1 } else { 0 } ));
                    NbtElement::Compound(nbt)
                });
            }
            NbtElement::Compound(nbt)
        });
        Some(Nbt { name : String::new(), root : nbt })
    }

}


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BiomeTempModifier {
    None,
    Frozen
}
impl BiomeTempModifier { fn as_str(&self) -> &'static str { match (self) {
    BiomeTempModifier::None   => "none",
    BiomeTempModifier::Frozen => "frozen"
} } }


pub struct BiomeEffects {
    pub fog_colour            : Colour,
    pub water_colour          : Colour,
    pub water_fog_colour      : Colour,
    pub sky_colour            : Colour,
    pub foliage_colour        : Colour,
    pub grass_colour          : Colour,
    pub grass_colour_modifier : Option<BiomeColourModifier>,
    pub particle              : Option<BiomeParticle>,
    pub ambient_sound         : Option<BiomeAmbientSound>,
    pub mood_sound            : Option<BiomeMoodSound>,
    pub additions_sound       : Option<BiomeAdditionsSound>,
    pub music                 : Option<BiomeMusic>
}


#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BiomeColourModifier {
    None,
    DarkForest,
    Swamp
}
impl BiomeColourModifier { fn as_str(&self) -> &'static str { match (self) {
    Self::None       => "none",
    Self::DarkForest => "dark_forest",
    Self::Swamp      => "swamp"
} } }


pub struct BiomeParticle {
    pub options     : Particle,
    pub probability : f32
}


pub struct BiomeAmbientSound {
    pub sound : Identifier,
    pub range : Option<f32>
}


pub struct BiomeMoodSound {
    pub sound               : Identifier,
    pub tick_delay          : u32,
    pub block_search_extent : u32,
    pub offset              : f64
}


pub struct BiomeAdditionsSound {
    pub sound       : Identifier,
    pub tick_chance : f64
}


pub struct BiomeMusic {
    pub sound     : Identifier,
    pub min_delay : u32,
    pub max_delay : u32,
    pub replace   : bool
}
