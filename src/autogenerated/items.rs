use crate::packet::*;
impl Item {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<Item> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::vanilla_const("air"),
                Item {
                    id: Identifier::vanilla_const("air"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone"),
                Item {
                    id: Identifier::vanilla_const("stone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("granite"),
                Item {
                    id: Identifier::vanilla_const("granite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_granite"),
                Item {
                    id: Identifier::vanilla_const("polished_granite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diorite"),
                Item {
                    id: Identifier::vanilla_const("diorite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_diorite"),
                Item {
                    id: Identifier::vanilla_const("polished_diorite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("andesite"),
                Item {
                    id: Identifier::vanilla_const("andesite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_andesite"),
                Item {
                    id: Identifier::vanilla_const("polished_andesite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate"),
                Item {
                    id: Identifier::vanilla_const("deepslate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobbled_deepslate"),
                Item {
                    id: Identifier::vanilla_const("cobbled_deepslate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_deepslate"),
                Item {
                    id: Identifier::vanilla_const("polished_deepslate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("calcite"),
                Item {
                    id: Identifier::vanilla_const("calcite"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff"),
                Item {
                    id: Identifier::vanilla_const("tuff"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_slab"),
                Item {
                    id: Identifier::vanilla_const("tuff_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_stairs"),
                Item {
                    id: Identifier::vanilla_const("tuff_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_wall"),
                Item {
                    id: Identifier::vanilla_const("tuff_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_tuff"),
                Item {
                    id: Identifier::vanilla_const("chiseled_tuff"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_tuff"),
                Item {
                    id: Identifier::vanilla_const("polished_tuff"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_tuff_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_tuff_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_tuff_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_tuff_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_tuff_wall"),
                Item {
                    id: Identifier::vanilla_const("polished_tuff_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_bricks"),
                Item {
                    id: Identifier::vanilla_const("tuff_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("tuff_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("tuff_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tuff_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("tuff_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_tuff_bricks"),
                Item {
                    id: Identifier::vanilla_const("chiseled_tuff_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dripstone_block"),
                Item {
                    id: Identifier::vanilla_const("dripstone_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("grass_block"),
                Item {
                    id: Identifier::vanilla_const("grass_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dirt"),
                Item {
                    id: Identifier::vanilla_const("dirt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("coarse_dirt"),
                Item {
                    id: Identifier::vanilla_const("coarse_dirt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("podzol"),
                Item {
                    id: Identifier::vanilla_const("podzol"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rooted_dirt"),
                Item {
                    id: Identifier::vanilla_const("rooted_dirt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mud"),
                Item {
                    id: Identifier::vanilla_const("mud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_nylium"),
                Item {
                    id: Identifier::vanilla_const("crimson_nylium"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_nylium"),
                Item {
                    id: Identifier::vanilla_const("warped_nylium"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobblestone"),
                Item {
                    id: Identifier::vanilla_const("cobblestone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_planks"),
                Item {
                    id: Identifier::vanilla_const("oak_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_planks"),
                Item {
                    id: Identifier::vanilla_const("spruce_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_planks"),
                Item {
                    id: Identifier::vanilla_const("birch_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_planks"),
                Item {
                    id: Identifier::vanilla_const("jungle_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_planks"),
                Item {
                    id: Identifier::vanilla_const("acacia_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_planks"),
                Item {
                    id: Identifier::vanilla_const("cherry_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_planks"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_planks"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_planks"),
                Item {
                    id: Identifier::vanilla_const("mangrove_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_planks"),
                Item {
                    id: Identifier::vanilla_const("bamboo_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_planks"),
                Item {
                    id: Identifier::vanilla_const("crimson_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_planks"),
                Item {
                    id: Identifier::vanilla_const("warped_planks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_mosaic"),
                Item {
                    id: Identifier::vanilla_const("bamboo_mosaic"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_sapling"),
                Item {
                    id: Identifier::vanilla_const("oak_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_sapling"),
                Item {
                    id: Identifier::vanilla_const("spruce_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_sapling"),
                Item {
                    id: Identifier::vanilla_const("birch_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_sapling"),
                Item {
                    id: Identifier::vanilla_const("jungle_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_sapling"),
                Item {
                    id: Identifier::vanilla_const("acacia_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_sapling"),
                Item {
                    id: Identifier::vanilla_const("cherry_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_sapling"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_sapling"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_sapling"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_propagule"),
                Item {
                    id: Identifier::vanilla_const("mangrove_propagule"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bedrock"),
                Item {
                    id: Identifier::vanilla_const("bedrock"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sand"),
                Item {
                    id: Identifier::vanilla_const("sand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("suspicious_sand"),
                Item {
                    id: Identifier::vanilla_const("suspicious_sand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("suspicious_gravel"),
                Item {
                    id: Identifier::vanilla_const("suspicious_gravel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_sand"),
                Item {
                    id: Identifier::vanilla_const("red_sand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gravel"),
                Item {
                    id: Identifier::vanilla_const("gravel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("coal_ore"),
                Item {
                    id: Identifier::vanilla_const("coal_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_coal_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_coal_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_ore"),
                Item {
                    id: Identifier::vanilla_const("iron_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_iron_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_iron_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_ore"),
                Item {
                    id: Identifier::vanilla_const("copper_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_copper_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_copper_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gold_ore"),
                Item {
                    id: Identifier::vanilla_const("gold_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_gold_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_gold_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("redstone_ore"),
                Item {
                    id: Identifier::vanilla_const("redstone_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_redstone_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_redstone_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("emerald_ore"),
                Item {
                    id: Identifier::vanilla_const("emerald_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_emerald_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_emerald_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lapis_ore"),
                Item {
                    id: Identifier::vanilla_const("lapis_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_lapis_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_lapis_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_ore"),
                Item {
                    id: Identifier::vanilla_const("diamond_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_diamond_ore"),
                Item {
                    id: Identifier::vanilla_const("deepslate_diamond_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_gold_ore"),
                Item {
                    id: Identifier::vanilla_const("nether_gold_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_quartz_ore"),
                Item {
                    id: Identifier::vanilla_const("nether_quartz_ore"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ancient_debris"),
                Item {
                    id: Identifier::vanilla_const("ancient_debris"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("coal_block"),
                Item {
                    id: Identifier::vanilla_const("coal_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raw_iron_block"),
                Item {
                    id: Identifier::vanilla_const("raw_iron_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raw_copper_block"),
                Item {
                    id: Identifier::vanilla_const("raw_copper_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raw_gold_block"),
                Item {
                    id: Identifier::vanilla_const("raw_gold_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("heavy_core"),
                Item {
                    id: Identifier::vanilla_const("heavy_core"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("amethyst_block"),
                Item {
                    id: Identifier::vanilla_const("amethyst_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("budding_amethyst"),
                Item {
                    id: Identifier::vanilla_const("budding_amethyst"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_block"),
                Item {
                    id: Identifier::vanilla_const("iron_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_block"),
                Item {
                    id: Identifier::vanilla_const("copper_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gold_block"),
                Item {
                    id: Identifier::vanilla_const("gold_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_block"),
                Item {
                    id: Identifier::vanilla_const("diamond_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_block"),
                Item {
                    id: Identifier::vanilla_const("netherite_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_copper"),
                Item {
                    id: Identifier::vanilla_const("exposed_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_copper"),
                Item {
                    id: Identifier::vanilla_const("weathered_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_copper"),
                Item {
                    id: Identifier::vanilla_const("oxidized_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("exposed_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("weathered_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("oxidized_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_copper"),
                Item {
                    id: Identifier::vanilla_const("cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("exposed_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("weathered_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("oxidized_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("exposed_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("weathered_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("oxidized_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("exposed_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("weathered_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("oxidized_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_copper_block"),
                Item {
                    id: Identifier::vanilla_const("waxed_copper_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_chiseled_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_chiseled_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_cut_copper"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_cut_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("waxed_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_cut_copper_stairs"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_cut_copper_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("waxed_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_cut_copper_slab"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_cut_copper_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_log"),
                Item {
                    id: Identifier::vanilla_const("oak_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_log"),
                Item {
                    id: Identifier::vanilla_const("spruce_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_log"),
                Item {
                    id: Identifier::vanilla_const("birch_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_log"),
                Item {
                    id: Identifier::vanilla_const("jungle_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_log"),
                Item {
                    id: Identifier::vanilla_const("acacia_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_log"),
                Item {
                    id: Identifier::vanilla_const("cherry_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_log"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_log"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_log"),
                Item {
                    id: Identifier::vanilla_const("mangrove_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_roots"),
                Item {
                    id: Identifier::vanilla_const("mangrove_roots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("muddy_mangrove_roots"),
                Item {
                    id: Identifier::vanilla_const("muddy_mangrove_roots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_stem"),
                Item {
                    id: Identifier::vanilla_const("crimson_stem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_stem"),
                Item {
                    id: Identifier::vanilla_const("warped_stem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_block"),
                Item {
                    id: Identifier::vanilla_const("bamboo_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_oak_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_oak_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_spruce_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_spruce_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_birch_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_birch_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_jungle_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_jungle_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_acacia_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_acacia_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_cherry_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_cherry_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_dark_oak_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_dark_oak_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_pale_oak_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_pale_oak_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_mangrove_log"),
                Item {
                    id: Identifier::vanilla_const("stripped_mangrove_log"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_crimson_stem"),
                Item {
                    id: Identifier::vanilla_const("stripped_crimson_stem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_warped_stem"),
                Item {
                    id: Identifier::vanilla_const("stripped_warped_stem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_oak_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_oak_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_spruce_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_spruce_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_birch_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_birch_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_jungle_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_jungle_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_acacia_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_acacia_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_cherry_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_cherry_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_dark_oak_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_dark_oak_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_pale_oak_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_pale_oak_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_mangrove_wood"),
                Item {
                    id: Identifier::vanilla_const("stripped_mangrove_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_crimson_hyphae"),
                Item {
                    id: Identifier::vanilla_const("stripped_crimson_hyphae"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_warped_hyphae"),
                Item {
                    id: Identifier::vanilla_const("stripped_warped_hyphae"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stripped_bamboo_block"),
                Item {
                    id: Identifier::vanilla_const("stripped_bamboo_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_wood"),
                Item {
                    id: Identifier::vanilla_const("oak_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_wood"),
                Item {
                    id: Identifier::vanilla_const("spruce_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_wood"),
                Item {
                    id: Identifier::vanilla_const("birch_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_wood"),
                Item {
                    id: Identifier::vanilla_const("jungle_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_wood"),
                Item {
                    id: Identifier::vanilla_const("acacia_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_wood"),
                Item {
                    id: Identifier::vanilla_const("cherry_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_wood"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_wood"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_wood"),
                Item {
                    id: Identifier::vanilla_const("mangrove_wood"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_hyphae"),
                Item {
                    id: Identifier::vanilla_const("crimson_hyphae"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_hyphae"),
                Item {
                    id: Identifier::vanilla_const("warped_hyphae"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_leaves"),
                Item {
                    id: Identifier::vanilla_const("oak_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_leaves"),
                Item {
                    id: Identifier::vanilla_const("spruce_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_leaves"),
                Item {
                    id: Identifier::vanilla_const("birch_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_leaves"),
                Item {
                    id: Identifier::vanilla_const("jungle_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_leaves"),
                Item {
                    id: Identifier::vanilla_const("acacia_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_leaves"),
                Item {
                    id: Identifier::vanilla_const("cherry_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_leaves"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_leaves"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_leaves"),
                Item {
                    id: Identifier::vanilla_const("mangrove_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("azalea_leaves"),
                Item {
                    id: Identifier::vanilla_const("azalea_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flowering_azalea_leaves"),
                Item {
                    id: Identifier::vanilla_const("flowering_azalea_leaves"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sponge"),
                Item {
                    id: Identifier::vanilla_const("sponge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wet_sponge"),
                Item {
                    id: Identifier::vanilla_const("wet_sponge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glass"),
                Item {
                    id: Identifier::vanilla_const("glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tinted_glass"),
                Item {
                    id: Identifier::vanilla_const("tinted_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lapis_block"),
                Item {
                    id: Identifier::vanilla_const("lapis_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sandstone"),
                Item {
                    id: Identifier::vanilla_const("sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_sandstone"),
                Item {
                    id: Identifier::vanilla_const("chiseled_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_sandstone"),
                Item {
                    id: Identifier::vanilla_const("cut_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobweb"),
                Item {
                    id: Identifier::vanilla_const("cobweb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("short_grass"),
                Item {
                    id: Identifier::vanilla_const("short_grass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fern"),
                Item {
                    id: Identifier::vanilla_const("fern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bush"),
                Item {
                    id: Identifier::vanilla_const("bush"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("azalea"),
                Item {
                    id: Identifier::vanilla_const("azalea"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flowering_azalea"),
                Item {
                    id: Identifier::vanilla_const("flowering_azalea"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_bush"),
                Item {
                    id: Identifier::vanilla_const("dead_bush"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("firefly_bush"),
                Item {
                    id: Identifier::vanilla_const("firefly_bush"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("short_dry_grass"),
                Item {
                    id: Identifier::vanilla_const("short_dry_grass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tall_dry_grass"),
                Item {
                    id: Identifier::vanilla_const("tall_dry_grass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("seagrass"),
                Item {
                    id: Identifier::vanilla_const("seagrass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sea_pickle"),
                Item {
                    id: Identifier::vanilla_const("sea_pickle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_wool"),
                Item {
                    id: Identifier::vanilla_const("white_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_wool"),
                Item {
                    id: Identifier::vanilla_const("orange_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_wool"),
                Item {
                    id: Identifier::vanilla_const("magenta_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_wool"),
                Item {
                    id: Identifier::vanilla_const("light_blue_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_wool"),
                Item {
                    id: Identifier::vanilla_const("yellow_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_wool"),
                Item {
                    id: Identifier::vanilla_const("lime_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_wool"),
                Item {
                    id: Identifier::vanilla_const("pink_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_wool"),
                Item {
                    id: Identifier::vanilla_const("gray_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_wool"),
                Item {
                    id: Identifier::vanilla_const("light_gray_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_wool"),
                Item {
                    id: Identifier::vanilla_const("cyan_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_wool"),
                Item {
                    id: Identifier::vanilla_const("purple_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_wool"),
                Item {
                    id: Identifier::vanilla_const("blue_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_wool"),
                Item {
                    id: Identifier::vanilla_const("brown_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_wool"),
                Item {
                    id: Identifier::vanilla_const("green_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_wool"),
                Item {
                    id: Identifier::vanilla_const("red_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_wool"),
                Item {
                    id: Identifier::vanilla_const("black_wool"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dandelion"),
                Item {
                    id: Identifier::vanilla_const("dandelion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("open_eyeblossom"),
                Item {
                    id: Identifier::vanilla_const("open_eyeblossom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("closed_eyeblossom"),
                Item {
                    id: Identifier::vanilla_const("closed_eyeblossom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("poppy"),
                Item {
                    id: Identifier::vanilla_const("poppy"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_orchid"),
                Item {
                    id: Identifier::vanilla_const("blue_orchid"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("allium"),
                Item {
                    id: Identifier::vanilla_const("allium"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("azure_bluet"),
                Item {
                    id: Identifier::vanilla_const("azure_bluet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_tulip"),
                Item {
                    id: Identifier::vanilla_const("red_tulip"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_tulip"),
                Item {
                    id: Identifier::vanilla_const("orange_tulip"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_tulip"),
                Item {
                    id: Identifier::vanilla_const("white_tulip"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_tulip"),
                Item {
                    id: Identifier::vanilla_const("pink_tulip"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxeye_daisy"),
                Item {
                    id: Identifier::vanilla_const("oxeye_daisy"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cornflower"),
                Item {
                    id: Identifier::vanilla_const("cornflower"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lily_of_the_valley"),
                Item {
                    id: Identifier::vanilla_const("lily_of_the_valley"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither_rose"),
                Item {
                    id: Identifier::vanilla_const("wither_rose"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("torchflower"),
                Item {
                    id: Identifier::vanilla_const("torchflower"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pitcher_plant"),
                Item {
                    id: Identifier::vanilla_const("pitcher_plant"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spore_blossom"),
                Item {
                    id: Identifier::vanilla_const("spore_blossom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_mushroom"),
                Item {
                    id: Identifier::vanilla_const("brown_mushroom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_mushroom"),
                Item {
                    id: Identifier::vanilla_const("red_mushroom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_fungus"),
                Item {
                    id: Identifier::vanilla_const("crimson_fungus"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_fungus"),
                Item {
                    id: Identifier::vanilla_const("warped_fungus"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_roots"),
                Item {
                    id: Identifier::vanilla_const("crimson_roots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_roots"),
                Item {
                    id: Identifier::vanilla_const("warped_roots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_sprouts"),
                Item {
                    id: Identifier::vanilla_const("nether_sprouts"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weeping_vines"),
                Item {
                    id: Identifier::vanilla_const("weeping_vines"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("twisting_vines"),
                Item {
                    id: Identifier::vanilla_const("twisting_vines"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sugar_cane"),
                Item {
                    id: Identifier::vanilla_const("sugar_cane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("kelp"),
                Item {
                    id: Identifier::vanilla_const("kelp"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_petals"),
                Item {
                    id: Identifier::vanilla_const("pink_petals"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wildflowers"),
                Item {
                    id: Identifier::vanilla_const("wildflowers"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leaf_litter"),
                Item {
                    id: Identifier::vanilla_const("leaf_litter"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("moss_carpet"),
                Item {
                    id: Identifier::vanilla_const("moss_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("moss_block"),
                Item {
                    id: Identifier::vanilla_const("moss_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_moss_carpet"),
                Item {
                    id: Identifier::vanilla_const("pale_moss_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_hanging_moss"),
                Item {
                    id: Identifier::vanilla_const("pale_hanging_moss"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_moss_block"),
                Item {
                    id: Identifier::vanilla_const("pale_moss_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hanging_roots"),
                Item {
                    id: Identifier::vanilla_const("hanging_roots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("big_dripleaf"),
                Item {
                    id: Identifier::vanilla_const("big_dripleaf"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("small_dripleaf"),
                Item {
                    id: Identifier::vanilla_const("small_dripleaf"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo"),
                Item {
                    id: Identifier::vanilla_const("bamboo"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_slab"),
                Item {
                    id: Identifier::vanilla_const("oak_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_slab"),
                Item {
                    id: Identifier::vanilla_const("spruce_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_slab"),
                Item {
                    id: Identifier::vanilla_const("birch_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_slab"),
                Item {
                    id: Identifier::vanilla_const("jungle_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_slab"),
                Item {
                    id: Identifier::vanilla_const("acacia_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_slab"),
                Item {
                    id: Identifier::vanilla_const("cherry_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_slab"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_slab"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_slab"),
                Item {
                    id: Identifier::vanilla_const("mangrove_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_slab"),
                Item {
                    id: Identifier::vanilla_const("bamboo_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_mosaic_slab"),
                Item {
                    id: Identifier::vanilla_const("bamboo_mosaic_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_slab"),
                Item {
                    id: Identifier::vanilla_const("crimson_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_slab"),
                Item {
                    id: Identifier::vanilla_const("warped_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_slab"),
                Item {
                    id: Identifier::vanilla_const("stone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_stone_slab"),
                Item {
                    id: Identifier::vanilla_const("smooth_stone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sandstone_slab"),
                Item {
                    id: Identifier::vanilla_const("sandstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_sandstone_slab"),
                Item {
                    id: Identifier::vanilla_const("cut_sandstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("petrified_oak_slab"),
                Item {
                    id: Identifier::vanilla_const("petrified_oak_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobblestone_slab"),
                Item {
                    id: Identifier::vanilla_const("cobblestone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brick_slab"),
                Item {
                    id: Identifier::vanilla_const("brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("stone_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mud_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("mud_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("nether_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("quartz_slab"),
                Item {
                    id: Identifier::vanilla_const("quartz_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_sandstone_slab"),
                Item {
                    id: Identifier::vanilla_const("red_sandstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_red_sandstone_slab"),
                Item {
                    id: Identifier::vanilla_const("cut_red_sandstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purpur_slab"),
                Item {
                    id: Identifier::vanilla_const("purpur_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_slab"),
                Item {
                    id: Identifier::vanilla_const("prismarine_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("prismarine_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_prismarine_slab"),
                Item {
                    id: Identifier::vanilla_const("dark_prismarine_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_quartz"),
                Item {
                    id: Identifier::vanilla_const("smooth_quartz"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_red_sandstone"),
                Item {
                    id: Identifier::vanilla_const("smooth_red_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_sandstone"),
                Item {
                    id: Identifier::vanilla_const("smooth_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_stone"),
                Item {
                    id: Identifier::vanilla_const("smooth_stone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bricks"),
                Item {
                    id: Identifier::vanilla_const("bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bookshelf"),
                Item {
                    id: Identifier::vanilla_const("bookshelf"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_bookshelf"),
                Item {
                    id: Identifier::vanilla_const("chiseled_bookshelf"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("decorated_pot"),
                Item {
                    id: Identifier::vanilla_const("decorated_pot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_cobblestone"),
                Item {
                    id: Identifier::vanilla_const("mossy_cobblestone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("obsidian"),
                Item {
                    id: Identifier::vanilla_const("obsidian"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("torch"),
                Item {
                    id: Identifier::vanilla_const("torch"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_rod"),
                Item {
                    id: Identifier::vanilla_const("end_rod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chorus_plant"),
                Item {
                    id: Identifier::vanilla_const("chorus_plant"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chorus_flower"),
                Item {
                    id: Identifier::vanilla_const("chorus_flower"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purpur_block"),
                Item {
                    id: Identifier::vanilla_const("purpur_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purpur_pillar"),
                Item {
                    id: Identifier::vanilla_const("purpur_pillar"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purpur_stairs"),
                Item {
                    id: Identifier::vanilla_const("purpur_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spawner"),
                Item {
                    id: Identifier::vanilla_const("spawner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creaking_heart"),
                Item {
                    id: Identifier::vanilla_const("creaking_heart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chest"),
                Item {
                    id: Identifier::vanilla_const("chest"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crafting_table"),
                Item {
                    id: Identifier::vanilla_const("crafting_table"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("farmland"),
                Item {
                    id: Identifier::vanilla_const("farmland"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("furnace"),
                Item {
                    id: Identifier::vanilla_const("furnace"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ladder"),
                Item {
                    id: Identifier::vanilla_const("ladder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobblestone_stairs"),
                Item {
                    id: Identifier::vanilla_const("cobblestone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snow"),
                Item {
                    id: Identifier::vanilla_const("snow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ice"),
                Item {
                    id: Identifier::vanilla_const("ice"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snow_block"),
                Item {
                    id: Identifier::vanilla_const("snow_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cactus"),
                Item {
                    id: Identifier::vanilla_const("cactus"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cactus_flower"),
                Item {
                    id: Identifier::vanilla_const("cactus_flower"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("clay"),
                Item {
                    id: Identifier::vanilla_const("clay"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jukebox"),
                Item {
                    id: Identifier::vanilla_const("jukebox"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_fence"),
                Item {
                    id: Identifier::vanilla_const("oak_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_fence"),
                Item {
                    id: Identifier::vanilla_const("spruce_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_fence"),
                Item {
                    id: Identifier::vanilla_const("birch_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_fence"),
                Item {
                    id: Identifier::vanilla_const("jungle_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_fence"),
                Item {
                    id: Identifier::vanilla_const("acacia_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_fence"),
                Item {
                    id: Identifier::vanilla_const("cherry_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_fence"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_fence"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_fence"),
                Item {
                    id: Identifier::vanilla_const("mangrove_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_fence"),
                Item {
                    id: Identifier::vanilla_const("bamboo_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_fence"),
                Item {
                    id: Identifier::vanilla_const("crimson_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_fence"),
                Item {
                    id: Identifier::vanilla_const("warped_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pumpkin"),
                Item {
                    id: Identifier::vanilla_const("pumpkin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("carved_pumpkin"),
                Item {
                    id: Identifier::vanilla_const("carved_pumpkin"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jack_o_lantern"),
                Item {
                    id: Identifier::vanilla_const("jack_o_lantern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherrack"),
                Item {
                    id: Identifier::vanilla_const("netherrack"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul_sand"),
                Item {
                    id: Identifier::vanilla_const("soul_sand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul_soil"),
                Item {
                    id: Identifier::vanilla_const("soul_soil"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("basalt"),
                Item {
                    id: Identifier::vanilla_const("basalt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_basalt"),
                Item {
                    id: Identifier::vanilla_const("polished_basalt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_basalt"),
                Item {
                    id: Identifier::vanilla_const("smooth_basalt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul_torch"),
                Item {
                    id: Identifier::vanilla_const("soul_torch"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glowstone"),
                Item {
                    id: Identifier::vanilla_const("glowstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_stone"),
                Item {
                    id: Identifier::vanilla_const("infested_stone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_cobblestone"),
                Item {
                    id: Identifier::vanilla_const("infested_cobblestone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("infested_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_mossy_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("infested_mossy_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_cracked_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("infested_cracked_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_chiseled_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("infested_chiseled_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("infested_deepslate"),
                Item {
                    id: Identifier::vanilla_const("infested_deepslate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("mossy_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cracked_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("cracked_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("chiseled_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("packed_mud"),
                Item {
                    id: Identifier::vanilla_const("packed_mud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mud_bricks"),
                Item {
                    id: Identifier::vanilla_const("mud_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_bricks"),
                Item {
                    id: Identifier::vanilla_const("deepslate_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cracked_deepslate_bricks"),
                Item {
                    id: Identifier::vanilla_const("cracked_deepslate_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_tiles"),
                Item {
                    id: Identifier::vanilla_const("deepslate_tiles"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cracked_deepslate_tiles"),
                Item {
                    id: Identifier::vanilla_const("cracked_deepslate_tiles"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_deepslate"),
                Item {
                    id: Identifier::vanilla_const("chiseled_deepslate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("reinforced_deepslate"),
                Item {
                    id: Identifier::vanilla_const("reinforced_deepslate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_mushroom_block"),
                Item {
                    id: Identifier::vanilla_const("brown_mushroom_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_mushroom_block"),
                Item {
                    id: Identifier::vanilla_const("red_mushroom_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mushroom_stem"),
                Item {
                    id: Identifier::vanilla_const("mushroom_stem"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_bars"),
                Item {
                    id: Identifier::vanilla_const("iron_bars"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chain"),
                Item {
                    id: Identifier::vanilla_const("chain"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glass_pane"),
                Item {
                    id: Identifier::vanilla_const("glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("melon"),
                Item {
                    id: Identifier::vanilla_const("melon"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vine"),
                Item {
                    id: Identifier::vanilla_const("vine"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_lichen"),
                Item {
                    id: Identifier::vanilla_const("glow_lichen"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_clump"),
                Item {
                    id: Identifier::vanilla_const("resin_clump"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_block"),
                Item {
                    id: Identifier::vanilla_const("resin_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_bricks"),
                Item {
                    id: Identifier::vanilla_const("resin_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("resin_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("resin_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("resin_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_resin_bricks"),
                Item {
                    id: Identifier::vanilla_const("chiseled_resin_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("stone_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mud_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("mud_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mycelium"),
                Item {
                    id: Identifier::vanilla_const("mycelium"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lily_pad"),
                Item {
                    id: Identifier::vanilla_const("lily_pad"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_bricks"),
                Item {
                    id: Identifier::vanilla_const("nether_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cracked_nether_bricks"),
                Item {
                    id: Identifier::vanilla_const("cracked_nether_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_nether_bricks"),
                Item {
                    id: Identifier::vanilla_const("chiseled_nether_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_brick_fence"),
                Item {
                    id: Identifier::vanilla_const("nether_brick_fence"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("nether_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk"),
                Item {
                    id: Identifier::vanilla_const("sculk"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_vein"),
                Item {
                    id: Identifier::vanilla_const("sculk_vein"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_catalyst"),
                Item {
                    id: Identifier::vanilla_const("sculk_catalyst"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_shrieker"),
                Item {
                    id: Identifier::vanilla_const("sculk_shrieker"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enchanting_table"),
                Item {
                    id: Identifier::vanilla_const("enchanting_table"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_portal_frame"),
                Item {
                    id: Identifier::vanilla_const("end_portal_frame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_stone"),
                Item {
                    id: Identifier::vanilla_const("end_stone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_stone_bricks"),
                Item {
                    id: Identifier::vanilla_const("end_stone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dragon_egg"),
                Item {
                    id: Identifier::vanilla_const("dragon_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sandstone_stairs"),
                Item {
                    id: Identifier::vanilla_const("sandstone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ender_chest"),
                Item {
                    id: Identifier::vanilla_const("ender_chest"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("emerald_block"),
                Item {
                    id: Identifier::vanilla_const("emerald_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_stairs"),
                Item {
                    id: Identifier::vanilla_const("oak_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_stairs"),
                Item {
                    id: Identifier::vanilla_const("spruce_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_stairs"),
                Item {
                    id: Identifier::vanilla_const("birch_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_stairs"),
                Item {
                    id: Identifier::vanilla_const("jungle_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_stairs"),
                Item {
                    id: Identifier::vanilla_const("acacia_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_stairs"),
                Item {
                    id: Identifier::vanilla_const("cherry_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_stairs"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_stairs"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_stairs"),
                Item {
                    id: Identifier::vanilla_const("mangrove_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_stairs"),
                Item {
                    id: Identifier::vanilla_const("bamboo_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_mosaic_stairs"),
                Item {
                    id: Identifier::vanilla_const("bamboo_mosaic_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_stairs"),
                Item {
                    id: Identifier::vanilla_const("crimson_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_stairs"),
                Item {
                    id: Identifier::vanilla_const("warped_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("command_block"),
                Item {
                    id: Identifier::vanilla_const("command_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("beacon"),
                Item {
                    id: Identifier::vanilla_const("beacon"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobblestone_wall"),
                Item {
                    id: Identifier::vanilla_const("cobblestone_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_cobblestone_wall"),
                Item {
                    id: Identifier::vanilla_const("mossy_cobblestone_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brick_wall"),
                Item {
                    id: Identifier::vanilla_const("brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_wall"),
                Item {
                    id: Identifier::vanilla_const("prismarine_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_sandstone_wall"),
                Item {
                    id: Identifier::vanilla_const("red_sandstone_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_stone_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("mossy_stone_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("granite_wall"),
                Item {
                    id: Identifier::vanilla_const("granite_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("stone_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mud_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("mud_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("nether_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("andesite_wall"),
                Item {
                    id: Identifier::vanilla_const("andesite_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_nether_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("red_nether_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sandstone_wall"),
                Item {
                    id: Identifier::vanilla_const("sandstone_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_stone_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("end_stone_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diorite_wall"),
                Item {
                    id: Identifier::vanilla_const("diorite_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blackstone_wall"),
                Item {
                    id: Identifier::vanilla_const("blackstone_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_wall"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobbled_deepslate_wall"),
                Item {
                    id: Identifier::vanilla_const("cobbled_deepslate_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_deepslate_wall"),
                Item {
                    id: Identifier::vanilla_const("polished_deepslate_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_brick_wall"),
                Item {
                    id: Identifier::vanilla_const("deepslate_brick_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_tile_wall"),
                Item {
                    id: Identifier::vanilla_const("deepslate_tile_wall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("anvil"),
                Item {
                    id: Identifier::vanilla_const("anvil"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chipped_anvil"),
                Item {
                    id: Identifier::vanilla_const("chipped_anvil"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("damaged_anvil"),
                Item {
                    id: Identifier::vanilla_const("damaged_anvil"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_quartz_block"),
                Item {
                    id: Identifier::vanilla_const("chiseled_quartz_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("quartz_block"),
                Item {
                    id: Identifier::vanilla_const("quartz_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("quartz_bricks"),
                Item {
                    id: Identifier::vanilla_const("quartz_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("quartz_pillar"),
                Item {
                    id: Identifier::vanilla_const("quartz_pillar"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("quartz_stairs"),
                Item {
                    id: Identifier::vanilla_const("quartz_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_terracotta"),
                Item {
                    id: Identifier::vanilla_const("white_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_terracotta"),
                Item {
                    id: Identifier::vanilla_const("orange_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_terracotta"),
                Item {
                    id: Identifier::vanilla_const("magenta_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_terracotta"),
                Item {
                    id: Identifier::vanilla_const("light_blue_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_terracotta"),
                Item {
                    id: Identifier::vanilla_const("yellow_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_terracotta"),
                Item {
                    id: Identifier::vanilla_const("lime_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_terracotta"),
                Item {
                    id: Identifier::vanilla_const("pink_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_terracotta"),
                Item {
                    id: Identifier::vanilla_const("gray_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_terracotta"),
                Item {
                    id: Identifier::vanilla_const("light_gray_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_terracotta"),
                Item {
                    id: Identifier::vanilla_const("cyan_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_terracotta"),
                Item {
                    id: Identifier::vanilla_const("purple_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_terracotta"),
                Item {
                    id: Identifier::vanilla_const("blue_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_terracotta"),
                Item {
                    id: Identifier::vanilla_const("brown_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_terracotta"),
                Item {
                    id: Identifier::vanilla_const("green_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_terracotta"),
                Item {
                    id: Identifier::vanilla_const("red_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_terracotta"),
                Item {
                    id: Identifier::vanilla_const("black_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("barrier"),
                Item {
                    id: Identifier::vanilla_const("barrier"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light"),
                Item {
                    id: Identifier::vanilla_const("light"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hay_block"),
                Item {
                    id: Identifier::vanilla_const("hay_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_carpet"),
                Item {
                    id: Identifier::vanilla_const("white_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_carpet"),
                Item {
                    id: Identifier::vanilla_const("orange_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_carpet"),
                Item {
                    id: Identifier::vanilla_const("magenta_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_carpet"),
                Item {
                    id: Identifier::vanilla_const("light_blue_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_carpet"),
                Item {
                    id: Identifier::vanilla_const("yellow_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_carpet"),
                Item {
                    id: Identifier::vanilla_const("lime_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_carpet"),
                Item {
                    id: Identifier::vanilla_const("pink_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_carpet"),
                Item {
                    id: Identifier::vanilla_const("gray_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_carpet"),
                Item {
                    id: Identifier::vanilla_const("light_gray_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_carpet"),
                Item {
                    id: Identifier::vanilla_const("cyan_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_carpet"),
                Item {
                    id: Identifier::vanilla_const("purple_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_carpet"),
                Item {
                    id: Identifier::vanilla_const("blue_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_carpet"),
                Item {
                    id: Identifier::vanilla_const("brown_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_carpet"),
                Item {
                    id: Identifier::vanilla_const("green_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_carpet"),
                Item {
                    id: Identifier::vanilla_const("red_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_carpet"),
                Item {
                    id: Identifier::vanilla_const("black_carpet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("terracotta"),
                Item {
                    id: Identifier::vanilla_const("terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("packed_ice"),
                Item {
                    id: Identifier::vanilla_const("packed_ice"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dirt_path"),
                Item {
                    id: Identifier::vanilla_const("dirt_path"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sunflower"),
                Item {
                    id: Identifier::vanilla_const("sunflower"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lilac"),
                Item {
                    id: Identifier::vanilla_const("lilac"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rose_bush"),
                Item {
                    id: Identifier::vanilla_const("rose_bush"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("peony"),
                Item {
                    id: Identifier::vanilla_const("peony"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tall_grass"),
                Item {
                    id: Identifier::vanilla_const("tall_grass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("large_fern"),
                Item {
                    id: Identifier::vanilla_const("large_fern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("white_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("orange_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("magenta_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("light_blue_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("yellow_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("lime_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("pink_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("gray_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("light_gray_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("cyan_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("purple_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("blue_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("brown_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("green_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("red_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_stained_glass"),
                Item {
                    id: Identifier::vanilla_const("black_stained_glass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("white_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("orange_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("magenta_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("light_blue_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("yellow_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("lime_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("pink_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("gray_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("light_gray_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("cyan_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("purple_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("blue_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("brown_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("green_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("red_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_stained_glass_pane"),
                Item {
                    id: Identifier::vanilla_const("black_stained_glass_pane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine"),
                Item {
                    id: Identifier::vanilla_const("prismarine"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_bricks"),
                Item {
                    id: Identifier::vanilla_const("prismarine_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_prismarine"),
                Item {
                    id: Identifier::vanilla_const("dark_prismarine"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_stairs"),
                Item {
                    id: Identifier::vanilla_const("prismarine_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("prismarine_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_prismarine_stairs"),
                Item {
                    id: Identifier::vanilla_const("dark_prismarine_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sea_lantern"),
                Item {
                    id: Identifier::vanilla_const("sea_lantern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_sandstone"),
                Item {
                    id: Identifier::vanilla_const("red_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_red_sandstone"),
                Item {
                    id: Identifier::vanilla_const("chiseled_red_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cut_red_sandstone"),
                Item {
                    id: Identifier::vanilla_const("cut_red_sandstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_sandstone_stairs"),
                Item {
                    id: Identifier::vanilla_const("red_sandstone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("repeating_command_block"),
                Item {
                    id: Identifier::vanilla_const("repeating_command_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chain_command_block"),
                Item {
                    id: Identifier::vanilla_const("chain_command_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magma_block"),
                Item {
                    id: Identifier::vanilla_const("magma_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_wart_block"),
                Item {
                    id: Identifier::vanilla_const("nether_wart_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_wart_block"),
                Item {
                    id: Identifier::vanilla_const("warped_wart_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_nether_bricks"),
                Item {
                    id: Identifier::vanilla_const("red_nether_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bone_block"),
                Item {
                    id: Identifier::vanilla_const("bone_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("structure_void"),
                Item {
                    id: Identifier::vanilla_const("structure_void"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shulker_box"),
                Item {
                    id: Identifier::vanilla_const("shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("white_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("orange_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("magenta_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("light_blue_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("yellow_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("lime_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("pink_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("gray_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("light_gray_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("cyan_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("purple_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("blue_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("brown_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("green_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("red_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_shulker_box"),
                Item {
                    id: Identifier::vanilla_const("black_shulker_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("white_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("orange_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("magenta_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("light_blue_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("yellow_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("lime_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("pink_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("gray_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("light_gray_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("cyan_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("purple_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("blue_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("brown_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("green_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("red_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_glazed_terracotta"),
                Item {
                    id: Identifier::vanilla_const("black_glazed_terracotta"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_concrete"),
                Item {
                    id: Identifier::vanilla_const("white_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_concrete"),
                Item {
                    id: Identifier::vanilla_const("orange_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_concrete"),
                Item {
                    id: Identifier::vanilla_const("magenta_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_concrete"),
                Item {
                    id: Identifier::vanilla_const("light_blue_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_concrete"),
                Item {
                    id: Identifier::vanilla_const("yellow_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_concrete"),
                Item {
                    id: Identifier::vanilla_const("lime_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_concrete"),
                Item {
                    id: Identifier::vanilla_const("pink_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_concrete"),
                Item {
                    id: Identifier::vanilla_const("gray_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_concrete"),
                Item {
                    id: Identifier::vanilla_const("light_gray_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_concrete"),
                Item {
                    id: Identifier::vanilla_const("cyan_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_concrete"),
                Item {
                    id: Identifier::vanilla_const("purple_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_concrete"),
                Item {
                    id: Identifier::vanilla_const("blue_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_concrete"),
                Item {
                    id: Identifier::vanilla_const("brown_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_concrete"),
                Item {
                    id: Identifier::vanilla_const("green_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_concrete"),
                Item {
                    id: Identifier::vanilla_const("red_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_concrete"),
                Item {
                    id: Identifier::vanilla_const("black_concrete"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("white_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("orange_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("magenta_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("light_blue_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("yellow_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("lime_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("pink_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("gray_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("light_gray_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("cyan_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("purple_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("blue_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("brown_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("green_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("red_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_concrete_powder"),
                Item {
                    id: Identifier::vanilla_const("black_concrete_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("turtle_egg"),
                Item {
                    id: Identifier::vanilla_const("turtle_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sniffer_egg"),
                Item {
                    id: Identifier::vanilla_const("sniffer_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_tube_coral_block"),
                Item {
                    id: Identifier::vanilla_const("dead_tube_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_brain_coral_block"),
                Item {
                    id: Identifier::vanilla_const("dead_brain_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_bubble_coral_block"),
                Item {
                    id: Identifier::vanilla_const("dead_bubble_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_fire_coral_block"),
                Item {
                    id: Identifier::vanilla_const("dead_fire_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_horn_coral_block"),
                Item {
                    id: Identifier::vanilla_const("dead_horn_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tube_coral_block"),
                Item {
                    id: Identifier::vanilla_const("tube_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brain_coral_block"),
                Item {
                    id: Identifier::vanilla_const("brain_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bubble_coral_block"),
                Item {
                    id: Identifier::vanilla_const("bubble_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fire_coral_block"),
                Item {
                    id: Identifier::vanilla_const("fire_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("horn_coral_block"),
                Item {
                    id: Identifier::vanilla_const("horn_coral_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tube_coral"),
                Item {
                    id: Identifier::vanilla_const("tube_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brain_coral"),
                Item {
                    id: Identifier::vanilla_const("brain_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bubble_coral"),
                Item {
                    id: Identifier::vanilla_const("bubble_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fire_coral"),
                Item {
                    id: Identifier::vanilla_const("fire_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("horn_coral"),
                Item {
                    id: Identifier::vanilla_const("horn_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_brain_coral"),
                Item {
                    id: Identifier::vanilla_const("dead_brain_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_bubble_coral"),
                Item {
                    id: Identifier::vanilla_const("dead_bubble_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_fire_coral"),
                Item {
                    id: Identifier::vanilla_const("dead_fire_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_horn_coral"),
                Item {
                    id: Identifier::vanilla_const("dead_horn_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_tube_coral"),
                Item {
                    id: Identifier::vanilla_const("dead_tube_coral"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tube_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("tube_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brain_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("brain_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bubble_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("bubble_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fire_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("fire_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("horn_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("horn_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_tube_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("dead_tube_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_brain_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("dead_brain_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_bubble_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("dead_bubble_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_fire_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("dead_fire_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dead_horn_coral_fan"),
                Item {
                    id: Identifier::vanilla_const("dead_horn_coral_fan"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_ice"),
                Item {
                    id: Identifier::vanilla_const("blue_ice"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("conduit"),
                Item {
                    id: Identifier::vanilla_const("conduit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_granite_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_granite_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_red_sandstone_stairs"),
                Item {
                    id: Identifier::vanilla_const("smooth_red_sandstone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_stone_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("mossy_stone_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_diorite_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_diorite_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_cobblestone_stairs"),
                Item {
                    id: Identifier::vanilla_const("mossy_cobblestone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_stone_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("end_stone_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_stairs"),
                Item {
                    id: Identifier::vanilla_const("stone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_sandstone_stairs"),
                Item {
                    id: Identifier::vanilla_const("smooth_sandstone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_quartz_stairs"),
                Item {
                    id: Identifier::vanilla_const("smooth_quartz_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("granite_stairs"),
                Item {
                    id: Identifier::vanilla_const("granite_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("andesite_stairs"),
                Item {
                    id: Identifier::vanilla_const("andesite_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_nether_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("red_nether_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_andesite_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_andesite_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diorite_stairs"),
                Item {
                    id: Identifier::vanilla_const("diorite_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobbled_deepslate_stairs"),
                Item {
                    id: Identifier::vanilla_const("cobbled_deepslate_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_deepslate_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_deepslate_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("deepslate_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_tile_stairs"),
                Item {
                    id: Identifier::vanilla_const("deepslate_tile_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_granite_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_granite_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_red_sandstone_slab"),
                Item {
                    id: Identifier::vanilla_const("smooth_red_sandstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_stone_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("mossy_stone_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_diorite_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_diorite_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mossy_cobblestone_slab"),
                Item {
                    id: Identifier::vanilla_const("mossy_cobblestone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_stone_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("end_stone_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_sandstone_slab"),
                Item {
                    id: Identifier::vanilla_const("smooth_sandstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smooth_quartz_slab"),
                Item {
                    id: Identifier::vanilla_const("smooth_quartz_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("granite_slab"),
                Item {
                    id: Identifier::vanilla_const("granite_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("andesite_slab"),
                Item {
                    id: Identifier::vanilla_const("andesite_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_nether_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("red_nether_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_andesite_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_andesite_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diorite_slab"),
                Item {
                    id: Identifier::vanilla_const("diorite_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cobbled_deepslate_slab"),
                Item {
                    id: Identifier::vanilla_const("cobbled_deepslate_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_deepslate_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_deepslate_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("deepslate_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("deepslate_tile_slab"),
                Item {
                    id: Identifier::vanilla_const("deepslate_tile_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("scaffolding"),
                Item {
                    id: Identifier::vanilla_const("scaffolding"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("redstone"),
                Item {
                    id: Identifier::vanilla_const("redstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("redstone_torch"),
                Item {
                    id: Identifier::vanilla_const("redstone_torch"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("redstone_block"),
                Item {
                    id: Identifier::vanilla_const("redstone_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("repeater"),
                Item {
                    id: Identifier::vanilla_const("repeater"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("comparator"),
                Item {
                    id: Identifier::vanilla_const("comparator"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piston"),
                Item {
                    id: Identifier::vanilla_const("piston"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sticky_piston"),
                Item {
                    id: Identifier::vanilla_const("sticky_piston"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("slime_block"),
                Item {
                    id: Identifier::vanilla_const("slime_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("honey_block"),
                Item {
                    id: Identifier::vanilla_const("honey_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("observer"),
                Item {
                    id: Identifier::vanilla_const("observer"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hopper"),
                Item {
                    id: Identifier::vanilla_const("hopper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dispenser"),
                Item {
                    id: Identifier::vanilla_const("dispenser"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dropper"),
                Item {
                    id: Identifier::vanilla_const("dropper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lectern"),
                Item {
                    id: Identifier::vanilla_const("lectern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("target"),
                Item {
                    id: Identifier::vanilla_const("target"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lever"),
                Item {
                    id: Identifier::vanilla_const("lever"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lightning_rod"),
                Item {
                    id: Identifier::vanilla_const("lightning_rod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("daylight_detector"),
                Item {
                    id: Identifier::vanilla_const("daylight_detector"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sculk_sensor"),
                Item {
                    id: Identifier::vanilla_const("sculk_sensor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("calibrated_sculk_sensor"),
                Item {
                    id: Identifier::vanilla_const("calibrated_sculk_sensor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tripwire_hook"),
                Item {
                    id: Identifier::vanilla_const("tripwire_hook"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trapped_chest"),
                Item {
                    id: Identifier::vanilla_const("trapped_chest"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tnt"),
                Item {
                    id: Identifier::vanilla_const("tnt"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("redstone_lamp"),
                Item {
                    id: Identifier::vanilla_const("redstone_lamp"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("note_block"),
                Item {
                    id: Identifier::vanilla_const("note_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_button"),
                Item {
                    id: Identifier::vanilla_const("stone_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_button"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_button"),
                Item {
                    id: Identifier::vanilla_const("oak_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_button"),
                Item {
                    id: Identifier::vanilla_const("spruce_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_button"),
                Item {
                    id: Identifier::vanilla_const("birch_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_button"),
                Item {
                    id: Identifier::vanilla_const("jungle_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_button"),
                Item {
                    id: Identifier::vanilla_const("acacia_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_button"),
                Item {
                    id: Identifier::vanilla_const("cherry_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_button"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_button"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_button"),
                Item {
                    id: Identifier::vanilla_const("mangrove_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_button"),
                Item {
                    id: Identifier::vanilla_const("bamboo_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_button"),
                Item {
                    id: Identifier::vanilla_const("crimson_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_button"),
                Item {
                    id: Identifier::vanilla_const("warped_button"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("stone_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_weighted_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("light_weighted_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("heavy_weighted_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("heavy_weighted_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("oak_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("spruce_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("birch_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("jungle_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("acacia_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("cherry_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("mangrove_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("bamboo_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("crimson_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_pressure_plate"),
                Item {
                    id: Identifier::vanilla_const("warped_pressure_plate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_door"),
                Item {
                    id: Identifier::vanilla_const("iron_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_door"),
                Item {
                    id: Identifier::vanilla_const("oak_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_door"),
                Item {
                    id: Identifier::vanilla_const("spruce_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_door"),
                Item {
                    id: Identifier::vanilla_const("birch_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_door"),
                Item {
                    id: Identifier::vanilla_const("jungle_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_door"),
                Item {
                    id: Identifier::vanilla_const("acacia_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_door"),
                Item {
                    id: Identifier::vanilla_const("cherry_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_door"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_door"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_door"),
                Item {
                    id: Identifier::vanilla_const("mangrove_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_door"),
                Item {
                    id: Identifier::vanilla_const("bamboo_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_door"),
                Item {
                    id: Identifier::vanilla_const("crimson_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_door"),
                Item {
                    id: Identifier::vanilla_const("warped_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_door"),
                Item {
                    id: Identifier::vanilla_const("copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_copper_door"),
                Item {
                    id: Identifier::vanilla_const("exposed_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_copper_door"),
                Item {
                    id: Identifier::vanilla_const("weathered_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_copper_door"),
                Item {
                    id: Identifier::vanilla_const("oxidized_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_copper_door"),
                Item {
                    id: Identifier::vanilla_const("waxed_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_copper_door"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_copper_door"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_copper_door"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_copper_door"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("iron_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("oak_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("spruce_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("birch_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("jungle_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("acacia_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("cherry_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("mangrove_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("bamboo_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("crimson_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("warped_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("exposed_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("weathered_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("oxidized_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("waxed_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_copper_trapdoor"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_copper_trapdoor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("oak_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("spruce_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("birch_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("jungle_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("acacia_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("cherry_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("mangrove_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("bamboo_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("crimson_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_fence_gate"),
                Item {
                    id: Identifier::vanilla_const("warped_fence_gate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("powered_rail"),
                Item {
                    id: Identifier::vanilla_const("powered_rail"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("detector_rail"),
                Item {
                    id: Identifier::vanilla_const("detector_rail"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rail"),
                Item {
                    id: Identifier::vanilla_const("rail"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("activator_rail"),
                Item {
                    id: Identifier::vanilla_const("activator_rail"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("saddle"),
                Item {
                    id: Identifier::vanilla_const("saddle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("minecart"),
                Item {
                    id: Identifier::vanilla_const("minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chest_minecart"),
                Item {
                    id: Identifier::vanilla_const("chest_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("furnace_minecart"),
                Item {
                    id: Identifier::vanilla_const("furnace_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tnt_minecart"),
                Item {
                    id: Identifier::vanilla_const("tnt_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hopper_minecart"),
                Item {
                    id: Identifier::vanilla_const("hopper_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("carrot_on_a_stick"),
                Item {
                    id: Identifier::vanilla_const("carrot_on_a_stick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_fungus_on_a_stick"),
                Item {
                    id: Identifier::vanilla_const("warped_fungus_on_a_stick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("phantom_membrane"),
                Item {
                    id: Identifier::vanilla_const("phantom_membrane"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("elytra"),
                Item {
                    id: Identifier::vanilla_const("elytra"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_boat"),
                Item {
                    id: Identifier::vanilla_const("oak_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("oak_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_boat"),
                Item {
                    id: Identifier::vanilla_const("spruce_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("spruce_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_boat"),
                Item {
                    id: Identifier::vanilla_const("birch_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("birch_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_boat"),
                Item {
                    id: Identifier::vanilla_const("jungle_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("jungle_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_boat"),
                Item {
                    id: Identifier::vanilla_const("acacia_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("acacia_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_boat"),
                Item {
                    id: Identifier::vanilla_const("cherry_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("cherry_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_boat"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_boat"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_boat"),
                Item {
                    id: Identifier::vanilla_const("mangrove_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_chest_boat"),
                Item {
                    id: Identifier::vanilla_const("mangrove_chest_boat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_raft"),
                Item {
                    id: Identifier::vanilla_const("bamboo_raft"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_chest_raft"),
                Item {
                    id: Identifier::vanilla_const("bamboo_chest_raft"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("structure_block"),
                Item {
                    id: Identifier::vanilla_const("structure_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jigsaw"),
                Item {
                    id: Identifier::vanilla_const("jigsaw"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("test_block"),
                Item {
                    id: Identifier::vanilla_const("test_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("test_instance_block"),
                Item {
                    id: Identifier::vanilla_const("test_instance_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("turtle_helmet"),
                Item {
                    id: Identifier::vanilla_const("turtle_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("turtle_scute"),
                Item {
                    id: Identifier::vanilla_const("turtle_scute"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("armadillo_scute"),
                Item {
                    id: Identifier::vanilla_const("armadillo_scute"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wolf_armor"),
                Item {
                    id: Identifier::vanilla_const("wolf_armor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flint_and_steel"),
                Item {
                    id: Identifier::vanilla_const("flint_and_steel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bowl"),
                Item {
                    id: Identifier::vanilla_const("bowl"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("apple"),
                Item {
                    id: Identifier::vanilla_const("apple"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bow"),
                Item {
                    id: Identifier::vanilla_const("bow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("arrow"),
                Item {
                    id: Identifier::vanilla_const("arrow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("coal"),
                Item {
                    id: Identifier::vanilla_const("coal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("charcoal"),
                Item {
                    id: Identifier::vanilla_const("charcoal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond"),
                Item {
                    id: Identifier::vanilla_const("diamond"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("emerald"),
                Item {
                    id: Identifier::vanilla_const("emerald"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lapis_lazuli"),
                Item {
                    id: Identifier::vanilla_const("lapis_lazuli"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("quartz"),
                Item {
                    id: Identifier::vanilla_const("quartz"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("amethyst_shard"),
                Item {
                    id: Identifier::vanilla_const("amethyst_shard"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raw_iron"),
                Item {
                    id: Identifier::vanilla_const("raw_iron"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_ingot"),
                Item {
                    id: Identifier::vanilla_const("iron_ingot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raw_copper"),
                Item {
                    id: Identifier::vanilla_const("raw_copper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_ingot"),
                Item {
                    id: Identifier::vanilla_const("copper_ingot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raw_gold"),
                Item {
                    id: Identifier::vanilla_const("raw_gold"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gold_ingot"),
                Item {
                    id: Identifier::vanilla_const("gold_ingot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_ingot"),
                Item {
                    id: Identifier::vanilla_const("netherite_ingot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_scrap"),
                Item {
                    id: Identifier::vanilla_const("netherite_scrap"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wooden_sword"),
                Item {
                    id: Identifier::vanilla_const("wooden_sword"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wooden_shovel"),
                Item {
                    id: Identifier::vanilla_const("wooden_shovel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wooden_pickaxe"),
                Item {
                    id: Identifier::vanilla_const("wooden_pickaxe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wooden_axe"),
                Item {
                    id: Identifier::vanilla_const("wooden_axe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wooden_hoe"),
                Item {
                    id: Identifier::vanilla_const("wooden_hoe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_sword"),
                Item {
                    id: Identifier::vanilla_const("stone_sword"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_shovel"),
                Item {
                    id: Identifier::vanilla_const("stone_shovel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_pickaxe"),
                Item {
                    id: Identifier::vanilla_const("stone_pickaxe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_axe"),
                Item {
                    id: Identifier::vanilla_const("stone_axe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stone_hoe"),
                Item {
                    id: Identifier::vanilla_const("stone_hoe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_sword"),
                Item {
                    id: Identifier::vanilla_const("golden_sword"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_shovel"),
                Item {
                    id: Identifier::vanilla_const("golden_shovel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_pickaxe"),
                Item {
                    id: Identifier::vanilla_const("golden_pickaxe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_axe"),
                Item {
                    id: Identifier::vanilla_const("golden_axe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_hoe"),
                Item {
                    id: Identifier::vanilla_const("golden_hoe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_sword"),
                Item {
                    id: Identifier::vanilla_const("iron_sword"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_shovel"),
                Item {
                    id: Identifier::vanilla_const("iron_shovel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_pickaxe"),
                Item {
                    id: Identifier::vanilla_const("iron_pickaxe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_axe"),
                Item {
                    id: Identifier::vanilla_const("iron_axe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_hoe"),
                Item {
                    id: Identifier::vanilla_const("iron_hoe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_sword"),
                Item {
                    id: Identifier::vanilla_const("diamond_sword"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_shovel"),
                Item {
                    id: Identifier::vanilla_const("diamond_shovel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_pickaxe"),
                Item {
                    id: Identifier::vanilla_const("diamond_pickaxe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_axe"),
                Item {
                    id: Identifier::vanilla_const("diamond_axe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_hoe"),
                Item {
                    id: Identifier::vanilla_const("diamond_hoe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_sword"),
                Item {
                    id: Identifier::vanilla_const("netherite_sword"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_shovel"),
                Item {
                    id: Identifier::vanilla_const("netherite_shovel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_pickaxe"),
                Item {
                    id: Identifier::vanilla_const("netherite_pickaxe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_axe"),
                Item {
                    id: Identifier::vanilla_const("netherite_axe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_hoe"),
                Item {
                    id: Identifier::vanilla_const("netherite_hoe"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stick"),
                Item {
                    id: Identifier::vanilla_const("stick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mushroom_stew"),
                Item {
                    id: Identifier::vanilla_const("mushroom_stew"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("string"),
                Item {
                    id: Identifier::vanilla_const("string"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("feather"),
                Item {
                    id: Identifier::vanilla_const("feather"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gunpowder"),
                Item {
                    id: Identifier::vanilla_const("gunpowder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wheat_seeds"),
                Item {
                    id: Identifier::vanilla_const("wheat_seeds"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wheat"),
                Item {
                    id: Identifier::vanilla_const("wheat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bread"),
                Item {
                    id: Identifier::vanilla_const("bread"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leather_helmet"),
                Item {
                    id: Identifier::vanilla_const("leather_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leather_chestplate"),
                Item {
                    id: Identifier::vanilla_const("leather_chestplate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leather_leggings"),
                Item {
                    id: Identifier::vanilla_const("leather_leggings"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leather_boots"),
                Item {
                    id: Identifier::vanilla_const("leather_boots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chainmail_helmet"),
                Item {
                    id: Identifier::vanilla_const("chainmail_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chainmail_chestplate"),
                Item {
                    id: Identifier::vanilla_const("chainmail_chestplate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chainmail_leggings"),
                Item {
                    id: Identifier::vanilla_const("chainmail_leggings"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chainmail_boots"),
                Item {
                    id: Identifier::vanilla_const("chainmail_boots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_helmet"),
                Item {
                    id: Identifier::vanilla_const("iron_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_chestplate"),
                Item {
                    id: Identifier::vanilla_const("iron_chestplate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_leggings"),
                Item {
                    id: Identifier::vanilla_const("iron_leggings"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_boots"),
                Item {
                    id: Identifier::vanilla_const("iron_boots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_helmet"),
                Item {
                    id: Identifier::vanilla_const("diamond_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_chestplate"),
                Item {
                    id: Identifier::vanilla_const("diamond_chestplate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_leggings"),
                Item {
                    id: Identifier::vanilla_const("diamond_leggings"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_boots"),
                Item {
                    id: Identifier::vanilla_const("diamond_boots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_helmet"),
                Item {
                    id: Identifier::vanilla_const("golden_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_chestplate"),
                Item {
                    id: Identifier::vanilla_const("golden_chestplate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_leggings"),
                Item {
                    id: Identifier::vanilla_const("golden_leggings"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_boots"),
                Item {
                    id: Identifier::vanilla_const("golden_boots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_helmet"),
                Item {
                    id: Identifier::vanilla_const("netherite_helmet"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_chestplate"),
                Item {
                    id: Identifier::vanilla_const("netherite_chestplate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_leggings"),
                Item {
                    id: Identifier::vanilla_const("netherite_leggings"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_boots"),
                Item {
                    id: Identifier::vanilla_const("netherite_boots"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flint"),
                Item {
                    id: Identifier::vanilla_const("flint"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("porkchop"),
                Item {
                    id: Identifier::vanilla_const("porkchop"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_porkchop"),
                Item {
                    id: Identifier::vanilla_const("cooked_porkchop"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("painting"),
                Item {
                    id: Identifier::vanilla_const("painting"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_apple"),
                Item {
                    id: Identifier::vanilla_const("golden_apple"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enchanted_golden_apple"),
                Item {
                    id: Identifier::vanilla_const("enchanted_golden_apple"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_sign"),
                Item {
                    id: Identifier::vanilla_const("oak_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_sign"),
                Item {
                    id: Identifier::vanilla_const("spruce_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_sign"),
                Item {
                    id: Identifier::vanilla_const("birch_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_sign"),
                Item {
                    id: Identifier::vanilla_const("jungle_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_sign"),
                Item {
                    id: Identifier::vanilla_const("acacia_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_sign"),
                Item {
                    id: Identifier::vanilla_const("cherry_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_sign"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_sign"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_sign"),
                Item {
                    id: Identifier::vanilla_const("mangrove_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_sign"),
                Item {
                    id: Identifier::vanilla_const("bamboo_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_sign"),
                Item {
                    id: Identifier::vanilla_const("crimson_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_sign"),
                Item {
                    id: Identifier::vanilla_const("warped_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oak_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("oak_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spruce_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("spruce_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("birch_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("birch_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jungle_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("jungle_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("acacia_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("acacia_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cherry_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("cherry_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dark_oak_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("dark_oak_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pale_oak_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("pale_oak_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mangrove_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("mangrove_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bamboo_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("bamboo_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crimson_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("crimson_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warped_hanging_sign"),
                Item {
                    id: Identifier::vanilla_const("warped_hanging_sign"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bucket"),
                Item {
                    id: Identifier::vanilla_const("bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("water_bucket"),
                Item {
                    id: Identifier::vanilla_const("water_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lava_bucket"),
                Item {
                    id: Identifier::vanilla_const("lava_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("powder_snow_bucket"),
                Item {
                    id: Identifier::vanilla_const("powder_snow_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snowball"),
                Item {
                    id: Identifier::vanilla_const("snowball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leather"),
                Item {
                    id: Identifier::vanilla_const("leather"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("milk_bucket"),
                Item {
                    id: Identifier::vanilla_const("milk_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pufferfish_bucket"),
                Item {
                    id: Identifier::vanilla_const("pufferfish_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("salmon_bucket"),
                Item {
                    id: Identifier::vanilla_const("salmon_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cod_bucket"),
                Item {
                    id: Identifier::vanilla_const("cod_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tropical_fish_bucket"),
                Item {
                    id: Identifier::vanilla_const("tropical_fish_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("axolotl_bucket"),
                Item {
                    id: Identifier::vanilla_const("axolotl_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tadpole_bucket"),
                Item {
                    id: Identifier::vanilla_const("tadpole_bucket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brick"),
                Item {
                    id: Identifier::vanilla_const("brick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("clay_ball"),
                Item {
                    id: Identifier::vanilla_const("clay_ball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dried_kelp_block"),
                Item {
                    id: Identifier::vanilla_const("dried_kelp_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("paper"),
                Item {
                    id: Identifier::vanilla_const("paper"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("book"),
                Item {
                    id: Identifier::vanilla_const("book"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("slime_ball"),
                Item {
                    id: Identifier::vanilla_const("slime_ball"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("egg"),
                Item {
                    id: Identifier::vanilla_const("egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_egg"),
                Item {
                    id: Identifier::vanilla_const("blue_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_egg"),
                Item {
                    id: Identifier::vanilla_const("brown_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("compass"),
                Item {
                    id: Identifier::vanilla_const("compass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("recovery_compass"),
                Item {
                    id: Identifier::vanilla_const("recovery_compass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bundle"),
                Item {
                    id: Identifier::vanilla_const("bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_bundle"),
                Item {
                    id: Identifier::vanilla_const("white_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_bundle"),
                Item {
                    id: Identifier::vanilla_const("orange_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_bundle"),
                Item {
                    id: Identifier::vanilla_const("magenta_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_bundle"),
                Item {
                    id: Identifier::vanilla_const("light_blue_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_bundle"),
                Item {
                    id: Identifier::vanilla_const("yellow_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_bundle"),
                Item {
                    id: Identifier::vanilla_const("lime_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_bundle"),
                Item {
                    id: Identifier::vanilla_const("pink_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_bundle"),
                Item {
                    id: Identifier::vanilla_const("gray_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_bundle"),
                Item {
                    id: Identifier::vanilla_const("light_gray_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_bundle"),
                Item {
                    id: Identifier::vanilla_const("cyan_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_bundle"),
                Item {
                    id: Identifier::vanilla_const("purple_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_bundle"),
                Item {
                    id: Identifier::vanilla_const("blue_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_bundle"),
                Item {
                    id: Identifier::vanilla_const("brown_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_bundle"),
                Item {
                    id: Identifier::vanilla_const("green_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_bundle"),
                Item {
                    id: Identifier::vanilla_const("red_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_bundle"),
                Item {
                    id: Identifier::vanilla_const("black_bundle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fishing_rod"),
                Item {
                    id: Identifier::vanilla_const("fishing_rod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("clock"),
                Item {
                    id: Identifier::vanilla_const("clock"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spyglass"),
                Item {
                    id: Identifier::vanilla_const("spyglass"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glowstone_dust"),
                Item {
                    id: Identifier::vanilla_const("glowstone_dust"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cod"),
                Item {
                    id: Identifier::vanilla_const("cod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("salmon"),
                Item {
                    id: Identifier::vanilla_const("salmon"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tropical_fish"),
                Item {
                    id: Identifier::vanilla_const("tropical_fish"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pufferfish"),
                Item {
                    id: Identifier::vanilla_const("pufferfish"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_cod"),
                Item {
                    id: Identifier::vanilla_const("cooked_cod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_salmon"),
                Item {
                    id: Identifier::vanilla_const("cooked_salmon"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ink_sac"),
                Item {
                    id: Identifier::vanilla_const("ink_sac"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_ink_sac"),
                Item {
                    id: Identifier::vanilla_const("glow_ink_sac"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cocoa_beans"),
                Item {
                    id: Identifier::vanilla_const("cocoa_beans"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_dye"),
                Item {
                    id: Identifier::vanilla_const("white_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_dye"),
                Item {
                    id: Identifier::vanilla_const("orange_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_dye"),
                Item {
                    id: Identifier::vanilla_const("magenta_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_dye"),
                Item {
                    id: Identifier::vanilla_const("light_blue_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_dye"),
                Item {
                    id: Identifier::vanilla_const("yellow_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_dye"),
                Item {
                    id: Identifier::vanilla_const("lime_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_dye"),
                Item {
                    id: Identifier::vanilla_const("pink_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_dye"),
                Item {
                    id: Identifier::vanilla_const("gray_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_dye"),
                Item {
                    id: Identifier::vanilla_const("light_gray_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_dye"),
                Item {
                    id: Identifier::vanilla_const("cyan_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_dye"),
                Item {
                    id: Identifier::vanilla_const("purple_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_dye"),
                Item {
                    id: Identifier::vanilla_const("blue_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_dye"),
                Item {
                    id: Identifier::vanilla_const("brown_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_dye"),
                Item {
                    id: Identifier::vanilla_const("green_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_dye"),
                Item {
                    id: Identifier::vanilla_const("red_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_dye"),
                Item {
                    id: Identifier::vanilla_const("black_dye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bone_meal"),
                Item {
                    id: Identifier::vanilla_const("bone_meal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bone"),
                Item {
                    id: Identifier::vanilla_const("bone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sugar"),
                Item {
                    id: Identifier::vanilla_const("sugar"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cake"),
                Item {
                    id: Identifier::vanilla_const("cake"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_bed"),
                Item {
                    id: Identifier::vanilla_const("white_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_bed"),
                Item {
                    id: Identifier::vanilla_const("orange_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_bed"),
                Item {
                    id: Identifier::vanilla_const("magenta_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_bed"),
                Item {
                    id: Identifier::vanilla_const("light_blue_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_bed"),
                Item {
                    id: Identifier::vanilla_const("yellow_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_bed"),
                Item {
                    id: Identifier::vanilla_const("lime_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_bed"),
                Item {
                    id: Identifier::vanilla_const("pink_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_bed"),
                Item {
                    id: Identifier::vanilla_const("gray_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_bed"),
                Item {
                    id: Identifier::vanilla_const("light_gray_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_bed"),
                Item {
                    id: Identifier::vanilla_const("cyan_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_bed"),
                Item {
                    id: Identifier::vanilla_const("purple_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_bed"),
                Item {
                    id: Identifier::vanilla_const("blue_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_bed"),
                Item {
                    id: Identifier::vanilla_const("brown_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_bed"),
                Item {
                    id: Identifier::vanilla_const("green_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_bed"),
                Item {
                    id: Identifier::vanilla_const("red_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_bed"),
                Item {
                    id: Identifier::vanilla_const("black_bed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cookie"),
                Item {
                    id: Identifier::vanilla_const("cookie"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crafter"),
                Item {
                    id: Identifier::vanilla_const("crafter"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("filled_map"),
                Item {
                    id: Identifier::vanilla_const("filled_map"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shears"),
                Item {
                    id: Identifier::vanilla_const("shears"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("melon_slice"),
                Item {
                    id: Identifier::vanilla_const("melon_slice"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dried_kelp"),
                Item {
                    id: Identifier::vanilla_const("dried_kelp"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pumpkin_seeds"),
                Item {
                    id: Identifier::vanilla_const("pumpkin_seeds"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("melon_seeds"),
                Item {
                    id: Identifier::vanilla_const("melon_seeds"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("beef"),
                Item {
                    id: Identifier::vanilla_const("beef"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_beef"),
                Item {
                    id: Identifier::vanilla_const("cooked_beef"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chicken"),
                Item {
                    id: Identifier::vanilla_const("chicken"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_chicken"),
                Item {
                    id: Identifier::vanilla_const("cooked_chicken"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rotten_flesh"),
                Item {
                    id: Identifier::vanilla_const("rotten_flesh"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ender_pearl"),
                Item {
                    id: Identifier::vanilla_const("ender_pearl"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blaze_rod"),
                Item {
                    id: Identifier::vanilla_const("blaze_rod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ghast_tear"),
                Item {
                    id: Identifier::vanilla_const("ghast_tear"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gold_nugget"),
                Item {
                    id: Identifier::vanilla_const("gold_nugget"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_wart"),
                Item {
                    id: Identifier::vanilla_const("nether_wart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glass_bottle"),
                Item {
                    id: Identifier::vanilla_const("glass_bottle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("potion"),
                Item {
                    id: Identifier::vanilla_const("potion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spider_eye"),
                Item {
                    id: Identifier::vanilla_const("spider_eye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fermented_spider_eye"),
                Item {
                    id: Identifier::vanilla_const("fermented_spider_eye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blaze_powder"),
                Item {
                    id: Identifier::vanilla_const("blaze_powder"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magma_cream"),
                Item {
                    id: Identifier::vanilla_const("magma_cream"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brewing_stand"),
                Item {
                    id: Identifier::vanilla_const("brewing_stand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cauldron"),
                Item {
                    id: Identifier::vanilla_const("cauldron"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ender_eye"),
                Item {
                    id: Identifier::vanilla_const("ender_eye"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glistering_melon_slice"),
                Item {
                    id: Identifier::vanilla_const("glistering_melon_slice"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("armadillo_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("armadillo_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("allay_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("allay_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("axolotl_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("axolotl_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bat_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("bat_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bee_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("bee_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blaze_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("blaze_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bogged_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("bogged_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("breeze_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("breeze_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cat_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("cat_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("camel_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("camel_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cave_spider_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("cave_spider_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chicken_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("chicken_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cod_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("cod_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cow_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("cow_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creeper_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("creeper_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dolphin_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("dolphin_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("donkey_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("donkey_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("drowned_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("drowned_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("elder_guardian_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("elder_guardian_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ender_dragon_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("ender_dragon_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enderman_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("enderman_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("endermite_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("endermite_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("evoker_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("evoker_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fox_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("fox_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("frog_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("frog_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ghast_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("ghast_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_squid_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("glow_squid_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("goat_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("goat_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("guardian_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("guardian_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("hoglin_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("hoglin_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("horse_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("horse_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("husk_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("husk_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_golem_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("iron_golem_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("llama_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("llama_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magma_cube_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("magma_cube_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mooshroom_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("mooshroom_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mule_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("mule_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ocelot_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("ocelot_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("panda_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("panda_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("parrot_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("parrot_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("phantom_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("phantom_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pig_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("pig_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piglin_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("piglin_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piglin_brute_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("piglin_brute_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pillager_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("pillager_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polar_bear_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("polar_bear_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pufferfish_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("pufferfish_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rabbit_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("rabbit_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ravager_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("ravager_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("salmon_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("salmon_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sheep_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("sheep_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shulker_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("shulker_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("silverfish_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("silverfish_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skeleton_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("skeleton_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skeleton_horse_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("skeleton_horse_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("slime_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("slime_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sniffer_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("sniffer_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snow_golem_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("snow_golem_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spider_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("spider_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("squid_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("squid_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stray_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("stray_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("strider_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("strider_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tadpole_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("tadpole_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trader_llama_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("trader_llama_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tropical_fish_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("tropical_fish_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("turtle_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("turtle_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vex_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("vex_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("villager_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("villager_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vindicator_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("vindicator_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wandering_trader_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("wandering_trader_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("warden_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("warden_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("witch_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("witch_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("wither_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither_skeleton_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("wither_skeleton_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wolf_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("wolf_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zoglin_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("zoglin_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creaking_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("creaking_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("zombie_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie_horse_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("zombie_horse_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie_villager_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("zombie_villager_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombified_piglin_spawn_egg"),
                Item {
                    id: Identifier::vanilla_const("zombified_piglin_spawn_egg"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("experience_bottle"),
                Item {
                    id: Identifier::vanilla_const("experience_bottle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fire_charge"),
                Item {
                    id: Identifier::vanilla_const("fire_charge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wind_charge"),
                Item {
                    id: Identifier::vanilla_const("wind_charge"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("writable_book"),
                Item {
                    id: Identifier::vanilla_const("writable_book"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("written_book"),
                Item {
                    id: Identifier::vanilla_const("written_book"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("breeze_rod"),
                Item {
                    id: Identifier::vanilla_const("breeze_rod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mace"),
                Item {
                    id: Identifier::vanilla_const("mace"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item_frame"),
                Item {
                    id: Identifier::vanilla_const("item_frame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_item_frame"),
                Item {
                    id: Identifier::vanilla_const("glow_item_frame"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flower_pot"),
                Item {
                    id: Identifier::vanilla_const("flower_pot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("carrot"),
                Item {
                    id: Identifier::vanilla_const("carrot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("potato"),
                Item {
                    id: Identifier::vanilla_const("potato"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("baked_potato"),
                Item {
                    id: Identifier::vanilla_const("baked_potato"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("poisonous_potato"),
                Item {
                    id: Identifier::vanilla_const("poisonous_potato"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("map"),
                Item {
                    id: Identifier::vanilla_const("map"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_carrot"),
                Item {
                    id: Identifier::vanilla_const("golden_carrot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skeleton_skull"),
                Item {
                    id: Identifier::vanilla_const("skeleton_skull"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wither_skeleton_skull"),
                Item {
                    id: Identifier::vanilla_const("wither_skeleton_skull"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("player_head"),
                Item {
                    id: Identifier::vanilla_const("player_head"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("zombie_head"),
                Item {
                    id: Identifier::vanilla_const("zombie_head"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creeper_head"),
                Item {
                    id: Identifier::vanilla_const("creeper_head"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dragon_head"),
                Item {
                    id: Identifier::vanilla_const("dragon_head"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piglin_head"),
                Item {
                    id: Identifier::vanilla_const("piglin_head"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_star"),
                Item {
                    id: Identifier::vanilla_const("nether_star"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pumpkin_pie"),
                Item {
                    id: Identifier::vanilla_const("pumpkin_pie"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("firework_rocket"),
                Item {
                    id: Identifier::vanilla_const("firework_rocket"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("firework_star"),
                Item {
                    id: Identifier::vanilla_const("firework_star"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enchanted_book"),
                Item {
                    id: Identifier::vanilla_const("enchanted_book"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nether_brick"),
                Item {
                    id: Identifier::vanilla_const("nether_brick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("resin_brick"),
                Item {
                    id: Identifier::vanilla_const("resin_brick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_shard"),
                Item {
                    id: Identifier::vanilla_const("prismarine_shard"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prismarine_crystals"),
                Item {
                    id: Identifier::vanilla_const("prismarine_crystals"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rabbit"),
                Item {
                    id: Identifier::vanilla_const("rabbit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_rabbit"),
                Item {
                    id: Identifier::vanilla_const("cooked_rabbit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rabbit_stew"),
                Item {
                    id: Identifier::vanilla_const("rabbit_stew"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rabbit_foot"),
                Item {
                    id: Identifier::vanilla_const("rabbit_foot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rabbit_hide"),
                Item {
                    id: Identifier::vanilla_const("rabbit_hide"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("armor_stand"),
                Item {
                    id: Identifier::vanilla_const("armor_stand"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_horse_armor"),
                Item {
                    id: Identifier::vanilla_const("iron_horse_armor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("golden_horse_armor"),
                Item {
                    id: Identifier::vanilla_const("golden_horse_armor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("diamond_horse_armor"),
                Item {
                    id: Identifier::vanilla_const("diamond_horse_armor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("leather_horse_armor"),
                Item {
                    id: Identifier::vanilla_const("leather_horse_armor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lead"),
                Item {
                    id: Identifier::vanilla_const("lead"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("name_tag"),
                Item {
                    id: Identifier::vanilla_const("name_tag"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("command_block_minecart"),
                Item {
                    id: Identifier::vanilla_const("command_block_minecart"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mutton"),
                Item {
                    id: Identifier::vanilla_const("mutton"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cooked_mutton"),
                Item {
                    id: Identifier::vanilla_const("cooked_mutton"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_banner"),
                Item {
                    id: Identifier::vanilla_const("white_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_banner"),
                Item {
                    id: Identifier::vanilla_const("orange_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_banner"),
                Item {
                    id: Identifier::vanilla_const("magenta_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_banner"),
                Item {
                    id: Identifier::vanilla_const("light_blue_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_banner"),
                Item {
                    id: Identifier::vanilla_const("yellow_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_banner"),
                Item {
                    id: Identifier::vanilla_const("lime_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_banner"),
                Item {
                    id: Identifier::vanilla_const("pink_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_banner"),
                Item {
                    id: Identifier::vanilla_const("gray_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_banner"),
                Item {
                    id: Identifier::vanilla_const("light_gray_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_banner"),
                Item {
                    id: Identifier::vanilla_const("cyan_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_banner"),
                Item {
                    id: Identifier::vanilla_const("purple_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_banner"),
                Item {
                    id: Identifier::vanilla_const("blue_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_banner"),
                Item {
                    id: Identifier::vanilla_const("brown_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_banner"),
                Item {
                    id: Identifier::vanilla_const("green_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_banner"),
                Item {
                    id: Identifier::vanilla_const("red_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_banner"),
                Item {
                    id: Identifier::vanilla_const("black_banner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("end_crystal"),
                Item {
                    id: Identifier::vanilla_const("end_crystal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chorus_fruit"),
                Item {
                    id: Identifier::vanilla_const("chorus_fruit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("popped_chorus_fruit"),
                Item {
                    id: Identifier::vanilla_const("popped_chorus_fruit"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("torchflower_seeds"),
                Item {
                    id: Identifier::vanilla_const("torchflower_seeds"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pitcher_pod"),
                Item {
                    id: Identifier::vanilla_const("pitcher_pod"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("beetroot"),
                Item {
                    id: Identifier::vanilla_const("beetroot"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("beetroot_seeds"),
                Item {
                    id: Identifier::vanilla_const("beetroot_seeds"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("beetroot_soup"),
                Item {
                    id: Identifier::vanilla_const("beetroot_soup"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dragon_breath"),
                Item {
                    id: Identifier::vanilla_const("dragon_breath"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("splash_potion"),
                Item {
                    id: Identifier::vanilla_const("splash_potion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spectral_arrow"),
                Item {
                    id: Identifier::vanilla_const("spectral_arrow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tipped_arrow"),
                Item {
                    id: Identifier::vanilla_const("tipped_arrow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lingering_potion"),
                Item {
                    id: Identifier::vanilla_const("lingering_potion"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shield"),
                Item {
                    id: Identifier::vanilla_const("shield"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("totem_of_undying"),
                Item {
                    id: Identifier::vanilla_const("totem_of_undying"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shulker_shell"),
                Item {
                    id: Identifier::vanilla_const("shulker_shell"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("iron_nugget"),
                Item {
                    id: Identifier::vanilla_const("iron_nugget"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("knowledge_book"),
                Item {
                    id: Identifier::vanilla_const("knowledge_book"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("debug_stick"),
                Item {
                    id: Identifier::vanilla_const("debug_stick"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_13"),
                Item {
                    id: Identifier::vanilla_const("music_disc_13"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_cat"),
                Item {
                    id: Identifier::vanilla_const("music_disc_cat"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_blocks"),
                Item {
                    id: Identifier::vanilla_const("music_disc_blocks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_chirp"),
                Item {
                    id: Identifier::vanilla_const("music_disc_chirp"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_creator"),
                Item {
                    id: Identifier::vanilla_const("music_disc_creator"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_creator_music_box"),
                Item {
                    id: Identifier::vanilla_const("music_disc_creator_music_box"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_far"),
                Item {
                    id: Identifier::vanilla_const("music_disc_far"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_mall"),
                Item {
                    id: Identifier::vanilla_const("music_disc_mall"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_mellohi"),
                Item {
                    id: Identifier::vanilla_const("music_disc_mellohi"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_stal"),
                Item {
                    id: Identifier::vanilla_const("music_disc_stal"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_strad"),
                Item {
                    id: Identifier::vanilla_const("music_disc_strad"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_ward"),
                Item {
                    id: Identifier::vanilla_const("music_disc_ward"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_11"),
                Item {
                    id: Identifier::vanilla_const("music_disc_11"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_wait"),
                Item {
                    id: Identifier::vanilla_const("music_disc_wait"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_otherside"),
                Item {
                    id: Identifier::vanilla_const("music_disc_otherside"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_relic"),
                Item {
                    id: Identifier::vanilla_const("music_disc_relic"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_5"),
                Item {
                    id: Identifier::vanilla_const("music_disc_5"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_pigstep"),
                Item {
                    id: Identifier::vanilla_const("music_disc_pigstep"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc_precipice"),
                Item {
                    id: Identifier::vanilla_const("music_disc_precipice"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("disc_fragment_5"),
                Item {
                    id: Identifier::vanilla_const("disc_fragment_5"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trident"),
                Item {
                    id: Identifier::vanilla_const("trident"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("nautilus_shell"),
                Item {
                    id: Identifier::vanilla_const("nautilus_shell"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("heart_of_the_sea"),
                Item {
                    id: Identifier::vanilla_const("heart_of_the_sea"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crossbow"),
                Item {
                    id: Identifier::vanilla_const("crossbow"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("suspicious_stew"),
                Item {
                    id: Identifier::vanilla_const("suspicious_stew"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("loom"),
                Item {
                    id: Identifier::vanilla_const("loom"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flower_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("flower_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("creeper_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("creeper_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skull_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("skull_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mojang_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("mojang_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("globe_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("globe_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("piglin_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("piglin_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flow_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("flow_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("guster_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("guster_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("field_masoned_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("field_masoned_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bordure_indented_banner_pattern"),
                Item {
                    id: Identifier::vanilla_const("bordure_indented_banner_pattern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("goat_horn"),
                Item {
                    id: Identifier::vanilla_const("goat_horn"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("composter"),
                Item {
                    id: Identifier::vanilla_const("composter"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("barrel"),
                Item {
                    id: Identifier::vanilla_const("barrel"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smoker"),
                Item {
                    id: Identifier::vanilla_const("smoker"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blast_furnace"),
                Item {
                    id: Identifier::vanilla_const("blast_furnace"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cartography_table"),
                Item {
                    id: Identifier::vanilla_const("cartography_table"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fletching_table"),
                Item {
                    id: Identifier::vanilla_const("fletching_table"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("grindstone"),
                Item {
                    id: Identifier::vanilla_const("grindstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("smithing_table"),
                Item {
                    id: Identifier::vanilla_const("smithing_table"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("stonecutter"),
                Item {
                    id: Identifier::vanilla_const("stonecutter"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bell"),
                Item {
                    id: Identifier::vanilla_const("bell"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lantern"),
                Item {
                    id: Identifier::vanilla_const("lantern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul_lantern"),
                Item {
                    id: Identifier::vanilla_const("soul_lantern"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sweet_berries"),
                Item {
                    id: Identifier::vanilla_const("sweet_berries"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("glow_berries"),
                Item {
                    id: Identifier::vanilla_const("glow_berries"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("campfire"),
                Item {
                    id: Identifier::vanilla_const("campfire"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("soul_campfire"),
                Item {
                    id: Identifier::vanilla_const("soul_campfire"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shroomlight"),
                Item {
                    id: Identifier::vanilla_const("shroomlight"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("honeycomb"),
                Item {
                    id: Identifier::vanilla_const("honeycomb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bee_nest"),
                Item {
                    id: Identifier::vanilla_const("bee_nest"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("beehive"),
                Item {
                    id: Identifier::vanilla_const("beehive"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("honey_bottle"),
                Item {
                    id: Identifier::vanilla_const("honey_bottle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("honeycomb_block"),
                Item {
                    id: Identifier::vanilla_const("honeycomb_block"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lodestone"),
                Item {
                    id: Identifier::vanilla_const("lodestone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("crying_obsidian"),
                Item {
                    id: Identifier::vanilla_const("crying_obsidian"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blackstone"),
                Item {
                    id: Identifier::vanilla_const("blackstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blackstone_slab"),
                Item {
                    id: Identifier::vanilla_const("blackstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blackstone_stairs"),
                Item {
                    id: Identifier::vanilla_const("blackstone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gilded_blackstone"),
                Item {
                    id: Identifier::vanilla_const("gilded_blackstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("chiseled_polished_blackstone"),
                Item {
                    id: Identifier::vanilla_const("chiseled_polished_blackstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_bricks"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_brick_slab"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_brick_slab"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("polished_blackstone_brick_stairs"),
                Item {
                    id: Identifier::vanilla_const("polished_blackstone_brick_stairs"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cracked_polished_blackstone_bricks"),
                Item {
                    id: Identifier::vanilla_const("cracked_polished_blackstone_bricks"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("respawn_anchor"),
                Item {
                    id: Identifier::vanilla_const("respawn_anchor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("candle"),
                Item {
                    id: Identifier::vanilla_const("candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("white_candle"),
                Item {
                    id: Identifier::vanilla_const("white_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("orange_candle"),
                Item {
                    id: Identifier::vanilla_const("orange_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("magenta_candle"),
                Item {
                    id: Identifier::vanilla_const("magenta_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_blue_candle"),
                Item {
                    id: Identifier::vanilla_const("light_blue_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("yellow_candle"),
                Item {
                    id: Identifier::vanilla_const("yellow_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("lime_candle"),
                Item {
                    id: Identifier::vanilla_const("lime_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pink_candle"),
                Item {
                    id: Identifier::vanilla_const("pink_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gray_candle"),
                Item {
                    id: Identifier::vanilla_const("gray_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("light_gray_candle"),
                Item {
                    id: Identifier::vanilla_const("light_gray_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("cyan_candle"),
                Item {
                    id: Identifier::vanilla_const("cyan_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("purple_candle"),
                Item {
                    id: Identifier::vanilla_const("purple_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blue_candle"),
                Item {
                    id: Identifier::vanilla_const("blue_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brown_candle"),
                Item {
                    id: Identifier::vanilla_const("brown_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("green_candle"),
                Item {
                    id: Identifier::vanilla_const("green_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("red_candle"),
                Item {
                    id: Identifier::vanilla_const("red_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("black_candle"),
                Item {
                    id: Identifier::vanilla_const("black_candle"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("small_amethyst_bud"),
                Item {
                    id: Identifier::vanilla_const("small_amethyst_bud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("medium_amethyst_bud"),
                Item {
                    id: Identifier::vanilla_const("medium_amethyst_bud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("large_amethyst_bud"),
                Item {
                    id: Identifier::vanilla_const("large_amethyst_bud"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("amethyst_cluster"),
                Item {
                    id: Identifier::vanilla_const("amethyst_cluster"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pointed_dripstone"),
                Item {
                    id: Identifier::vanilla_const("pointed_dripstone"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ochre_froglight"),
                Item {
                    id: Identifier::vanilla_const("ochre_froglight"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("verdant_froglight"),
                Item {
                    id: Identifier::vanilla_const("verdant_froglight"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("pearlescent_froglight"),
                Item {
                    id: Identifier::vanilla_const("pearlescent_froglight"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("frogspawn"),
                Item {
                    id: Identifier::vanilla_const("frogspawn"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("echo_shard"),
                Item {
                    id: Identifier::vanilla_const("echo_shard"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brush"),
                Item {
                    id: Identifier::vanilla_const("brush"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("netherite_upgrade_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("netherite_upgrade_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sentry_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("sentry_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("dune_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("dune_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("coast_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("coast_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wild_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("wild_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ward_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("ward_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("eye_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("eye_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vex_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("vex_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tide_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("tide_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snout_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("snout_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("rib_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("rib_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spire_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("spire_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("wayfinder_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const(
                        "wayfinder_armor_trim_smithing_template",
                    ),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shaper_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("shaper_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("silence_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("silence_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("raiser_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("raiser_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("host_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("host_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flow_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("flow_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("bolt_armor_trim_smithing_template"),
                Item {
                    id: Identifier::vanilla_const("bolt_armor_trim_smithing_template"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("angler_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("angler_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("archer_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("archer_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("arms_up_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("arms_up_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("blade_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("blade_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("brewer_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("brewer_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("burn_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("burn_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("danger_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("danger_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("explorer_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("explorer_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flow_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("flow_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("friend_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("friend_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("guster_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("guster_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("heart_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("heart_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("heartbreak_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("heartbreak_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("howl_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("howl_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("miner_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("miner_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mourner_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("mourner_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("plenty_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("plenty_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("prize_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("prize_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("scrape_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("scrape_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sheaf_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("sheaf_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("shelter_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("shelter_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("skull_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("skull_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("snort_pottery_sherd"),
                Item {
                    id: Identifier::vanilla_const("snort_pottery_sherd"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_grate"),
                Item {
                    id: Identifier::vanilla_const("copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("exposed_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("weathered_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("oxidized_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("waxed_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_copper_grate"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_copper_grate"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("exposed_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("exposed_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weathered_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("weathered_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxidized_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("oxidized_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("waxed_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_exposed_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("waxed_exposed_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_weathered_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("waxed_weathered_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("waxed_oxidized_copper_bulb"),
                Item {
                    id: Identifier::vanilla_const("waxed_oxidized_copper_bulb"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trial_spawner"),
                Item {
                    id: Identifier::vanilla_const("trial_spawner"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("trial_key"),
                Item {
                    id: Identifier::vanilla_const("trial_key"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ominous_trial_key"),
                Item {
                    id: Identifier::vanilla_const("ominous_trial_key"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("vault"),
                Item {
                    id: Identifier::vanilla_const("vault"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ominous_bottle"),
                Item {
                    id: Identifier::vanilla_const("ominous_bottle"),
                },
            );
        registry
    }
}
