use crate::packet::*;
impl DamageType {
    #[allow(dead_code)]
    #[allow(redundant_semicolons)]
    pub fn vanilla_registry() -> Registry<DamageType> {
        let mut registry = Registry::new();
        registry
            .insert(
                Identifier::new("minecraft", "dry_out"),
                DamageType {
                    message_id: String::from("dryout"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "freeze"),
                DamageType {
                    message_id: String::from("freeze"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: Some(DamageEffects::Freezing),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "in_wall"),
                DamageType {
                    message_id: String::from("inWall"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "wither_skull"),
                DamageType {
                    message_id: String::from("witherSkull"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "mob_projectile"),
                DamageType {
                    message_id: String::from("mob"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "falling_anvil"),
                DamageType {
                    message_id: String::from("anvil"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "hot_floor"),
                DamageType {
                    message_id: String::from("hotFloor"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "trident"),
                DamageType {
                    message_id: String::from("trident"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "player_explosion"),
                DamageType {
                    message_id: String::from("explosion.player"),
                    scaling: DamageDifficultyScaling::Always,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "generic"),
                DamageType {
                    message_id: String::from("generic"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "falling_stalactite"),
                DamageType {
                    message_id: String::from("fallingStalactite"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "lava"),
                DamageType {
                    message_id: String::from("lava"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "wind_charge"),
                DamageType {
                    message_id: String::from("mob"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "falling_block"),
                DamageType {
                    message_id: String::from("fallingBlock"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "cactus"),
                DamageType {
                    message_id: String::from("cactus"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "thrown"),
                DamageType {
                    message_id: String::from("thrown"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "indirect_magic"),
                DamageType {
                    message_id: String::from("indirectMagic"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "outside_border"),
                DamageType {
                    message_id: String::from("outsideBorder"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "out_of_world"),
                DamageType {
                    message_id: String::from("outOfWorld"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "campfire"),
                DamageType {
                    message_id: String::from("inFire"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "dragon_breath"),
                DamageType {
                    message_id: String::from("dragonBreath"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "cramming"),
                DamageType {
                    message_id: String::from("cramming"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "starve"),
                DamageType {
                    message_id: String::from("starve"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "in_fire"),
                DamageType {
                    message_id: String::from("inFire"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "bad_respawn_point"),
                DamageType {
                    message_id: String::from("badRespawnPoint"),
                    scaling: DamageDifficultyScaling::Always,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: Some(DeathMessageType::IntentionalGameDesign),
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "on_fire"),
                DamageType {
                    message_id: String::from("onFire"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "explosion"),
                DamageType {
                    message_id: String::from("explosion"),
                    scaling: DamageDifficultyScaling::Always,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "unattributed_fireball"),
                DamageType {
                    message_id: String::from("onFire"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "thorns"),
                DamageType {
                    message_id: String::from("thorns"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Thorns),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "arrow"),
                DamageType {
                    message_id: String::from("arrow"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "fireworks"),
                DamageType {
                    message_id: String::from("fireworks"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "wither"),
                DamageType {
                    message_id: String::from("wither"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "stalagmite"),
                DamageType {
                    message_id: String::from("stalagmite"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "player_attack"),
                DamageType {
                    message_id: String::from("player"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "mace_smash"),
                DamageType {
                    message_id: String::from("mace_smash"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "fly_into_wall"),
                DamageType {
                    message_id: String::from("flyIntoWall"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "magic"),
                DamageType {
                    message_id: String::from("magic"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "generic_kill"),
                DamageType {
                    message_id: String::from("genericKill"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "fireball"),
                DamageType {
                    message_id: String::from("fireball"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Burning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "fall"),
                DamageType {
                    message_id: String::from("fall"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: Some(DeathMessageType::FallVariants),
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "lightning_bolt"),
                DamageType {
                    message_id: String::from("lightningBolt"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "mob_attack_no_aggro"),
                DamageType {
                    message_id: String::from("mob"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "mob_attack"),
                DamageType {
                    message_id: String::from("mob"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "sonic_boom"),
                DamageType {
                    message_id: String::from("sonic_boom"),
                    scaling: DamageDifficultyScaling::Always,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "drown"),
                DamageType {
                    message_id: String::from("drown"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: Some(DamageEffects::Drowning),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "ender_pearl"),
                DamageType {
                    message_id: String::from("fall"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0f32,
                    effects: None,
                    death_message_type: Some(DeathMessageType::FallVariants),
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "spit"),
                DamageType {
                    message_id: String::from("mob"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "sweet_berry_bush"),
                DamageType {
                    message_id: String::from("sweetBerryBush"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: Some(DamageEffects::Poking),
                    death_message_type: None,
                },
            );
        registry
            .insert(
                Identifier::new("minecraft", "sting"),
                DamageType {
                    message_id: String::from("sting"),
                    scaling: DamageDifficultyScaling::NonLivingPlayer,
                    exhaustion: 0.1f32,
                    effects: None,
                    death_message_type: None,
                },
            );
        registry
    }
}
