use crate::packet::*;
impl SoundEvent {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<SoundEvent> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.ambient_with_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.ambient_with_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.ambient_without_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.ambient_without_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.item_given"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.item_given"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.item_taken"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.item_taken"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.allay.item_thrown"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.allay.item_thrown"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.cave"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.cave"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.basalt_deltas.additions"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.basalt_deltas.additions"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.basalt_deltas.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.basalt_deltas.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.basalt_deltas.mood"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.basalt_deltas.mood"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.crimson_forest.additions"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.crimson_forest.additions"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.crimson_forest.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.crimson_forest.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.crimson_forest.mood"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.crimson_forest.mood"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.nether_wastes.additions"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.nether_wastes.additions"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.nether_wastes.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.nether_wastes.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.nether_wastes.mood"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.nether_wastes.mood"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.soul_sand_valley.additions"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "ambient.soul_sand_valley.additions",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.soul_sand_valley.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.soul_sand_valley.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.soul_sand_valley.mood"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.soul_sand_valley.mood"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.warped_forest.additions"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.warped_forest.additions"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.warped_forest.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.warped_forest.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.warped_forest.mood"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.warped_forest.mood"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.underwater.enter"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.underwater.enter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.underwater.exit"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.underwater.exit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.underwater.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.underwater.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.underwater.loop.additions"),
                SoundEvent {
                    name: Identifier::vanilla_const("ambient.underwater.loop.additions"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ambient.underwater.loop.additions.rare"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "ambient.underwater.loop.additions.rare",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const(
                    "ambient.underwater.loop.additions.ultra_rare",
                ),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "ambient.underwater.loop.additions.ultra_rare",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.chime"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.chime"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.resonate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.resonate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_cluster.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_cluster.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_cluster.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_cluster.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_cluster.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_cluster.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_cluster.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_cluster.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.amethyst_cluster.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.amethyst_cluster.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ancient_debris.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ancient_debris.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ancient_debris.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ancient_debris.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ancient_debris.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ancient_debris.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ancient_debris.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ancient_debris.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ancient_debris.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ancient_debris.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.destroy"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.destroy"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.land"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.anvil.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.anvil.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.hurt_reduced"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.hurt_reduced"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.roll"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.roll"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.scute_drop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.scute_drop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.unroll_finish"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.unroll_finish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.peek"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.peek"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.unroll_start"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.unroll_start"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armadillo.brush"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armadillo.brush"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_chain"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_chain"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_diamond"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_diamond"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_elytra"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_elytra"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_generic"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_generic"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_gold"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_gold"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_iron"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_iron"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_leather"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_leather"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_netherite"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_netherite"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_turtle"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_turtle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.equip_wolf"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.equip_wolf"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.armor.unequip_wolf"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.armor.unequip_wolf"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armor_stand.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armor_stand.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armor_stand.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armor_stand.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armor_stand.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armor_stand.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.armor_stand.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.armor_stand.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.arrow.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.arrow.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.arrow.hit_player"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.arrow.hit_player"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.arrow.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.arrow.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.axe.strip"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.axe.strip"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.axe.scrape"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.axe.scrape"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.axe.wax_off"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.axe.wax_off"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.idle_air"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.idle_air"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.idle_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.idle_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.splash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.splash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.axolotl.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.axolotl.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea_leaves.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea_leaves.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea_leaves.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea_leaves.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea_leaves.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea_leaves.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea_leaves.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea_leaves.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.azalea_leaves.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.azalea_leaves.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_sapling.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_sapling.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_sapling.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_sapling.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_sapling.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_sapling.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_door.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood_door.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_door.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood_door.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_trapdoor.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood_trapdoor.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_trapdoor.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood_trapdoor.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_button.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_button.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_button.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood_button.click_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_pressure_plate.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_pressure_plate.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_pressure_plate.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_pressure_plate.click_on",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_fence_gate.close"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_fence_gate.close",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_fence_gate.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bamboo_wood_fence_gate.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.barrel.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.barrel.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.barrel.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.barrel.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.basalt.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.basalt.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.basalt.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.basalt.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.basalt.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.basalt.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.basalt.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.basalt.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.basalt.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.basalt.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bat.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bat.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bat.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bat.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bat.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bat.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bat.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bat.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bat.takeoff"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bat.takeoff"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beacon.activate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beacon.activate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beacon.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beacon.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beacon.deactivate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beacon.deactivate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beacon.power_select"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beacon.power_select"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bee.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bee.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bee.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bee.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bee.loop_aggressive"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bee.loop_aggressive"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bee.loop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bee.loop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bee.sting"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bee.sting"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bee.pollinate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bee.pollinate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beehive.drip"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beehive.drip"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beehive.enter"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beehive.enter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beehive.exit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beehive.exit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beehive.shear"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beehive.shear"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.beehive.work"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.beehive.work"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bell.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bell.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bell.resonate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bell.resonate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.blaze.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.blaze.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.blaze.burn"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.blaze.burn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.blaze.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.blaze.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.blaze.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.blaze.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.blaze.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.blaze.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.boat.paddle_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.boat.paddle_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.boat.paddle_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.boat.paddle_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bogged.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bogged.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bogged.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bogged.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bogged.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bogged.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bogged.shear"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bogged.shear"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.bogged.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.bogged.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bone_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bone_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bone_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bone_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bone_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bone_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bone_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bone_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bone_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bone_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bone_meal.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bone_meal.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.book.page_turn"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.book.page_turn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.book.put"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.book.put"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.blastfurnace.fire_crackle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.blastfurnace.fire_crackle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bottle.empty"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bottle.empty"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bottle.fill"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bottle.fill"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bottle.fill_dragonbreath"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bottle.fill_dragonbreath"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.charge"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.charge"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.deflect"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.deflect"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.inhale"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.inhale"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.idle_ground"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.idle_ground"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.idle_air"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.idle_air"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.slide"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.slide"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.whirl"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.whirl"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.breeze.wind_burst"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.breeze.wind_burst"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.brewing_stand.brew"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.brewing_stand.brew"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.brush.brushing.generic"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.brush.brushing.generic"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.brush.brushing.sand"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.brush.brushing.sand"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.brush.brushing.gravel"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.brush.brushing.gravel"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.brush.brushing.sand.complete"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.brush.brushing.sand.complete"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.brush.brushing.gravel.complete"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "item.brush.brushing.gravel.complete",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bubble_column.bubble_pop"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.bubble_column.bubble_pop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bubble_column.upwards_ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bubble_column.upwards_ambient",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bubble_column.upwards_inside"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bubble_column.upwards_inside",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bubble_column.whirlpool_ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bubble_column.whirlpool_ambient",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bubble_column.whirlpool_inside"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bubble_column.whirlpool_inside",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.hud.bubble_pop"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.hud.bubble_pop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.empty"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.empty"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.empty_axolotl"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.empty_axolotl"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.empty_fish"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.empty_fish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.empty_lava"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.empty_lava"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.empty_powder_snow"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.empty_powder_snow"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.empty_tadpole"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.empty_tadpole"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.fill"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.fill"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.fill_axolotl"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.fill_axolotl"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.fill_fish"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.fill_fish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.fill_lava"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.fill_lava"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.fill_powder_snow"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.fill_powder_snow"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bucket.fill_tadpole"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bucket.fill_tadpole"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bundle.drop_contents"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bundle.drop_contents"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bundle.insert"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bundle.insert"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bundle.insert_fail"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bundle.insert_fail"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.bundle.remove_one"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.bundle.remove_one"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cake.add_candle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cake.add_candle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.calcite.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.calcite.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.calcite.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.calcite.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.calcite.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.calcite.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.calcite.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.calcite.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.calcite.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.calcite.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.dash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.dash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.dash_ready"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.dash_ready"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.saddle"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.saddle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.sit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.sit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.stand"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.stand"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.camel.step_sand"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.camel.step_sand"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.campfire.crackle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.campfire.crackle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.extinguish"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.extinguish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.candle.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.candle.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.stray_ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.stray_ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.hiss"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.hiss"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.beg_for_food"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.beg_for_food"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.purr"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.purr"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cat.purreow"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cat.purreow"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cave_vines.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cave_vines.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cave_vines.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cave_vines.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cave_vines.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cave_vines.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cave_vines.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cave_vines.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cave_vines.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cave_vines.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cave_vines.pick_berries"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cave_vines.pick_berries"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chain.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chain.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chain.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chain.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chain.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chain.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chain.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chain.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chain.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chain.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_sapling.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_sapling.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_sapling.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_sapling.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_sapling.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_sapling.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_sapling.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_sapling.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_sapling.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_sapling.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_leaves.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_leaves.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_leaves.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_leaves.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_leaves.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_leaves.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_leaves.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_leaves.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_leaves.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_leaves.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_hanging_sign.step"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_hanging_sign.step",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_hanging_sign.break"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_hanging_sign.break",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_hanging_sign.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_hanging_sign.fall",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_hanging_sign.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_hanging_sign.hit",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_hanging_sign.place"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_hanging_sign.place",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_door.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood_door.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_door.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood_door.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_trapdoor.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood_trapdoor.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_trapdoor.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood_trapdoor.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_button.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_button.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_button.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood_button.click_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_pressure_plate.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_pressure_plate.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_pressure_plate.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_pressure_plate.click_on",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_fence_gate.close"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.cherry_wood_fence_gate.close",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cherry_wood_fence_gate.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cherry_wood_fence_gate.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chest.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chest.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chest.locked"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chest.locked"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chest.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chest.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.chicken.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.chicken.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.chicken.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.chicken.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.chicken.egg"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.chicken.egg"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.chicken.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.chicken.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.chicken.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.chicken.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.insert"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.insert"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.insert.enchanted"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.chiseled_bookshelf.insert.enchanted",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.pickup"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.pickup"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.pickup.enchanted"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.chiseled_bookshelf.pickup.enchanted",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chiseled_bookshelf.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chiseled_bookshelf.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chorus_flower.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chorus_flower.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.chorus_flower.grow"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.chorus_flower.grow"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.chorus_fruit.teleport"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.chorus_fruit.teleport"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cobweb.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cobweb.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cobweb.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cobweb.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cobweb.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cobweb.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cobweb.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cobweb.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.cobweb.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.cobweb.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cod.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cod.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cod.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cod.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cod.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cod.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cod.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cod.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.comparator.click"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.comparator.click"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.composter.empty"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.composter.empty"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.composter.fill"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.composter.fill"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.composter.fill_success"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.composter.fill_success"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.composter.ready"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.composter.ready"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.conduit.activate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.conduit.activate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.conduit.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.conduit.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.conduit.ambient.short"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.conduit.ambient.short"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.conduit.attack.target"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.conduit.attack.target"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.conduit.deactivate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.conduit.deactivate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.turn_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.turn_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_bulb.turn_off"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_bulb.turn_off"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_door.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_door.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_door.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_door.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_grate.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_grate.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_grate.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_grate.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_grate.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_grate.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_grate.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_grate.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_grate.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_grate.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_trapdoor.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_trapdoor.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.copper_trapdoor.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.copper_trapdoor.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.coral_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.coral_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.coral_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.coral_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.coral_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.coral_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.coral_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.coral_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.coral_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.coral_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cow.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cow.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cow.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cow.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cow.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cow.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cow.milk"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cow.milk"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.cow.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.cow.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.crafter.craft"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.crafter.craft"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.crafter.fail"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.crafter.fail"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.activate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.activate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.deactivate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.deactivate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.freeze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.freeze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.unfreeze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.unfreeze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.sway"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.sway"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creaking.twitch"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creaking.twitch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.idle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.idle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.creaking_heart.spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.creaking_heart.spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creeper.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creeper.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creeper.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creeper.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.creeper.primed"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.creeper.primed"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.crop.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.crop.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crop.plant"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crop.plant"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.loading_end"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.loading_end"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.loading_middle"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.loading_middle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.loading_start"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.loading_start"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.quick_charge_1"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.quick_charge_1"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.quick_charge_2"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.quick_charge_2"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.quick_charge_3"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.quick_charge_3"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.crossbow.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.crossbow.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.insert"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.insert"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.insert_fail"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.insert_fail"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.decorated_pot.shatter"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.decorated_pot.shatter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_bricks.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_bricks.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_bricks.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_bricks.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_bricks.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_bricks.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_bricks.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_bricks.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_bricks.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_bricks.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_tiles.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_tiles.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_tiles.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_tiles.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_tiles.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_tiles.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_tiles.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_tiles.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.deepslate_tiles.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.deepslate_tiles.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dispenser.dispense"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dispenser.dispense"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dispenser.fail"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dispenser.fail"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dispenser.launch"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dispenser.launch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.ambient_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.ambient_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.play"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.play"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.splash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.splash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dolphin.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dolphin.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.chest"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.chest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.donkey.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.donkey.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dripstone_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dripstone_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dripstone_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dripstone_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dripstone_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dripstone_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dripstone_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dripstone_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.dripstone_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.dripstone_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.land"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.drip_lava"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pointed_dripstone.drip_lava"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pointed_dripstone.drip_water"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.pointed_dripstone.drip_water",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const(
                    "block.pointed_dripstone.drip_lava_into_cauldron",
                ),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.pointed_dripstone.drip_lava_into_cauldron",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const(
                    "block.pointed_dripstone.drip_water_into_cauldron",
                ),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.pointed_dripstone.drip_water_into_cauldron",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.tilt_down"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.tilt_down"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.big_dripleaf.tilt_up"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.big_dripleaf.tilt_up"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.ambient_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.ambient_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.death_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.death_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.hurt_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.hurt_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.drowned.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.drowned.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.dye.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.dye.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.egg.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.egg.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.ambient_land"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.elder_guardian.ambient_land",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.curse"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.curse"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.death_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.death_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.elder_guardian.hurt_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.elder_guardian.hurt_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.elytra.flying"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.elytra.flying"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.enchantment_table.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.enchantment_table.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ender_chest.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ender_chest.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ender_chest.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ender_chest.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_dragon.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_dragon.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_dragon.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_dragon.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.dragon_fireball.explode"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.dragon_fireball.explode"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_dragon.flap"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_dragon.flap"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_dragon.growl"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_dragon.growl"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_dragon.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_dragon.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_dragon.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_dragon.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_eye.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_eye.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_eye.launch"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_eye.launch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.enderman.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.enderman.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.enderman.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.enderman.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.enderman.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.enderman.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.enderman.scream"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.enderman.scream"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.enderman.stare"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.enderman.stare"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.enderman.teleport"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.enderman.teleport"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.endermite.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.endermite.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.endermite.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.endermite.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.endermite.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.endermite.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.endermite.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.endermite.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ender_pearl.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ender_pearl.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.end_gateway.spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.end_gateway.spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.end_portal_frame.fill"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.end_portal_frame.fill"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.end_portal.spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.end_portal.spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.cast_spell"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.cast_spell"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker_fangs.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker_fangs.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.prepare_attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.prepare_attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.prepare_summon"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.prepare_summon"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.evoker.prepare_wololo"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.evoker.prepare_wololo"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.experience_bottle.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.experience_bottle.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.experience_orb.pickup"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.experience_orb.pickup"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.eyeblossom.open_long"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.eyeblossom.open_long"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.eyeblossom.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.eyeblossom.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.eyeblossom.close_long"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.eyeblossom.close_long"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.eyeblossom.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.eyeblossom.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.eyeblossom.idle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.eyeblossom.idle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fence_gate.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fence_gate.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fence_gate.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fence_gate.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.firecharge.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.firecharge.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.blast"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.firework_rocket.blast"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.blast_far"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.firework_rocket.blast_far"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.large_blast"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.firework_rocket.large_blast",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.large_blast_far"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.firework_rocket.large_blast_far",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.launch"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.firework_rocket.launch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.firework_rocket.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.twinkle"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.firework_rocket.twinkle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.firework_rocket.twinkle_far"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.firework_rocket.twinkle_far",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fire.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fire.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fire.extinguish"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fire.extinguish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fish.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fish.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fishing_bobber.retrieve"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fishing_bobber.retrieve"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fishing_bobber.splash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fishing_bobber.splash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fishing_bobber.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fishing_bobber.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.flintandsteel.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.flintandsteel.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.flowering_azalea.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.flowering_azalea.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.flowering_azalea.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.flowering_azalea.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.flowering_azalea.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.flowering_azalea.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.flowering_azalea.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.flowering_azalea.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.flowering_azalea.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.flowering_azalea.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.aggro"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.aggro"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.bite"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.bite"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.screech"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.screech"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.sleep"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.sleep"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.sniff"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.sniff"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.spit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.spit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.fox.teleport"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.fox.teleport"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_sand.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_sand.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_sand.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_sand.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_sand.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_sand.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_sand.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_sand.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_sand.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_sand.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_gravel.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_gravel.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_gravel.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_gravel.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_gravel.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_gravel.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_gravel.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_gravel.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.suspicious_gravel.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.suspicious_gravel.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.froglight.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.froglight.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.froglight.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.froglight.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.froglight.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.froglight.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.froglight.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.froglight.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.froglight.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.froglight.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.frogspawn.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.frogspawn.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.frogspawn.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.frogspawn.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.frogspawn.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.frogspawn.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.frogspawn.hatch"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.frogspawn.hatch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.frogspawn.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.frogspawn.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.frogspawn.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.frogspawn.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.lay_spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.lay_spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.long_jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.long_jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.frog.tongue"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.frog.tongue"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.roots.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.roots.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.roots.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.roots.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.roots.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.roots.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.roots.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.roots.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.roots.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.roots.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.furnace.fire_crackle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.furnace.fire_crackle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.big_fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.big_fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.burn"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.burn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.drink"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.drink"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.explode"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.explode"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.extinguish_fire"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.extinguish_fire"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.small_fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.small_fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.splash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.splash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.generic.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.generic.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ghast.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ghast.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ghast.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ghast.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ghast.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ghast.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ghast.scream"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ghast.scream"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ghast.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ghast.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ghast.warn"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ghast.warn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gilded_blackstone.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gilded_blackstone.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gilded_blackstone.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gilded_blackstone.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gilded_blackstone.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gilded_blackstone.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gilded_blackstone.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gilded_blackstone.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gilded_blackstone.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gilded_blackstone.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.glass.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.glass.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.glass.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.glass.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.glass.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.glass.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.glass.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.glass.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.glass.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.glass.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.glow_ink_sac.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.glow_ink_sac.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_item_frame.add_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_item_frame.add_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_item_frame.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_item_frame.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_item_frame.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_item_frame.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_item_frame.remove_item"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.glow_item_frame.remove_item",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_item_frame.rotate_item"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.glow_item_frame.rotate_item",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_squid.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_squid.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_squid.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_squid.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_squid.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_squid.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.glow_squid.squirt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.glow_squid.squirt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.long_jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.long_jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.milk"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.milk"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.prepare_ram"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.prepare_ram"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.ram_impact"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.ram_impact"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.horn_break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.horn_break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.long_jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.long_jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.milk"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.milk"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.prepare_ram"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.prepare_ram"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.screaming.ram_impact"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.screaming.ram_impact"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.goat.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.goat.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.grass.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.grass.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.grass.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.grass.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.grass.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.grass.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.grass.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.grass.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.grass.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.grass.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gravel.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gravel.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gravel.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gravel.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gravel.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gravel.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gravel.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gravel.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.gravel.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.gravel.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.grindstone.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.grindstone.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.growing_plant.crop"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.growing_plant.crop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.ambient_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.ambient_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.death_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.death_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.guardian.hurt_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.guardian.hurt_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_roots.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_roots.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_roots.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_roots.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_roots.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_roots.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_roots.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_roots.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_roots.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_roots.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_sign.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_sign.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_sign.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_sign.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_sign.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_sign.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_sign.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_sign.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_sign.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.hanging_sign.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.heavy_core.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.heavy_core.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.heavy_core.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.heavy_core.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.heavy_core.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.heavy_core.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.heavy_core.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.heavy_core.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.heavy_core.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.heavy_core.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_hanging_sign.step"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_hanging_sign.step",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_hanging_sign.break"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_hanging_sign.break",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_hanging_sign.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_hanging_sign.fall",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_hanging_sign.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_hanging_sign.hit",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_hanging_sign.place"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_hanging_sign.place",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_hanging_sign.step"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_hanging_sign.step",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_hanging_sign.break"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_hanging_sign.break",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_hanging_sign.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_hanging_sign.fall",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_hanging_sign.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_hanging_sign.hit",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.bamboo_wood_hanging_sign.place"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.bamboo_wood_hanging_sign.place",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.spawn_mob"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.spawn_mob"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.about_to_spawn_item"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.trial_spawner.about_to_spawn_item",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.spawn_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.spawn_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.spawn_item_begin"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.trial_spawner.spawn_item_begin",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.detect_player"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.detect_player"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.ominous_activate"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.trial_spawner.ominous_activate",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.ambient_ominous"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.trial_spawner.ambient_ominous",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.open_shutter"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.open_shutter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.close_shutter"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.close_shutter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.trial_spawner.eject_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.trial_spawner.eject_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.hoe.till"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.hoe.till"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.converted_to_zombified"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.hoglin.converted_to_zombified",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.retreat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.retreat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hoglin.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hoglin.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.honey_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.honey_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.honey_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.honey_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.honey_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.honey_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.honey_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.honey_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.honey_block.slide"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.honey_block.slide"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.honey_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.honey_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.honeycomb.wax_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.honeycomb.wax_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.honey_bottle.drink"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.honey_bottle.drink"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.0"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.0"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.1"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.1"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.2"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.2"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.3"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.3"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.4"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.4"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.5"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.5"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.6"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.6"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.goat_horn.sound.7"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.goat_horn.sound.7"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.armor"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.armor"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.breathe"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.breathe"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.gallop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.gallop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.saddle"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.saddle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.horse.step_wood"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.horse.step_wood"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hostile.big_fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hostile.big_fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hostile.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hostile.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hostile.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hostile.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hostile.small_fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hostile.small_fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hostile.splash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hostile.splash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.hostile.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.hostile.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.husk.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.husk.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.husk.converted_to_zombie"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.husk.converted_to_zombie"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.husk.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.husk.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.husk.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.husk.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.husk.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.husk.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.illusioner.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.cast_spell"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.illusioner.cast_spell"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.illusioner.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.illusioner.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.mirror_move"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.illusioner.mirror_move"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.prepare_blindness"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.illusioner.prepare_blindness",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.illusioner.prepare_mirror"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.illusioner.prepare_mirror"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.ink_sac.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.ink_sac.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.iron_door.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.iron_door.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.iron_door.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.iron_door.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.iron_golem.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.iron_golem.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.iron_golem.damage"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.iron_golem.damage"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.iron_golem.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.iron_golem.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.iron_golem.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.iron_golem.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.iron_golem.repair"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.iron_golem.repair"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.iron_golem.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.iron_golem.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.iron_trapdoor.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.iron_trapdoor.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.iron_trapdoor.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.iron_trapdoor.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item_frame.add_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item_frame.add_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item_frame.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item_frame.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item_frame.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item_frame.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item_frame.remove_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item_frame.remove_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item_frame.rotate_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item_frame.rotate_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.item.pickup"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.item.pickup"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ladder.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ladder.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ladder.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ladder.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ladder.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ladder.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ladder.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ladder.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.ladder.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.ladder.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lantern.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lantern.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lantern.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lantern.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lantern.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lantern.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lantern.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lantern.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lantern.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lantern.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.large_amethyst_bud.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.large_amethyst_bud.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.large_amethyst_bud.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.large_amethyst_bud.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lava.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lava.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lava.extinguish"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lava.extinguish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lava.pop"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lava.pop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.leash_knot.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.leash_knot.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.leash_knot.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.leash_knot.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lever.click"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lever.click"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.lightning_bolt.impact"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.lightning_bolt.impact"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.lightning_bolt.thunder"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.lightning_bolt.thunder"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.lingering_potion.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.lingering_potion.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.chest"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.chest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.spit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.spit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.llama.swag"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.llama.swag"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.death_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.death_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lodestone.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lodestone.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lodestone.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lodestone.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lodestone.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lodestone.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lodestone.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lodestone.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lodestone.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lodestone.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.lodestone_compass.lock"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.lodestone_compass.lock"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.mace.smash_air"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.mace.smash_air"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.mace.smash_ground"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.mace.smash_ground"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.mace.smash_ground_heavy"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.mace.smash_ground_heavy"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.hurt_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.hurt_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.squish"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.squish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.magma_cube.squish_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.magma_cube.squish_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mangrove_roots.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mangrove_roots.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mangrove_roots.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mangrove_roots.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mangrove_roots.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mangrove_roots.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mangrove_roots.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mangrove_roots.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mangrove_roots.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mangrove_roots.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.medium_amethyst_bud.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.medium_amethyst_bud.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.medium_amethyst_bud.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.medium_amethyst_bud.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.metal.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.metal.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.metal.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.metal.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal_pressure_plate.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.metal_pressure_plate.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal_pressure_plate.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.metal_pressure_plate.click_on",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.metal.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.metal.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.minecart.inside.underwater"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.minecart.inside.underwater"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.minecart.inside"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.minecart.inside"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.minecart.riding"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.minecart.riding"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mooshroom.convert"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mooshroom.convert"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mooshroom.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mooshroom.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mooshroom.milk"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mooshroom.milk"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mooshroom.suspicious_milk"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mooshroom.suspicious_milk"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mooshroom.shear"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mooshroom.shear"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss_carpet.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss_carpet.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss_carpet.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss_carpet.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss_carpet.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss_carpet.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss_carpet.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss_carpet.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss_carpet.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss_carpet.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pink_petals.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pink_petals.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pink_petals.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pink_petals.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pink_petals.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pink_petals.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pink_petals.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pink_petals.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pink_petals.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pink_petals.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.moss.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.moss.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud_bricks.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud_bricks.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud_bricks.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud_bricks.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud_bricks.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud_bricks.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud_bricks.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud_bricks.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.mud_bricks.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.mud_bricks.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.muddy_mangrove_roots.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.muddy_mangrove_roots.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.muddy_mangrove_roots.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.muddy_mangrove_roots.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.muddy_mangrove_roots.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.muddy_mangrove_roots.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.muddy_mangrove_roots.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.muddy_mangrove_roots.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.muddy_mangrove_roots.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.muddy_mangrove_roots.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.chest"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.chest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.mule.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.mule.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.creative"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.creative"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.credits"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.credits"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.5"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.5"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.11"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.11"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.13"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.13"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.blocks"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.blocks"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.cat"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.cat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.chirp"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.chirp"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.far"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.far"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.mall"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.mall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.mellohi"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.mellohi"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.pigstep"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.pigstep"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.stal"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.stal"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.strad"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.strad"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.wait"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.wait"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.ward"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.ward"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.otherside"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.otherside"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.relic"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.relic"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.creator"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.creator"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.creator_music_box"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.creator_music_box"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music_disc.precipice"),
                SoundEvent {
                    name: Identifier::vanilla_const("music_disc.precipice"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.dragon"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.dragon"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.end"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.end"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.game"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.game"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.menu"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.menu"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.nether.basalt_deltas"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.nether.basalt_deltas"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.nether.crimson_forest"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.nether.crimson_forest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.deep_dark"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.deep_dark"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.dripstone_caves"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.dripstone_caves"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.grove"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.grove"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.jagged_peaks"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.jagged_peaks"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.lush_caves"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.lush_caves"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.swamp"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.swamp"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.forest"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.forest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.old_growth_taiga"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.old_growth_taiga"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.meadow"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.meadow"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.cherry_grove"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.cherry_grove"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.nether.nether_wastes"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.nether.nether_wastes"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.frozen_peaks"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.frozen_peaks"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.snowy_slopes"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.snowy_slopes"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.nether.soul_sand_valley"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.nether.soul_sand_valley"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.stony_peaks"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.stony_peaks"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.nether.warped_forest"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.nether.warped_forest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.flower_forest"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.flower_forest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.desert"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.desert"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.badlands"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.badlands"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.jungle"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.jungle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.sparse_jungle"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.sparse_jungle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.overworld.bamboo_jungle"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.overworld.bamboo_jungle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("music.under_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("music.under_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_bricks.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_bricks.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_bricks.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_bricks.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_bricks.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_bricks.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_bricks.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_bricks.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_bricks.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_bricks.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wart.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wart.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.nether_wart.plant"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.nether_wart.plant"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_door.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood_door.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_door.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood_door.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_trapdoor.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood_trapdoor.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_trapdoor.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood_trapdoor.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_button.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_button.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_button.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood_button.click_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_pressure_plate.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_pressure_plate.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_pressure_plate.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_pressure_plate.click_on",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_fence_gate.close"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.nether_wood_fence_gate.close",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_wood_fence_gate.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_wood_fence_gate.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("intentionally_empty"),
                SoundEvent {
                    name: Identifier::vanilla_const("intentionally_empty"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.packed_mud.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.packed_mud.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.packed_mud.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.packed_mud.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.packed_mud.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.packed_mud.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.packed_mud.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.packed_mud.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.packed_mud.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.packed_mud.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stem.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stem.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stem.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stem.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stem.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stem.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stem.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stem.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stem.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stem.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nylium.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nylium.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nylium.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nylium.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nylium.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nylium.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nylium.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nylium.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nylium.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nylium.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_sprouts.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_sprouts.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_sprouts.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_sprouts.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_sprouts.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_sprouts.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_sprouts.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_sprouts.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_sprouts.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_sprouts.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fungus.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fungus.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fungus.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fungus.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fungus.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fungus.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fungus.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fungus.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.fungus.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.fungus.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.weeping_vines.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.weeping_vines.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.weeping_vines.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.weeping_vines.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.weeping_vines.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.weeping_vines.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.weeping_vines.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.weeping_vines.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.weeping_vines.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.weeping_vines.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wart_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wart_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wart_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wart_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wart_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wart_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wart_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wart_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wart_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wart_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherite_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherite_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherite_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherite_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherite_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherite_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherite_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherite_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherite_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherite_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherrack.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherrack.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherrack.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherrack.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherrack.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherrack.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherrack.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherrack.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.netherrack.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.netherrack.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.basedrum"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.basedrum"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.bass"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.bass"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.bell"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.bell"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.chime"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.chime"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.flute"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.flute"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.guitar"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.guitar"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.harp"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.harp"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.hat"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.hat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.pling"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.pling"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.snare"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.snare"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.xylophone"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.xylophone"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.iron_xylophone"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.iron_xylophone"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.cow_bell"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.cow_bell"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.didgeridoo"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.didgeridoo"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.bit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.bit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.banjo"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.banjo"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.imitate.zombie"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.imitate.zombie"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.imitate.skeleton"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.imitate.skeleton"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.imitate.creeper"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.imitate.creeper"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.imitate.ender_dragon"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.note_block.imitate.ender_dragon",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.imitate.wither_skeleton"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.note_block.imitate.wither_skeleton",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.note_block.imitate.piglin"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.note_block.imitate.piglin"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ocelot.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ocelot.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ocelot.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ocelot.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ocelot.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ocelot.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.ominous_bottle.dispose"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.ominous_bottle.dispose"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.painting.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.painting.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.painting.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.painting.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pale_hanging_moss.idle"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pale_hanging_moss.idle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.pre_sneeze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.pre_sneeze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.sneeze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.sneeze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.cant_breed"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.cant_breed"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.aggressive_ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.aggressive_ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.worried_ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.worried_ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.panda.bite"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.panda.bite"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.fly"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.fly"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.blaze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.blaze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.bogged"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.bogged"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.breeze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.breeze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.creaking"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.creaking"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.creeper"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.creeper"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.drowned"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.drowned"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.elder_guardian"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.parrot.imitate.elder_guardian",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.ender_dragon"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.parrot.imitate.ender_dragon",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.endermite"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.endermite"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.evoker"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.evoker"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.ghast"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.ghast"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.guardian"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.guardian"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.hoglin"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.hoglin"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.husk"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.husk"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.illusioner"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.illusioner"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.magma_cube"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.magma_cube"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.phantom"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.phantom"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.piglin"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.piglin"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.piglin_brute"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.parrot.imitate.piglin_brute",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.pillager"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.pillager"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.ravager"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.ravager"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.shulker"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.shulker"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.silverfish"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.silverfish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.skeleton"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.skeleton"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.slime"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.slime"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.spider"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.spider"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.stray"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.stray"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.vex"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.vex"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.vindicator"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.vindicator"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.warden"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.warden"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.witch"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.witch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.wither"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.wither"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.wither_skeleton"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.parrot.imitate.wither_skeleton",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.zoglin"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.zoglin"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.zombie"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.imitate.zombie"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.imitate.zombie_villager"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.parrot.imitate.zombie_villager",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.parrot.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.parrot.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.phantom.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.phantom.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.phantom.bite"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.phantom.bite"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.phantom.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.phantom.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.phantom.flap"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.phantom.flap"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.phantom.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.phantom.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.phantom.swoop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.phantom.swoop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pig.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pig.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pig.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pig.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pig.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pig.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pig.saddle"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pig.saddle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pig.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pig.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.admiring_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.admiring_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.jealous"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.jealous"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.retreat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.retreat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin.converted_to_zombified"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.piglin.converted_to_zombified",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin_brute.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin_brute.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin_brute.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin_brute.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin_brute.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin_brute.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin_brute.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin_brute.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin_brute.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.piglin_brute.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.piglin_brute.converted_to_zombified"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.piglin_brute.converted_to_zombified",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pillager.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pillager.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pillager.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pillager.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pillager.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pillager.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.pillager.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.pillager.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.piston.contract"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.piston.contract"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.piston.extend"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.piston.extend"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.attack.crit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.attack.crit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.attack.knockback"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.attack.knockback"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.attack.nodamage"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.attack.nodamage"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.attack.strong"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.attack.strong"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.attack.sweep"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.attack.sweep"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.attack.weak"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.attack.weak"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.big_fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.big_fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.breath"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.breath"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.burp"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.burp"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.hurt_drown"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.hurt_drown"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.hurt_freeze"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.hurt_freeze"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.hurt_on_fire"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.hurt_on_fire"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.hurt_sweet_berry_bush"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.player.hurt_sweet_berry_bush",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.levelup"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.levelup"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.small_fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.small_fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.splash"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.splash"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.splash.high_speed"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.splash.high_speed"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.player.teleport"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.player.teleport"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.polar_bear.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.polar_bear.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.polar_bear.ambient_baby"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.polar_bear.ambient_baby"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.polar_bear.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.polar_bear.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.polar_bear.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.polar_bear.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.polar_bear.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.polar_bear.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.polar_bear.warning"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.polar_bear.warning"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_deepslate.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_deepslate.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_deepslate.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_deepslate.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_deepslate.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_deepslate.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_deepslate.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_deepslate.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_deepslate.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_deepslate.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.portal.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.portal.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.portal.travel"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.portal.travel"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.portal.trigger"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.portal.trigger"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.powder_snow.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.powder_snow.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.powder_snow.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.powder_snow.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.powder_snow.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.powder_snow.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.powder_snow.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.powder_snow.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.powder_snow.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.powder_snow.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.blow_out"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.blow_out"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.blow_up"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.blow_up"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.puffer_fish.sting"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.puffer_fish.sting"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.pumpkin.carve"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.pumpkin.carve"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.rabbit.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.rabbit.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.rabbit.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.rabbit.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.rabbit.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.rabbit.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.rabbit.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.rabbit.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.rabbit.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.rabbit.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("event.raid.horn"),
                SoundEvent {
                    name: Identifier::vanilla_const("event.raid.horn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.stunned"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.stunned"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.ravager.roar"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.ravager.roar"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_gold_ore.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_gold_ore.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_gold_ore.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_gold_ore.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_gold_ore.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_gold_ore.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_gold_ore.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_gold_ore.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_gold_ore.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_gold_ore.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_ore.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_ore.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_ore.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_ore.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_ore.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_ore.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_ore.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_ore.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.nether_ore.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.nether_ore.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.redstone_torch.burnout"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.redstone_torch.burnout"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.respawn_anchor.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.respawn_anchor.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.respawn_anchor.charge"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.respawn_anchor.charge"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.respawn_anchor.deplete"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.respawn_anchor.deplete"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.respawn_anchor.set_spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.respawn_anchor.set_spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.rooted_dirt.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.rooted_dirt.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.rooted_dirt.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.rooted_dirt.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.rooted_dirt.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.rooted_dirt.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.rooted_dirt.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.rooted_dirt.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.rooted_dirt.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.rooted_dirt.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.salmon.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.salmon.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.salmon.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.salmon.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.salmon.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.salmon.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.salmon.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.salmon.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sand.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sand.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sand.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sand.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sand.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sand.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sand.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sand.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sand.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sand.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.scaffolding.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.scaffolding.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.scaffolding.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.scaffolding.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.scaffolding.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.scaffolding.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.scaffolding.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.scaffolding.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.scaffolding.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.scaffolding.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.spread"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.spread"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.charge"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.charge"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_catalyst.bloom"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_catalyst.bloom"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_catalyst.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_catalyst.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_catalyst.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_catalyst.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_catalyst.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_catalyst.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_catalyst.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_catalyst.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_catalyst.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_catalyst.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.clicking"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.clicking"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.clicking_stop"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.clicking_stop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_sensor.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_sensor.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_shrieker.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_shrieker.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_shrieker.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_shrieker.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_shrieker.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_shrieker.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_shrieker.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_shrieker.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_shrieker.shriek"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_shrieker.shriek"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_shrieker.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_shrieker.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_vein.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_vein.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_vein.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_vein.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_vein.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_vein.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_vein.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_vein.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sculk_vein.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sculk_vein.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sheep.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sheep.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sheep.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sheep.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sheep.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sheep.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sheep.shear"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sheep.shear"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sheep.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sheep.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.shield.block"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.shield.block"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.shield.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.shield.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shroomlight.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shroomlight.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shroomlight.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shroomlight.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shroomlight.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shroomlight.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shroomlight.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shroomlight.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shroomlight.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shroomlight.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.shovel.flatten"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.shovel.flatten"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shulker_box.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shulker_box.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.shulker_box.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.shulker_box.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker_bullet.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker_bullet.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker_bullet.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker_bullet.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.hurt_closed"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.hurt_closed"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.shulker.teleport"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.shulker.teleport"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.silverfish.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.silverfish.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.silverfish.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.silverfish.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.silverfish.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.silverfish.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.silverfish.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.silverfish.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton.converted_to_stray"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.skeleton.converted_to_stray",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton_horse.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton_horse.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton_horse.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton_horse.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.ambient_water"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.skeleton_horse.ambient_water",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.gallop_water"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.skeleton_horse.gallop_water",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.jump_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton_horse.jump_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton_horse.step_water"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton_horse.step_water"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.skeleton.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.skeleton.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.jump"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.jump"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.squish"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.squish"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.slime_block.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.slime_block.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.slime_block.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.slime_block.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.slime_block.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.slime_block.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.slime_block.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.slime_block.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.slime_block.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.slime_block.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_amethyst_bud.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_amethyst_bud.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_amethyst_bud.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_amethyst_bud.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_dripleaf.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_dripleaf.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_dripleaf.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_dripleaf.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_dripleaf.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_dripleaf.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_dripleaf.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_dripleaf.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.small_dripleaf.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.small_dripleaf.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_sand.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_sand.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_sand.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_sand.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_sand.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_sand.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_sand.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_sand.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_sand.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_sand.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_soil.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_soil.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_soil.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_soil.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_soil.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_soil.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_soil.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_soil.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.soul_soil.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.soul_soil.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("particle.soul_escape"),
                SoundEvent {
                    name: Identifier::vanilla_const("particle.soul_escape"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spawner.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spawner.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spawner.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spawner.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spawner.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spawner.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spawner.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spawner.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spawner.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spawner.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin_bricks.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin_bricks.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin_bricks.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin_bricks.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin_bricks.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin_bricks.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin_bricks.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin_bricks.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.resin_bricks.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.resin_bricks.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spore_blossom.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spore_blossom.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spore_blossom.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spore_blossom.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spore_blossom.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spore_blossom.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spore_blossom.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spore_blossom.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.spore_blossom.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.spore_blossom.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.happy"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.happy"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.retreat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.retreat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.step_lava"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.step_lava"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.strider.saddle"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.strider.saddle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.death_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.death_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.hurt_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.hurt_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.jump_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.jump_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.slime.squish_small"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.slime.squish_small"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.smithing_table.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.smithing_table.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.smoker.smoke"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.smoker.smoke"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.eat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.eat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.idle"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.idle"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.drop_seed"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.drop_seed"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.scenting"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.scenting"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.sniffing"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.sniffing"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.searching"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.searching"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.digging"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.digging"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.digging_stop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.digging_stop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.sniffer.happy"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.sniffer.happy"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sniffer_egg.plop"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sniffer_egg.plop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sniffer_egg.crack"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sniffer_egg.crack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sniffer_egg.hatch"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sniffer_egg.hatch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.snowball.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.snowball.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.snow.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.snow.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.snow.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.snow.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.snow_golem.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.snow_golem.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.snow_golem.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.snow_golem.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.snow_golem.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.snow_golem.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.snow_golem.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.snow_golem.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.snow_golem.shear"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.snow_golem.shear"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.snow.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.snow.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.snow.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.snow.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.snow.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.snow.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.spider.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.spider.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.spider.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.spider.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.spider.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.spider.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.spider.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.spider.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.splash_potion.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.splash_potion.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.splash_potion.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.splash_potion.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sponge.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sponge.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sponge.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sponge.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sponge.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sponge.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sponge.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sponge.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sponge.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sponge.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sponge.absorb"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sponge.absorb"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.spyglass.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.spyglass.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.spyglass.stop_using"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.spyglass.stop_using"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.squid.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.squid.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.squid.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.squid.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.squid.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.squid.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.squid.squirt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.squid.squirt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone_button.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone_button.click_off"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone_button.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone_button.click_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone_pressure_plate.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.stone_pressure_plate.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone_pressure_plate.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.stone_pressure_plate.click_on",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.stone.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.stone.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.stray.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.stray.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.stray.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.stray.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.stray.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.stray.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.stray.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.stray.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sweet_berry_bush.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sweet_berry_bush.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sweet_berry_bush.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sweet_berry_bush.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sweet_berry_bush.pick_berries"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.sweet_berry_bush.pick_berries",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tadpole.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tadpole.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tadpole.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tadpole.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tadpole.grow_up"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tadpole.grow_up"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tadpole.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tadpole.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("enchant.thorns.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("enchant.thorns.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tnt.primed"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tnt.primed"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.totem.use"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.totem.use"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.hit_ground"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.hit_ground"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.return"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.return"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.riptide_1"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.riptide_1"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.riptide_2"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.riptide_2"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.riptide_3"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.riptide_3"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.trident.thunder"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.trident.thunder"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tripwire.attach"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tripwire.attach"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tripwire.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tripwire.click_off"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tripwire.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tripwire.click_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tripwire.detach"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tripwire.detach"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tropical_fish.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tropical_fish.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tropical_fish.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tropical_fish.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tropical_fish.flop"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tropical_fish.flop"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.tropical_fish.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.tropical_fish.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff_bricks.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff_bricks.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff_bricks.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff_bricks.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff_bricks.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff_bricks.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff_bricks.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff_bricks.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.tuff_bricks.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.tuff_bricks.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_tuff.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_tuff.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_tuff.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_tuff.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_tuff.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_tuff.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_tuff.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_tuff.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.polished_tuff.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.polished_tuff.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.ambient_land"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.ambient_land"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.death_baby"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.death_baby"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.egg_break"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.egg_break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.egg_crack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.egg_crack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.egg_hatch"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.egg_hatch"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.hurt_baby"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.hurt_baby"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.lay_egg"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.lay_egg"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.shamble"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.shamble"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.shamble_baby"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.shamble_baby"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.turtle.swim"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.turtle.swim"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.button.click"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.button.click"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.loom.select_pattern"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.loom.select_pattern"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.loom.take_result"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.loom.take_result"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.cartography_table.take_result"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.cartography_table.take_result"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.stonecutter.take_result"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.stonecutter.take_result"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.stonecutter.select_recipe"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.stonecutter.select_recipe"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.toast.challenge_complete"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.toast.challenge_complete"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.toast.in"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.toast.in"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("ui.toast.out"),
                SoundEvent {
                    name: Identifier::vanilla_const("ui.toast.out"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.activate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.activate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.close_shutter"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.close_shutter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.deactivate"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.deactivate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.eject_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.eject_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.reject_rewarded_player"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.vault.reject_rewarded_player",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.insert_item"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.insert_item"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.insert_item_fail"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.insert_item_fail"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.open_shutter"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.open_shutter"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vault.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vault.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vex.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vex.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vex.charge"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vex.charge"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vex.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vex.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vex.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vex.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.no"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.no"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.trade"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.trade"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.yes"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.yes"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_armorer"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_armorer"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_butcher"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_butcher"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_cartographer"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_cartographer"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_cleric"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_cleric"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_farmer"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_farmer"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_fisherman"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_fisherman"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_fletcher"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_fletcher"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_leatherworker"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.villager.work_leatherworker",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_librarian"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_librarian"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_mason"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_mason"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_shepherd"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_shepherd"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_toolsmith"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_toolsmith"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.villager.work_weaponsmith"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.villager.work_weaponsmith"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vindicator.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vindicator.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vindicator.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vindicator.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vindicator.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vindicator.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.vindicator.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.vindicator.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vine.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vine.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vine.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vine.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vine.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vine.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vine.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vine.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.vine.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.vine.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.lily_pad.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.lily_pad.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wandering_trader.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wandering_trader.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.disappeared"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.wandering_trader.disappeared",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.drink_milk"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.wandering_trader.drink_milk",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.drink_potion"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.wandering_trader.drink_potion",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wandering_trader.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.no"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wandering_trader.no"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.reappeared"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.wandering_trader.reappeared",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.trade"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wandering_trader.trade"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wandering_trader.yes"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wandering_trader.yes"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.agitated"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.agitated"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.attack_impact"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.attack_impact"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.dig"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.dig"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.emerge"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.emerge"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.heartbeat"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.heartbeat"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.listening"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.listening"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.listening_angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.listening_angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.nearby_close"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.nearby_close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.nearby_closer"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.nearby_closer"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.nearby_closest"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.nearby_closest"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.roar"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.roar"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.sniff"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.sniff"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.sonic_boom"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.sonic_boom"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.sonic_charge"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.sonic_charge"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.warden.tendril_clicks"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.warden.tendril_clicks"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.hanging_sign.waxed_interact_fail"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.hanging_sign.waxed_interact_fail",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.sign.waxed_interact_fail"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.sign.waxed_interact_fail"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.water.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.water.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weather.rain"),
                SoundEvent {
                    name: Identifier::vanilla_const("weather.rain"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("weather.rain.above"),
                SoundEvent {
                    name: Identifier::vanilla_const("weather.rain.above"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_grass.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_grass.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_grass.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_grass.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_grass.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_grass.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_grass.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_grass.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_grass.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_grass.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_sponge.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_sponge.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_sponge.dries"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_sponge.dries"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_sponge.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_sponge.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_sponge.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_sponge.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_sponge.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_sponge.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wet_sponge.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wet_sponge.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wind_charge.wind_burst"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wind_charge.wind_burst"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wind_charge.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wind_charge.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.witch.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.witch.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.witch.celebrate"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.witch.celebrate"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.witch.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.witch.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.witch.drink"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.witch.drink"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.witch.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.witch.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.witch.throw"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.witch.throw"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither.break_block"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither.break_block"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither.shoot"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither.shoot"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither_skeleton.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither_skeleton.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither_skeleton.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither_skeleton.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither_skeleton.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither_skeleton.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither_skeleton.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither_skeleton.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wither.spawn"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wither.spawn"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.wolf_armor.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.wolf_armor.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.wolf_armor.crack"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.wolf_armor.crack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.wolf_armor.damage"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.wolf_armor.damage"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("item.wolf_armor.repair"),
                SoundEvent {
                    name: Identifier::vanilla_const("item.wolf_armor.repair"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.growl"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.growl"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.howl"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.howl"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.pant"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.pant"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.shake"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.shake"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.wolf.whine"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.wolf.whine"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_door.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wooden_door.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_door.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wooden_door.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_trapdoor.close"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wooden_trapdoor.close"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_trapdoor.open"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wooden_trapdoor.open"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_button.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wooden_button.click_off"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_button.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wooden_button.click_on"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_pressure_plate.click_off"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.wooden_pressure_plate.click_off",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wooden_pressure_plate.click_on"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "block.wooden_pressure_plate.click_on",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wood.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wood.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wood.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wood.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wood.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wood.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wood.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wood.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wood.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wood.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wool.break"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wool.break"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wool.fall"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wool.fall"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wool.hit"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wool.hit"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wool.place"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wool.place"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block.wool.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("block.wool.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zoglin.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zoglin.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zoglin.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zoglin.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zoglin.attack"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zoglin.attack"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zoglin.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zoglin.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zoglin.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zoglin.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zoglin.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zoglin.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.attack_wooden_door"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.attack_wooden_door"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.attack_iron_door"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.attack_iron_door"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.break_wooden_door"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.break_wooden_door"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.converted_to_drowned"),
                SoundEvent {
                    name: Identifier::vanilla_const(
                        "entity.zombie.converted_to_drowned",
                    ),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.destroy_egg"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.destroy_egg"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_horse.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_horse.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_horse.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_horse.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_horse.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_horse.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.infect"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.infect"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombified_piglin.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombified_piglin.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombified_piglin.angry"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombified_piglin.angry"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombified_piglin.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombified_piglin.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombified_piglin.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombified_piglin.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_villager.ambient"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_villager.ambient"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_villager.converted"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_villager.converted"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_villager.cure"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_villager.cure"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_villager.death"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_villager.death"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_villager.hurt"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_villager.hurt"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity.zombie_villager.step"),
                SoundEvent {
                    name: Identifier::vanilla_const("entity.zombie_villager.step"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("event.mob_effect.bad_omen"),
                SoundEvent {
                    name: Identifier::vanilla_const("event.mob_effect.bad_omen"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("event.mob_effect.trial_omen"),
                SoundEvent {
                    name: Identifier::vanilla_const("event.mob_effect.trial_omen"),
                    fixed_range: None,
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("event.mob_effect.raid_omen"),
                SoundEvent {
                    name: Identifier::vanilla_const("event.mob_effect.raid_omen"),
                    fixed_range: None,
                },
            );
        registry
    }
}
