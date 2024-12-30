use crate::packet::*;
impl Biome {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<Biome> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::new("minecraft", "desert"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.desert"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "end_highlands"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(10518688i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(0i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "deep_frozen_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(3750089i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: Some(BiomeTempModifier::Frozen),
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "soul_sand_valley"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: Some(BiomeAdditionsSound {
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.soul_sand_valley.additions",
                            ),
                            tick_chance: 0.0111f64,
                        }),
                        ambient_sound: Some(
                            BiomeAmbientSound::Id(
                                Identifier::new(
                                    "minecraft",
                                    "ambient.soul_sand_valley.loop",
                                ),
                            ),
                        ),
                        fog_color: Colour::new_from_raw_int(1787717i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.soul_sand_valley.mood",
                            ),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.nether.soul_sand_valley"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: Some(BiomeParticle {
                            options: Particle {
                                id: Identifier::new("minecraft", "ash"),
                            },
                            probability: 0.00625f32,
                        }),
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "savanna"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "forest"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.forest"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7972607i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.7f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "grove"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.grove"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8495359i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: -0.2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "old_growth_birch_forest"),
                Biome {
                    downfall: 0.6f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.forest"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8037887i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.6f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "jagged_peaks"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.jagged_peaks"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8756735i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: -0.7f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "taiga"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8233983i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.25f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "deep_cold_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4020182i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "river"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "savanna_plateau"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "warm_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4445678i32),
                        water_fog_color: Colour::new_from_raw_int(270131i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "sparse_jungle"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.sparse_jungle"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7842047i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.95f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "windswept_hills"),
                Biome {
                    downfall: 0.3f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8233727i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "deep_dark"),
                Biome {
                    downfall: 0.4f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.deep_dark"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "dark_forest"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: Some(BiomeColourModifier::DarkForest),
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.forest"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7972607i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.7f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "end_midlands"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(10518688i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(0i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "birch_forest"),
                Biome {
                    downfall: 0.6f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.forest"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8037887i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.6f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "plains"),
                Biome {
                    downfall: 0.4f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "dripstone_caves"),
                Biome {
                    downfall: 0.4f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.dripstone_caves"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "sunflower_plains"),
                Biome {
                    downfall: 0.4f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "deep_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "stony_shore"),
                Biome {
                    downfall: 0.3f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8233727i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "badlands"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: Some(Colour::new_from_raw_int(10387789i32)),
                        grass_color: Some(Colour::new_from_raw_int(9470285i32)),
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.badlands"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "wooded_badlands"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: Some(Colour::new_from_raw_int(10387789i32)),
                        grass_color: Some(Colour::new_from_raw_int(9470285i32)),
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.badlands"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "end_barrens"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(10518688i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(0i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "meadow"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.meadow"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(937679i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "eroded_badlands"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: Some(Colour::new_from_raw_int(10387789i32)),
                        grass_color: Some(Colour::new_from_raw_int(9470285i32)),
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.badlands"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "swamp"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: Some(Colour::new_from_raw_int(6975545i32)),
                        grass_color: None,
                        grass_color_modifier: Some(BiomeColourModifier::Swamp),
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.swamp"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(6388580i32),
                        water_fog_color: Colour::new_from_raw_int(2302743i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "snowy_plains"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8364543i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "snowy_taiga"),
                Biome {
                    downfall: 0.4f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8625919i32),
                        water_color: Colour::new_from_raw_int(4020182i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: -0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "crimson_forest"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: Some(BiomeAdditionsSound {
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.crimson_forest.additions",
                            ),
                            tick_chance: 0.0111f64,
                        }),
                        ambient_sound: Some(
                            BiomeAmbientSound::Id(
                                Identifier::new("minecraft", "ambient.crimson_forest.loop"),
                            ),
                        ),
                        fog_color: Colour::new_from_raw_int(3343107i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.crimson_forest.mood",
                            ),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.nether.crimson_forest"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: Some(BiomeParticle {
                            options: Particle {
                                id: Identifier::new("minecraft", "crimson_spore"),
                            },
                            probability: 0.025f32,
                        }),
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "windswept_savanna"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "stony_peaks"),
                Biome {
                    downfall: 0.3f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.stony_peaks"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7776511i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 1f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "snowy_slopes"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.snowy_slopes"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8560639i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: -0.3f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "frozen_peaks"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.frozen_peaks"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8756735i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: -0.7f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "cherry_grove"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: Some(Colour::new_from_raw_int(11983713i32)),
                        grass_color: Some(Colour::new_from_raw_int(11983713i32)),
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.cherry_grove"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(6141935i32),
                        water_fog_color: Colour::new_from_raw_int(6141935i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "windswept_gravelly_hills"),
                Biome {
                    downfall: 0.3f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8233727i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "deep_lukewarm_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4566514i32),
                        water_fog_color: Colour::new_from_raw_int(267827i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "mangrove_swamp"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: Some(Colour::new_from_raw_int(9285927i32)),
                        grass_color: None,
                        grass_color_modifier: Some(BiomeColourModifier::Swamp),
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.swamp"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(3832426i32),
                        water_fog_color: Colour::new_from_raw_int(5077600i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "lukewarm_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4566514i32),
                        water_fog_color: Colour::new_from_raw_int(267827i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "windswept_forest"),
                Biome {
                    downfall: 0.3f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8233727i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "flower_forest"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.flower_forest"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7972607i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.7f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "cold_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4020182i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "snowy_beach"),
                Biome {
                    downfall: 0.3f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8364543i32),
                        water_color: Colour::new_from_raw_int(4020182i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.05f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "small_end_islands"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(10518688i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(0i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "the_end"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(10518688i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(0i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "frozen_ocean"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8364543i32),
                        water_color: Colour::new_from_raw_int(3750089i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0f32,
                    temperature_modifier: Some(BiomeTempModifier::Frozen),
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "old_growth_spruce_taiga"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.old_growth_taiga"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8233983i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.25f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "old_growth_pine_taiga"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.old_growth_taiga"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8168447i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.3f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "bamboo_jungle"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft",
                                "music.overworld.bamboo_jungle"), max_delay : 24000u32,
                                min_delay : 12000u32, replace_current_music : false },
                                weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7842047i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.95f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "lush_caves"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.lush_caves"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "mushroom_fields"),
                Biome {
                    downfall: 1f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7842047i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.9f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "pale_garden"),
                Biome {
                    downfall: 0.8f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(8484720i32),
                        foliage_color: Some(Colour::new_from_raw_int(8883574i32)),
                        grass_color: Some(Colour::new_from_raw_int(7832178i32)),
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(vec![]),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(12171705i32),
                        water_color: Colour::new_from_raw_int(7768221i32),
                        water_fog_color: Colour::new_from_raw_int(5597568i32),
                    },
                    has_precipitation: true,
                    temperature: 0.7f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "warped_forest"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: Some(BiomeAdditionsSound {
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.warped_forest.additions",
                            ),
                            tick_chance: 0.0111f64,
                        }),
                        ambient_sound: Some(
                            BiomeAmbientSound::Id(
                                Identifier::new("minecraft", "ambient.warped_forest.loop"),
                            ),
                        ),
                        fog_color: Colour::new_from_raw_int(1705242i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.warped_forest.mood",
                            ),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.nether.warped_forest"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: Some(BiomeParticle {
                            options: Particle {
                                id: Identifier::new("minecraft", "warped_spore"),
                            },
                            probability: 0.01428f32,
                        }),
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "basalt_deltas"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: Some(BiomeAdditionsSound {
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.basalt_deltas.additions",
                            ),
                            tick_chance: 0.0111f64,
                        }),
                        ambient_sound: Some(
                            BiomeAmbientSound::Id(
                                Identifier::new("minecraft", "ambient.basalt_deltas.loop"),
                            ),
                        ),
                        fog_color: Colour::new_from_raw_int(6840176i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.basalt_deltas.mood",
                            ),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.nether.basalt_deltas"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: Some(BiomeParticle {
                            options: Particle {
                                id: Identifier::new("minecraft", "white_ash"),
                            },
                            probability: 0.118093334f32,
                        }),
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "ice_spikes"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8364543i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "jungle"),
                Biome {
                    downfall: 0.9f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.overworld.jungle"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7842047i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.95f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "frozen_river"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8364543i32),
                        water_color: Colour::new_from_raw_int(3750089i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "nether_wastes"),
                Biome {
                    downfall: 0f32,
                    effects: BiomeEffects {
                        additions_sound: Some(BiomeAdditionsSound {
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.nether_wastes.additions",
                            ),
                            tick_chance: 0.0111f64,
                        }),
                        ambient_sound: Some(
                            BiomeAmbientSound::Id(
                                Identifier::new("minecraft", "ambient.nether_wastes.loop"),
                            ),
                        ),
                        fog_color: Colour::new_from_raw_int(3344392i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new(
                                "minecraft",
                                "ambient.nether_wastes.mood",
                            ),
                            tick_delay: 6000u32,
                        }),
                        music: Some(
                            vec![
                                BiomeMusicWeights { data : BiomeMusic { sound :
                                Identifier::new("minecraft", "music.nether.nether_wastes"),
                                max_delay : 24000u32, min_delay : 12000u32,
                                replace_current_music : false }, weight : 1u32 },
                            ],
                        ),
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7254527i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 2f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "the_void"),
                Biome {
                    downfall: 0.5f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(8103167i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: false,
                    temperature: 0.5f32,
                    temperature_modifier: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "beach"),
                Biome {
                    downfall: 0.4f32,
                    effects: BiomeEffects {
                        additions_sound: None,
                        ambient_sound: None,
                        fog_color: Colour::new_from_raw_int(12638463i32),
                        foliage_color: None,
                        grass_color: None,
                        grass_color_modifier: None,
                        mood_sound: Some(BiomeMoodSound {
                            block_search_extent: 8u32,
                            offset: 2f64,
                            sound: Identifier::new("minecraft", "ambient.cave"),
                            tick_delay: 6000u32,
                        }),
                        music: None,
                        particle: None,
                        sky_color: Colour::new_from_raw_int(7907327i32),
                        water_color: Colour::new_from_raw_int(4159204i32),
                        water_fog_color: Colour::new_from_raw_int(329011i32),
                    },
                    has_precipitation: true,
                    temperature: 0.8f32,
                    temperature_modifier: None,
                },
            );
        registry
    }
}
