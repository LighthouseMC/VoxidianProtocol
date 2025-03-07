use crate::packet::*;
impl AttributeType {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<AttributeType> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::vanilla_const("armor"),
                AttributeType {
                    id: Identifier::vanilla_const("armor"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("armor_toughness"),
                AttributeType {
                    id: Identifier::vanilla_const("armor_toughness"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("attack_damage"),
                AttributeType {
                    id: Identifier::vanilla_const("attack_damage"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("attack_knockback"),
                AttributeType {
                    id: Identifier::vanilla_const("attack_knockback"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("attack_speed"),
                AttributeType {
                    id: Identifier::vanilla_const("attack_speed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block_break_speed"),
                AttributeType {
                    id: Identifier::vanilla_const("block_break_speed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("block_interaction_range"),
                AttributeType {
                    id: Identifier::vanilla_const("block_interaction_range"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("burning_time"),
                AttributeType {
                    id: Identifier::vanilla_const("burning_time"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("explosion_knockback_resistance"),
                AttributeType {
                    id: Identifier::vanilla_const("explosion_knockback_resistance"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("entity_interaction_range"),
                AttributeType {
                    id: Identifier::vanilla_const("entity_interaction_range"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("fall_damage_multiplier"),
                AttributeType {
                    id: Identifier::vanilla_const("fall_damage_multiplier"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("flying_speed"),
                AttributeType {
                    id: Identifier::vanilla_const("flying_speed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("follow_range"),
                AttributeType {
                    id: Identifier::vanilla_const("follow_range"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("gravity"),
                AttributeType {
                    id: Identifier::vanilla_const("gravity"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("jump_strength"),
                AttributeType {
                    id: Identifier::vanilla_const("jump_strength"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("knockback_resistance"),
                AttributeType {
                    id: Identifier::vanilla_const("knockback_resistance"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("luck"),
                AttributeType {
                    id: Identifier::vanilla_const("luck"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("max_absorption"),
                AttributeType {
                    id: Identifier::vanilla_const("max_absorption"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("max_health"),
                AttributeType {
                    id: Identifier::vanilla_const("max_health"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("mining_efficiency"),
                AttributeType {
                    id: Identifier::vanilla_const("mining_efficiency"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("movement_efficiency"),
                AttributeType {
                    id: Identifier::vanilla_const("movement_efficiency"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("movement_speed"),
                AttributeType {
                    id: Identifier::vanilla_const("movement_speed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("oxygen_bonus"),
                AttributeType {
                    id: Identifier::vanilla_const("oxygen_bonus"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("safe_fall_distance"),
                AttributeType {
                    id: Identifier::vanilla_const("safe_fall_distance"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("scale"),
                AttributeType {
                    id: Identifier::vanilla_const("scale"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sneaking_speed"),
                AttributeType {
                    id: Identifier::vanilla_const("sneaking_speed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("spawn_reinforcements"),
                AttributeType {
                    id: Identifier::vanilla_const("spawn_reinforcements"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("step_height"),
                AttributeType {
                    id: Identifier::vanilla_const("step_height"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("submerged_mining_speed"),
                AttributeType {
                    id: Identifier::vanilla_const("submerged_mining_speed"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("sweeping_damage_ratio"),
                AttributeType {
                    id: Identifier::vanilla_const("sweeping_damage_ratio"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("tempt_range"),
                AttributeType {
                    id: Identifier::vanilla_const("tempt_range"),
                },
            );
        registry
            .insert(
                Identifier::vanilla_const("water_movement_efficiency"),
                AttributeType {
                    id: Identifier::vanilla_const("water_movement_efficiency"),
                },
            );
        registry
    }
}
