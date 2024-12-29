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
        let c = &self.fog_color;
        let d = &self.foliage_color.convert_option();
        let e = &self.grass_color.convert_option();
        let f = self.grass_color_modifier.convert_option();
        let g = &self.mood_sound.convert_option();
        let music = self.music.as_ref().map(|x| x.convert_vec()).convert_option();
        let i = &self.particle.convert_option();
        let h = &self.sky_color;
        let j = &self.water_color;
        let k = &self.water_fog_color;
        tokens.extend(quote! {
            BiomeEffects {
                additions_sound: #a,
                ambient_sound: #b,
                fog_color: #c,
                foliage_color: #d,
                grass_color: #e,
                grass_color_modifier: #f,
                mood_sound: #g,
                music: #music,
                particle: #i,
                sky_color: #h,
                water_color: #j,
                water_fog_color: #k
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
        match self {
            BiomeAmbientSound::Id(identifier) => {
                tokens.extend(quote! {
                    BiomeAmbientSound::Id(#identifier)
                });
            },
            BiomeAmbientSound::Ranged { sound, range } => {
                let range = range.convert_option();
                tokens.extend(quote! {
                    BiomeAmbientSound::Ranged {
                        sound: #sound,
                        range: #range
                    }
                });
            },
        }
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

impl ToTokens for BiomeMusicWeights {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let weight = self.weight;
        let data = &self.data;
        tokens.extend(quote! {
            BiomeMusicWeights {
                data: #data,
                weight: #weight
            }
        });
    }
}

impl ToTokens for BiomeMusic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sound = &self.sound;
        let a = self.max_delay;
        let b = self.min_delay;
        let c = self.replace_current_music;
        tokens.extend(quote! {
            BiomeMusic {
                sound: #sound,
                max_delay: #a,
                min_delay: #b,
                replace_current_music: #c
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