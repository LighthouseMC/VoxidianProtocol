use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::value::*;

use super::{ConvertOption, ConvertVec};

impl ToTokens for Biome {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let downfall = self.downfall;
        let effects = &self.effects;
        let has_precipitation = self.has_precipitation;
        let temperature = self.temperature;
        let temperature_modifier = self.temperature_modifier.convert_option();

        tokens.extend(quote! {
            Biome {
                downfall: #downfall,
                effects: #effects,
                has_precipitation: #has_precipitation,
                temperature: #temperature,
                temperature_modifier: #temperature_modifier
            }
        });
    }
}

impl ToTokens for BiomeTempModifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BiomeTempModifier::None => quote! { BiomeTempModifier::None },
            BiomeTempModifier::Frozen => quote! { BiomeTempModifier::Frozen },
        });
    }
}

impl ToTokens for BiomeEffects {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = &self.additions_sound.convert_option();
        let b = &self.ambient_sound.convert_option();
        let c = &self.fog_colour;
        let d = &self.foliage_colour;
        let e = &self.grass_colour;
        let f = self.grass_colour_modifier.convert_option();
        let g = &self.mood_sound.convert_option();
        let music = self.music.convert_option();
        let i = &self.particle.convert_option();
        let h = &self.sky_colour;
        let j = &self.water_colour;
        let k = &self.water_fog_colour;
        tokens.extend(quote! {
            BiomeEffects {
                additions_sound: #a,
                ambient_sound: #b,
                fog_colour: #c,
                foliage_colour: #d,
                grass_colour: #e,
                grass_colour_modifier: #f,
                mood_sound: #g,
                music: #music,
                particle: #i,
                sky_colour: #h,
                water_colour: #j,
                water_fog_colour: #k
            }
        });
    }
}



impl ToTokens for BiomeAdditionsSound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sound = &self.sound;
        let tick_chance = self.tick_chance;
        tokens.extend(quote! {
            BiomeAdditionsSound {
                sound: #sound,
                tick_chance: #tick_chance
            }
        });
    }
}

impl ToTokens for BiomeAmbientSound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let range = self.range;
        let sound = &self.sound;
        tokens.extend(quote! {
            BiomeAmbientSound {
                range: #range,
                sound: #sound
            }
        });
    }
}

impl ToTokens for BiomeMoodSound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let block_search_extent = self.block_search_extent;
        let offset = self.offset;
        let sound = &self.sound;
        let tick_delay = self.tick_delay;
        tokens.extend(quote! {
            BiomeMoodSound {
                block_search_extent: #block_search_extent,
                offset: #offset,
                sound: #sound,
                tick_delay: #tick_delay
            }
        });
    }
}

impl ToTokens for BiomeMusic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sound = &self.sound;
        tokens.extend(quote! {
            BiomeMusic {
                sound: #sound
            }
        });
    }
}

impl ToTokens for BiomeParticle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let options = &self.options;
        let probability = self.probability;
        tokens.extend(quote! {
            BiomeParticle {
                options: #options,
                probability: #probability
            }
        });
    }
}

impl ToTokens for BiomeColourModifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BiomeColourModifier::None => quote! { BiomeColourModifier::None },
            BiomeColourModifier::DarkForest => quote! { BiomeColourModifier::DarkForest },
            BiomeColourModifier::Swamp => quote! { BiomeColourModifier::Swamp },
        });
    }
}