use crate::packet::*;
impl ParticleType {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<ParticleType> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::vanilla_const("angry_villager"),
                ParticleType {
                    id: Identifier::vanilla_const("angry_villager"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block"),
                ParticleType {
                    id: Identifier::vanilla_const("block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block_marker"),
                ParticleType {
                    id: Identifier::vanilla_const("block_marker"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bubble"),
                ParticleType {
                    id: Identifier::vanilla_const("bubble"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cloud"),
                ParticleType {
                    id: Identifier::vanilla_const("cloud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crit"),
                ParticleType {
                    id: Identifier::vanilla_const("crit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("damage_indicator"),
                ParticleType {
                    id: Identifier::vanilla_const("damage_indicator"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dragon_breath"),
                ParticleType {
                    id: Identifier::vanilla_const("dragon_breath"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripping_lava"),
                ParticleType {
                    id: Identifier::vanilla_const("dripping_lava"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_lava"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_lava"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("landing_lava"),
                ParticleType {
                    id: Identifier::vanilla_const("landing_lava"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripping_water"),
                ParticleType {
                    id: Identifier::vanilla_const("dripping_water"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_water"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_water"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dust"),
                ParticleType {
                    id: Identifier::vanilla_const("dust"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dust_color_transition"),
                ParticleType {
                    id: Identifier::vanilla_const("dust_color_transition"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("effect"),
                ParticleType {
                    id: Identifier::vanilla_const("effect"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("elder_guardian"),
                ParticleType {
                    id: Identifier::vanilla_const("elder_guardian"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enchanted_hit"),
                ParticleType {
                    id: Identifier::vanilla_const("enchanted_hit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enchant"),
                ParticleType {
                    id: Identifier::vanilla_const("enchant"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_rod"),
                ParticleType {
                    id: Identifier::vanilla_const("end_rod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity_effect"),
                ParticleType {
                    id: Identifier::vanilla_const("entity_effect"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("explosion_emitter"),
                ParticleType {
                    id: Identifier::vanilla_const("explosion_emitter"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("explosion"),
                ParticleType {
                    id: Identifier::vanilla_const("explosion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gust"),
                ParticleType {
                    id: Identifier::vanilla_const("gust"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("small_gust"),
                ParticleType {
                    id: Identifier::vanilla_const("small_gust"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gust_emitter_large"),
                ParticleType {
                    id: Identifier::vanilla_const("gust_emitter_large"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gust_emitter_small"),
                ParticleType {
                    id: Identifier::vanilla_const("gust_emitter_small"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sonic_boom"),
                ParticleType {
                    id: Identifier::vanilla_const("sonic_boom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_dust"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_dust"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("firework"),
                ParticleType {
                    id: Identifier::vanilla_const("firework"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fishing"),
                ParticleType {
                    id: Identifier::vanilla_const("fishing"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flame"),
                ParticleType {
                    id: Identifier::vanilla_const("flame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested"),
                ParticleType {
                    id: Identifier::vanilla_const("infested"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_leaves"),
                ParticleType {
                    id: Identifier::vanilla_const("cherry_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_leaves"),
                ParticleType {
                    id: Identifier::vanilla_const("pale_oak_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_soul"),
                ParticleType {
                    id: Identifier::vanilla_const("sculk_soul"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_charge"),
                ParticleType {
                    id: Identifier::vanilla_const("sculk_charge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_charge_pop"),
                ParticleType {
                    id: Identifier::vanilla_const("sculk_charge_pop"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul_fire_flame"),
                ParticleType {
                    id: Identifier::vanilla_const("soul_fire_flame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul"),
                ParticleType {
                    id: Identifier::vanilla_const("soul"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flash"),
                ParticleType {
                    id: Identifier::vanilla_const("flash"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("happy_villager"),
                ParticleType {
                    id: Identifier::vanilla_const("happy_villager"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("composter"),
                ParticleType {
                    id: Identifier::vanilla_const("composter"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("heart"),
                ParticleType {
                    id: Identifier::vanilla_const("heart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("instant_effect"),
                ParticleType {
                    id: Identifier::vanilla_const("instant_effect"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item"),
                ParticleType {
                    id: Identifier::vanilla_const("item"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vibration"),
                ParticleType {
                    id: Identifier::vanilla_const("vibration"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trail"),
                ParticleType {
                    id: Identifier::vanilla_const("trail"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item_slime"),
                ParticleType {
                    id: Identifier::vanilla_const("item_slime"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item_cobweb"),
                ParticleType {
                    id: Identifier::vanilla_const("item_cobweb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item_snowball"),
                ParticleType {
                    id: Identifier::vanilla_const("item_snowball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("large_smoke"),
                ParticleType {
                    id: Identifier::vanilla_const("large_smoke"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lava"),
                ParticleType {
                    id: Identifier::vanilla_const("lava"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mycelium"),
                ParticleType {
                    id: Identifier::vanilla_const("mycelium"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("note"),
                ParticleType {
                    id: Identifier::vanilla_const("note"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("poof"),
                ParticleType {
                    id: Identifier::vanilla_const("poof"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("portal"),
                ParticleType {
                    id: Identifier::vanilla_const("portal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rain"),
                ParticleType {
                    id: Identifier::vanilla_const("rain"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smoke"),
                ParticleType {
                    id: Identifier::vanilla_const("smoke"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_smoke"),
                ParticleType {
                    id: Identifier::vanilla_const("white_smoke"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sneeze"),
                ParticleType {
                    id: Identifier::vanilla_const("sneeze"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spit"),
                ParticleType {
                    id: Identifier::vanilla_const("spit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("squid_ink"),
                ParticleType {
                    id: Identifier::vanilla_const("squid_ink"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sweep_attack"),
                ParticleType {
                    id: Identifier::vanilla_const("sweep_attack"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("totem_of_undying"),
                ParticleType {
                    id: Identifier::vanilla_const("totem_of_undying"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("underwater"),
                ParticleType {
                    id: Identifier::vanilla_const("underwater"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("splash"),
                ParticleType {
                    id: Identifier::vanilla_const("splash"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("witch"),
                ParticleType {
                    id: Identifier::vanilla_const("witch"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bubble_pop"),
                ParticleType {
                    id: Identifier::vanilla_const("bubble_pop"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("current_down"),
                ParticleType {
                    id: Identifier::vanilla_const("current_down"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bubble_column_up"),
                ParticleType {
                    id: Identifier::vanilla_const("bubble_column_up"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nautilus"),
                ParticleType {
                    id: Identifier::vanilla_const("nautilus"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dolphin"),
                ParticleType {
                    id: Identifier::vanilla_const("dolphin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("campfire_cosy_smoke"),
                ParticleType {
                    id: Identifier::vanilla_const("campfire_cosy_smoke"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("campfire_signal_smoke"),
                ParticleType {
                    id: Identifier::vanilla_const("campfire_signal_smoke"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripping_honey"),
                ParticleType {
                    id: Identifier::vanilla_const("dripping_honey"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_honey"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_honey"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("landing_honey"),
                ParticleType {
                    id: Identifier::vanilla_const("landing_honey"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_nectar"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_nectar"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_spore_blossom"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_spore_blossom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ash"),
                ParticleType {
                    id: Identifier::vanilla_const("ash"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_spore"),
                ParticleType {
                    id: Identifier::vanilla_const("crimson_spore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_spore"),
                ParticleType {
                    id: Identifier::vanilla_const("warped_spore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spore_blossom_air"),
                ParticleType {
                    id: Identifier::vanilla_const("spore_blossom_air"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripping_obsidian_tear"),
                ParticleType {
                    id: Identifier::vanilla_const("dripping_obsidian_tear"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_obsidian_tear"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_obsidian_tear"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("landing_obsidian_tear"),
                ParticleType {
                    id: Identifier::vanilla_const("landing_obsidian_tear"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("reverse_portal"),
                ParticleType {
                    id: Identifier::vanilla_const("reverse_portal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_ash"),
                ParticleType {
                    id: Identifier::vanilla_const("white_ash"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("small_flame"),
                ParticleType {
                    id: Identifier::vanilla_const("small_flame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snowflake"),
                ParticleType {
                    id: Identifier::vanilla_const("snowflake"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripping_dripstone_lava"),
                ParticleType {
                    id: Identifier::vanilla_const("dripping_dripstone_lava"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_dripstone_lava"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_dripstone_lava"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripping_dripstone_water"),
                ParticleType {
                    id: Identifier::vanilla_const("dripping_dripstone_water"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_dripstone_water"),
                ParticleType {
                    id: Identifier::vanilla_const("falling_dripstone_water"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_squid_ink"),
                ParticleType {
                    id: Identifier::vanilla_const("glow_squid_ink"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow"),
                ParticleType {
                    id: Identifier::vanilla_const("glow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wax_on"),
                ParticleType {
                    id: Identifier::vanilla_const("wax_on"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wax_off"),
                ParticleType {
                    id: Identifier::vanilla_const("wax_off"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("electric_spark"),
                ParticleType {
                    id: Identifier::vanilla_const("electric_spark"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("scrape"),
                ParticleType {
                    id: Identifier::vanilla_const("scrape"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shriek"),
                ParticleType {
                    id: Identifier::vanilla_const("shriek"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("egg_crack"),
                ParticleType {
                    id: Identifier::vanilla_const("egg_crack"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dust_plume"),
                ParticleType {
                    id: Identifier::vanilla_const("dust_plume"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trial_spawner_detection"),
                ParticleType {
                    id: Identifier::vanilla_const("trial_spawner_detection"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trial_spawner_detection_ominous"),
                ParticleType {
                    id: Identifier::vanilla_const("trial_spawner_detection_ominous"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vault_connection"),
                ParticleType {
                    id: Identifier::vanilla_const("vault_connection"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dust_pillar"),
                ParticleType {
                    id: Identifier::vanilla_const("dust_pillar"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ominous_spawning"),
                ParticleType {
                    id: Identifier::vanilla_const("ominous_spawning"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raid_omen"),
                ParticleType {
                    id: Identifier::vanilla_const("raid_omen"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trial_omen"),
                ParticleType {
                    id: Identifier::vanilla_const("trial_omen"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block_crumble"),
                ParticleType {
                    id: Identifier::vanilla_const("block_crumble"),
                },
            );
        registry
    }
}
