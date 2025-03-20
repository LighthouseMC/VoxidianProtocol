use crate::packet::*;
impl EntityType {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<EntityType> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::vanilla_const("acacia_boat"),
                EntityType {
                    id: Identifier::vanilla_const("acacia_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("acacia_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("allay"),
                EntityType {
                    id: Identifier::vanilla_const("allay"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("area_effect_cloud"),
                EntityType {
                    id: Identifier::vanilla_const("area_effect_cloud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("armadillo"),
                EntityType {
                    id: Identifier::vanilla_const("armadillo"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("armor_stand"),
                EntityType {
                    id: Identifier::vanilla_const("armor_stand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("arrow"),
                EntityType {
                    id: Identifier::vanilla_const("arrow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("axolotl"),
                EntityType {
                    id: Identifier::vanilla_const("axolotl"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_chest_raft"),
                EntityType {
                    id: Identifier::vanilla_const("bamboo_chest_raft"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_raft"),
                EntityType {
                    id: Identifier::vanilla_const("bamboo_raft"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bat"),
                EntityType {
                    id: Identifier::vanilla_const("bat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bee"),
                EntityType {
                    id: Identifier::vanilla_const("bee"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_boat"),
                EntityType {
                    id: Identifier::vanilla_const("birch_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("birch_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blaze"),
                EntityType {
                    id: Identifier::vanilla_const("blaze"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block_display"),
                EntityType {
                    id: Identifier::vanilla_const("block_display"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bogged"),
                EntityType {
                    id: Identifier::vanilla_const("bogged"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("breeze"),
                EntityType {
                    id: Identifier::vanilla_const("breeze"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("breeze_wind_charge"),
                EntityType {
                    id: Identifier::vanilla_const("breeze_wind_charge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("camel"),
                EntityType {
                    id: Identifier::vanilla_const("camel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cat"),
                EntityType {
                    id: Identifier::vanilla_const("cat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cave_spider"),
                EntityType {
                    id: Identifier::vanilla_const("cave_spider"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_boat"),
                EntityType {
                    id: Identifier::vanilla_const("cherry_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("cherry_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chest_minecart"),
                EntityType {
                    id: Identifier::vanilla_const("chest_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chicken"),
                EntityType {
                    id: Identifier::vanilla_const("chicken"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cod"),
                EntityType {
                    id: Identifier::vanilla_const("cod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("command_block_minecart"),
                EntityType {
                    id: Identifier::vanilla_const("command_block_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cow"),
                EntityType {
                    id: Identifier::vanilla_const("cow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creaking"),
                EntityType {
                    id: Identifier::vanilla_const("creaking"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creeper"),
                EntityType {
                    id: Identifier::vanilla_const("creeper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_boat"),
                EntityType {
                    id: Identifier::vanilla_const("dark_oak_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("dark_oak_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dolphin"),
                EntityType {
                    id: Identifier::vanilla_const("dolphin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("donkey"),
                EntityType {
                    id: Identifier::vanilla_const("donkey"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dragon_fireball"),
                EntityType {
                    id: Identifier::vanilla_const("dragon_fireball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("drowned"),
                EntityType {
                    id: Identifier::vanilla_const("drowned"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("egg"),
                EntityType {
                    id: Identifier::vanilla_const("egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("elder_guardian"),
                EntityType {
                    id: Identifier::vanilla_const("elder_guardian"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enderman"),
                EntityType {
                    id: Identifier::vanilla_const("enderman"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("endermite"),
                EntityType {
                    id: Identifier::vanilla_const("endermite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ender_dragon"),
                EntityType {
                    id: Identifier::vanilla_const("ender_dragon"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ender_pearl"),
                EntityType {
                    id: Identifier::vanilla_const("ender_pearl"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_crystal"),
                EntityType {
                    id: Identifier::vanilla_const("end_crystal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("evoker"),
                EntityType {
                    id: Identifier::vanilla_const("evoker"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("evoker_fangs"),
                EntityType {
                    id: Identifier::vanilla_const("evoker_fangs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("experience_bottle"),
                EntityType {
                    id: Identifier::vanilla_const("experience_bottle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("experience_orb"),
                EntityType {
                    id: Identifier::vanilla_const("experience_orb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("eye_of_ender"),
                EntityType {
                    id: Identifier::vanilla_const("eye_of_ender"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("falling_block"),
                EntityType {
                    id: Identifier::vanilla_const("falling_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fireball"),
                EntityType {
                    id: Identifier::vanilla_const("fireball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("firework_rocket"),
                EntityType {
                    id: Identifier::vanilla_const("firework_rocket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fox"),
                EntityType {
                    id: Identifier::vanilla_const("fox"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("frog"),
                EntityType {
                    id: Identifier::vanilla_const("frog"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("furnace_minecart"),
                EntityType {
                    id: Identifier::vanilla_const("furnace_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ghast"),
                EntityType {
                    id: Identifier::vanilla_const("ghast"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("giant"),
                EntityType {
                    id: Identifier::vanilla_const("giant"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_item_frame"),
                EntityType {
                    id: Identifier::vanilla_const("glow_item_frame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_squid"),
                EntityType {
                    id: Identifier::vanilla_const("glow_squid"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("goat"),
                EntityType {
                    id: Identifier::vanilla_const("goat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("guardian"),
                EntityType {
                    id: Identifier::vanilla_const("guardian"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hoglin"),
                EntityType {
                    id: Identifier::vanilla_const("hoglin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hopper_minecart"),
                EntityType {
                    id: Identifier::vanilla_const("hopper_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("horse"),
                EntityType {
                    id: Identifier::vanilla_const("horse"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("husk"),
                EntityType {
                    id: Identifier::vanilla_const("husk"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("illusioner"),
                EntityType {
                    id: Identifier::vanilla_const("illusioner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("interaction"),
                EntityType {
                    id: Identifier::vanilla_const("interaction"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_golem"),
                EntityType {
                    id: Identifier::vanilla_const("iron_golem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item"),
                EntityType {
                    id: Identifier::vanilla_const("item"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item_display"),
                EntityType {
                    id: Identifier::vanilla_const("item_display"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item_frame"),
                EntityType {
                    id: Identifier::vanilla_const("item_frame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_boat"),
                EntityType {
                    id: Identifier::vanilla_const("jungle_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("jungle_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leash_knot"),
                EntityType {
                    id: Identifier::vanilla_const("leash_knot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lightning_bolt"),
                EntityType {
                    id: Identifier::vanilla_const("lightning_bolt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("llama"),
                EntityType {
                    id: Identifier::vanilla_const("llama"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("llama_spit"),
                EntityType {
                    id: Identifier::vanilla_const("llama_spit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magma_cube"),
                EntityType {
                    id: Identifier::vanilla_const("magma_cube"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_boat"),
                EntityType {
                    id: Identifier::vanilla_const("mangrove_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("mangrove_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("marker"),
                EntityType {
                    id: Identifier::vanilla_const("marker"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("minecart"),
                EntityType {
                    id: Identifier::vanilla_const("minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mooshroom"),
                EntityType {
                    id: Identifier::vanilla_const("mooshroom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mule"),
                EntityType {
                    id: Identifier::vanilla_const("mule"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_boat"),
                EntityType {
                    id: Identifier::vanilla_const("oak_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("oak_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ocelot"),
                EntityType {
                    id: Identifier::vanilla_const("ocelot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ominous_item_spawner"),
                EntityType {
                    id: Identifier::vanilla_const("ominous_item_spawner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("painting"),
                EntityType {
                    id: Identifier::vanilla_const("painting"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_boat"),
                EntityType {
                    id: Identifier::vanilla_const("pale_oak_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("pale_oak_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("panda"),
                EntityType {
                    id: Identifier::vanilla_const("panda"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("parrot"),
                EntityType {
                    id: Identifier::vanilla_const("parrot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("phantom"),
                EntityType {
                    id: Identifier::vanilla_const("phantom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pig"),
                EntityType {
                    id: Identifier::vanilla_const("pig"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piglin"),
                EntityType {
                    id: Identifier::vanilla_const("piglin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piglin_brute"),
                EntityType {
                    id: Identifier::vanilla_const("piglin_brute"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pillager"),
                EntityType {
                    id: Identifier::vanilla_const("pillager"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polar_bear"),
                EntityType {
                    id: Identifier::vanilla_const("polar_bear"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("splash_potion"),
                EntityType {
                    id: Identifier::vanilla_const("splash_potion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lingering_potion"),
                EntityType {
                    id: Identifier::vanilla_const("lingering_potion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pufferfish"),
                EntityType {
                    id: Identifier::vanilla_const("pufferfish"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rabbit"),
                EntityType {
                    id: Identifier::vanilla_const("rabbit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ravager"),
                EntityType {
                    id: Identifier::vanilla_const("ravager"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("salmon"),
                EntityType {
                    id: Identifier::vanilla_const("salmon"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sheep"),
                EntityType {
                    id: Identifier::vanilla_const("sheep"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shulker"),
                EntityType {
                    id: Identifier::vanilla_const("shulker"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shulker_bullet"),
                EntityType {
                    id: Identifier::vanilla_const("shulker_bullet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("silverfish"),
                EntityType {
                    id: Identifier::vanilla_const("silverfish"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skeleton"),
                EntityType {
                    id: Identifier::vanilla_const("skeleton"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skeleton_horse"),
                EntityType {
                    id: Identifier::vanilla_const("skeleton_horse"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("slime"),
                EntityType {
                    id: Identifier::vanilla_const("slime"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("small_fireball"),
                EntityType {
                    id: Identifier::vanilla_const("small_fireball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sniffer"),
                EntityType {
                    id: Identifier::vanilla_const("sniffer"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snowball"),
                EntityType {
                    id: Identifier::vanilla_const("snowball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snow_golem"),
                EntityType {
                    id: Identifier::vanilla_const("snow_golem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spawner_minecart"),
                EntityType {
                    id: Identifier::vanilla_const("spawner_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spectral_arrow"),
                EntityType {
                    id: Identifier::vanilla_const("spectral_arrow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spider"),
                EntityType {
                    id: Identifier::vanilla_const("spider"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_boat"),
                EntityType {
                    id: Identifier::vanilla_const("spruce_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_chest_boat"),
                EntityType {
                    id: Identifier::vanilla_const("spruce_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("squid"),
                EntityType {
                    id: Identifier::vanilla_const("squid"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stray"),
                EntityType {
                    id: Identifier::vanilla_const("stray"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("strider"),
                EntityType {
                    id: Identifier::vanilla_const("strider"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tadpole"),
                EntityType {
                    id: Identifier::vanilla_const("tadpole"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("text_display"),
                EntityType {
                    id: Identifier::vanilla_const("text_display"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tnt"),
                EntityType {
                    id: Identifier::vanilla_const("tnt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tnt_minecart"),
                EntityType {
                    id: Identifier::vanilla_const("tnt_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trader_llama"),
                EntityType {
                    id: Identifier::vanilla_const("trader_llama"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trident"),
                EntityType {
                    id: Identifier::vanilla_const("trident"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tropical_fish"),
                EntityType {
                    id: Identifier::vanilla_const("tropical_fish"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("turtle"),
                EntityType {
                    id: Identifier::vanilla_const("turtle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vex"),
                EntityType {
                    id: Identifier::vanilla_const("vex"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("villager"),
                EntityType {
                    id: Identifier::vanilla_const("villager"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vindicator"),
                EntityType {
                    id: Identifier::vanilla_const("vindicator"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wandering_trader"),
                EntityType {
                    id: Identifier::vanilla_const("wandering_trader"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warden"),
                EntityType {
                    id: Identifier::vanilla_const("warden"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wind_charge"),
                EntityType {
                    id: Identifier::vanilla_const("wind_charge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("witch"),
                EntityType {
                    id: Identifier::vanilla_const("witch"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither"),
                EntityType {
                    id: Identifier::vanilla_const("wither"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither_skeleton"),
                EntityType {
                    id: Identifier::vanilla_const("wither_skeleton"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither_skull"),
                EntityType {
                    id: Identifier::vanilla_const("wither_skull"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wolf"),
                EntityType {
                    id: Identifier::vanilla_const("wolf"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zoglin"),
                EntityType {
                    id: Identifier::vanilla_const("zoglin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie"),
                EntityType {
                    id: Identifier::vanilla_const("zombie"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie_horse"),
                EntityType {
                    id: Identifier::vanilla_const("zombie_horse"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie_villager"),
                EntityType {
                    id: Identifier::vanilla_const("zombie_villager"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombified_piglin"),
                EntityType {
                    id: Identifier::vanilla_const("zombified_piglin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("player"),
                EntityType {
                    id: Identifier::vanilla_const("player"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fishing_bobber"),
                EntityType {
                    id: Identifier::vanilla_const("fishing_bobber"),
                },
            );
        registry
    }
}
