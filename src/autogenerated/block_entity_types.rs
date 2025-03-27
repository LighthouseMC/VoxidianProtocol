use crate::packet::s2c::play::ChunkBlockEntity;
use crate::packet::*;
impl ChunkBlockEntity {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn block_entity_type_registry() -> Registry<VarInt> {
        let mut registry = Registry::new();
        registry.insert(Identifier::vanilla_const("furnace"), VarInt::new(0i32));
        registry.insert(Identifier::vanilla_const("chest"), VarInt::new(1i32));
        registry.insert(Identifier::vanilla_const("trapped_chest"), VarInt::new(2i32));
        registry.insert(Identifier::vanilla_const("ender_chest"), VarInt::new(3i32));
        registry.insert(Identifier::vanilla_const("jukebox"), VarInt::new(4i32));
        registry.insert(Identifier::vanilla_const("dispenser"), VarInt::new(5i32));
        registry.insert(Identifier::vanilla_const("dropper"), VarInt::new(6i32));
        registry.insert(Identifier::vanilla_const("sign"), VarInt::new(7i32));
        registry.insert(Identifier::vanilla_const("hanging_sign"), VarInt::new(8i32));
        registry.insert(Identifier::vanilla_const("mob_spawner"), VarInt::new(9i32));
        registry.insert(Identifier::vanilla_const("creaking_heart"), VarInt::new(10i32));
        registry.insert(Identifier::vanilla_const("piston"), VarInt::new(11i32));
        registry.insert(Identifier::vanilla_const("brewing_stand"), VarInt::new(12i32));
        registry
            .insert(Identifier::vanilla_const("enchanting_table"), VarInt::new(13i32));
        registry.insert(Identifier::vanilla_const("end_portal"), VarInt::new(14i32));
        registry.insert(Identifier::vanilla_const("beacon"), VarInt::new(15i32));
        registry.insert(Identifier::vanilla_const("skull"), VarInt::new(16i32));
        registry
            .insert(Identifier::vanilla_const("daylight_detector"), VarInt::new(17i32));
        registry.insert(Identifier::vanilla_const("hopper"), VarInt::new(18i32));
        registry.insert(Identifier::vanilla_const("comparator"), VarInt::new(19i32));
        registry.insert(Identifier::vanilla_const("banner"), VarInt::new(20i32));
        registry
            .insert(Identifier::vanilla_const("structure_block"), VarInt::new(21i32));
        registry.insert(Identifier::vanilla_const("end_gateway"), VarInt::new(22i32));
        registry.insert(Identifier::vanilla_const("command_block"), VarInt::new(23i32));
        registry.insert(Identifier::vanilla_const("shulker_box"), VarInt::new(24i32));
        registry.insert(Identifier::vanilla_const("bed"), VarInt::new(25i32));
        registry.insert(Identifier::vanilla_const("conduit"), VarInt::new(26i32));
        registry.insert(Identifier::vanilla_const("barrel"), VarInt::new(27i32));
        registry.insert(Identifier::vanilla_const("smoker"), VarInt::new(28i32));
        registry.insert(Identifier::vanilla_const("blast_furnace"), VarInt::new(29i32));
        registry.insert(Identifier::vanilla_const("lectern"), VarInt::new(30i32));
        registry.insert(Identifier::vanilla_const("bell"), VarInt::new(31i32));
        registry.insert(Identifier::vanilla_const("jigsaw"), VarInt::new(32i32));
        registry.insert(Identifier::vanilla_const("campfire"), VarInt::new(33i32));
        registry.insert(Identifier::vanilla_const("beehive"), VarInt::new(34i32));
        registry.insert(Identifier::vanilla_const("sculk_sensor"), VarInt::new(35i32));
        registry
            .insert(
                Identifier::vanilla_const("calibrated_sculk_sensor"),
                VarInt::new(36i32),
            );
        registry.insert(Identifier::vanilla_const("sculk_catalyst"), VarInt::new(37i32));
        registry.insert(Identifier::vanilla_const("sculk_shrieker"), VarInt::new(38i32));
        registry
            .insert(Identifier::vanilla_const("chiseled_bookshelf"), VarInt::new(39i32));
        registry
            .insert(Identifier::vanilla_const("brushable_block"), VarInt::new(40i32));
        registry.insert(Identifier::vanilla_const("decorated_pot"), VarInt::new(41i32));
        registry.insert(Identifier::vanilla_const("crafter"), VarInt::new(42i32));
        registry.insert(Identifier::vanilla_const("trial_spawner"), VarInt::new(43i32));
        registry.insert(Identifier::vanilla_const("vault"), VarInt::new(44i32));
        registry.insert(Identifier::vanilla_const("test_block"), VarInt::new(45i32));
        registry
            .insert(
                Identifier::vanilla_const("test_instance_block"),
                VarInt::new(46i32),
            );
        registry
    }
}
