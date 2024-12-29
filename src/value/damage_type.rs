use super::*;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DamageType {
    pub message_id         : String,
    pub scaling            : DamageDifficultyScaling,
    pub exhaustion         : f32,
    pub effects            : Option<DamageEffects>,
    pub death_message_type : Option<DeathMessageType>
}

impl RegValue for DamageType {

    const REGISTRY_ID : Identifier = Identifier::vanilla_const("damage_type");
    
    fn to_registry_data_packet(&self) -> Option<Nbt> {
        let mut nbt = NbtCompound::new();
        nbt.insert("message_id" , NbtElement::String (self.message_id.clone()              ));
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
#[derive(Serialize, Deserialize, Debug)]
pub enum DamageDifficultyScaling {
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "when_caused_by_living_non_player")]
    NonLivingPlayer,
    #[serde(rename = "always")]
    Always
}
impl DamageDifficultyScaling { pub fn as_str(&self) -> &'static str { match (self) {
    Self::Never           => "never",
    Self::NonLivingPlayer => "when_caused_by_living_non_player",
    Self::Always          => "always"
} } }


#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Debug)]
pub enum DamageEffects {
    #[serde(rename = "hurt")]
    Hurt,
    #[serde(rename = "thorns")]
    Thorns,
    #[serde(rename = "drowning")]
    Drowning,
    #[serde(rename = "burning")]
    Burning,
    #[serde(rename = "poking")]
    Poking,
    #[serde(rename = "freezing")]
    Freezing
}
impl DamageEffects { pub fn as_str(&self) -> &'static str { match (self) {
    Self::Hurt     => "hurt",
    Self::Thorns   => "thorns",
    Self::Drowning => "drowning",
    Self::Burning  => "burning",
    Self::Poking   => "poking",
    Self::Freezing => "freezing"
} } }


#[derive(Copy, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Debug)]
pub enum DeathMessageType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "fall_variants")]
    FallVariants,
    #[serde(rename = "intentional_game_design")]
    IntentionalGameDesign
}
impl DeathMessageType { pub fn as_str(&self) -> &'static str { match (self) {
    Self::Default               => "default",
    Self::FallVariants          => "fall_variants",
    Self::IntentionalGameDesign => "intentional_game_design"
} } }