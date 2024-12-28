use voxidian_protocol_macros::import_damage_type_registry_from_file;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct DamageType {
    pub message            : String,
    pub scaling            : DamageDifficultyScaling,
    pub exhaustion         : f32,
    pub effects            : Option<DamageEffects>,
    pub death_message_type : Option<DeathMessageType>
}

impl DamageType {
    pub fn vanilla_registry() -> Registry<DamageType> {
        import_damage_type_registry_from_file!("generated/damage_types.json")
    }
}

impl RegValue for DamageType {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("damage_type");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("message_id" , NbtElement::String (self.message.clone()              ));
        nbt.insert("scaling"    , NbtElement::String (self.scaling.as_str().to_string() ));
        nbt.insert("exhaustion" , NbtElement::Float  (self.exhaustion                   ));
        if let Some(effects) = self.effects {
            nbt.insert("effects", NbtElement::String(effects.as_str().to_string()));
        }
        if let Some(death_message_type) = self.death_message_type {
            nbt.insert("death_message_type", NbtElement::String(death_message_type.as_str().to_string()));
        }
        Some(Nbt { name : String::new(), root : nbt })
    }

}


#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum DamageDifficultyScaling {
    Never,
    NonLivingPlayer,
    Always
}
impl DamageDifficultyScaling { fn as_str(&self) -> &'static str { match (self) {
    Self::Never           => "never",
    Self::NonLivingPlayer => "when_caused_by_living_non_player",
    Self::Always          => "always"
} } }


#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum DamageEffects {
    Hurt,
    Thorns,
    Drowning,
    Burning,
    Poking,
    Freezing
}
impl DamageEffects { fn as_str(&self) -> &'static str { match (self) {
    Self::Hurt     => "hurt",
    Self::Thorns   => "thorns",
    Self::Drowning => "drowning",
    Self::Burning  => "burning",
    Self::Poking   => "poking",
    Self::Freezing => "freezing"
} } }


#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum DeathMessageType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign
}
impl DeathMessageType { fn as_str(&self) -> &'static str { match (self) {
    Self::Default               => "default",
    Self::FallVariants          => "fall_variants",
    Self::IntentionalGameDesign => "intentional_game_design"
} } }

#[cfg(test)]
mod tests {
    use crate::value::Identifier;

    use super::DamageType;

    #[test]
    pub fn test_vanilla_registry() {
        let rg = DamageType::vanilla_registry();

        assert!(rg.get(&Identifier::new("minecraft", "in_fire")).is_some());
    }
}