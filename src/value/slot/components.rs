
use crate::value::PacketEnumOrdinal;
use crate::value::{Identifier, LengthPrefixVec, Nbt, PacketEncode, TextComponent};
use crate::value::PacketDecode;
use crate::value::PacketBuf;
use crate::value::EncodeError;
use crate::value::DecodeError;
use crate::value::VarInt;
use crate::packet::TODO;
use voxidian_protocol_macros::{component, packet_part};

use super::SlotData;

#[component("minecraft:custom_data")]
pub struct CustomData {
    pub data: Nbt
}

#[component("minecraft:max_stack_size")]
pub struct MaxStackSize {
    pub amount: VarInt
}

#[component("minecraft:max_damage")]
pub struct MaxDamage {
    pub amount: VarInt
}

#[component("minecraft:unbreakable")]
pub struct Unbreakable {
    pub show_tooltip: bool
}

#[component("minecraft:custom_name")]
pub struct CustomName {
    pub amount: TextComponent
}

#[component("minecraft:item_name")]
pub struct ItemName {
    pub amount: TextComponent
}

#[component("minecraft:item_model")]
pub struct ItemModel {
    pub amount: Identifier
}

#[component("minecraft:lore")]
pub struct Lore {
    pub amount: LengthPrefixVec<VarInt, TextComponent>
}

#[component("minecraft:rarity")]
pub struct Rarity {
    pub rarity: RarityLevel
}

#[packet_part(VarInt)]
pub enum RarityLevel {
    Common = 0,
    Uncommon = 1,
    Rare = 2,
    Epic = 3
}

#[component("minecraft:enchantments")]
pub struct Enchantments {
    // TODO: enchantment registry data
    pub show_in_tooltip: bool
}

// TODO: can_place_on and can_break need Block Predicates

// TODO: attribute_modifiers

#[component("minecraft:custom_model_data")]
pub struct CustomModelData {
    // TODO: probably outdated
    pub data: VarInt
}

#[component("minecraft:hide_additional_tooltip")]
pub struct HideAdditionalTooltip {}

#[component("minecraft:hide_tooltip")]
pub struct HideTooltip {}

#[component("minecraft:repair_cost")]
pub struct RepairCost {
    pub cost: VarInt
}

#[component("minecraft:creative_slot_lock")]
pub struct CreativeSlotLock {}

#[component("minecraft:enchantment_glint_override")]
pub struct EnchantmentGlintOverride {
    pub has_glint: bool
}

#[component("minecraft:intangible_projectile")]
pub struct IntangibleProjectile {}

#[component("minecraft:food")]
pub struct Food {
    pub food: VarInt,
    pub saturation: f32,
    pub can_always_eat: bool,
}

#[component("minecraft:consumable")]
pub struct Consumable {
    pub consume_seconds: f32,
    pub animation: EatingAnimation,
    pub consume_sound: Identifier,
    pub consume_effects: TODO
}

#[packet_part(VarInt)]
pub enum EatingAnimation {
    None = 0,
    Eat = 1,
    Drink,
    Spyglass,
    Block,
    Bow,
    Spear,
    Crossbow,
    Shield,
    Trident,
    TootHorn,
    Brush
}

#[component("minecraft:use_remainder")]
pub struct UseRemainder {
    pub remainder: SlotData
}

#[component("minecraft:use_cooldown")]
pub struct UseCooldown {
    pub seconds: f32,
    pub cooldown_group: Identifier
}

#[component("minecraft:damage_resistant")]
pub struct DamageResistant {
    pub damage_type_tag: String
}

// TODO: minecraft:tool

#[component("minecraft:enchantable")]
pub struct Enchantable {
    pub how_expensive: VarInt
}

// TODO: minecraft:equippable

// TODO: minecraft:repairable (what is ID Set?)

#[component("minecraft:glider")]
pub struct Glider {}

#[component("minecraft:tooltip_style")]
pub struct TooltipStyle {
    pub style: Identifier
}

// TODO: minecraft:death_protection

// TODO: minecraft:stored_enchantments

// TODO: minecraft:dyed_color - make custom Color type

#[component("minecraft:map_id")]
pub struct MapId {
    pub id: VarInt
}

#[component("minecraft:map_decorations")]
pub struct MapDecorations {
    pub decorations: Nbt
}

// TODO: minecraft:map_post_processing

// TODO: minecraft:charged_projectiles

// TODO: minecraft:bundle_contents

// TODO: minecraft:potion_contents

// TODO: minecraft:suspicious_stew_effects

// TODO: minecraft:writable_book_contents

// TODO: minecraft:written_book_contents

// TODO: minecraft:trim

// TODO: minecraft:debug_stick_state

// TODO: minecraft:entity_data

// TODO: minecraft:bucket_entity_data

// TODO: minecraft:block_entity_data

// TODO: minecraft:instrument

// TODO: minecraft:ominous_bottle_amplifier

// TODO: minecraft:jukebox_playable

// TODO: minecraft:recipes

// TODO: minecraft:lodestone_tracker

// TODO: minecraft:firework_explosion

// TODO: minecraft:fireworks

// TODO: minecraft:profile

// TODO: minecraft:note_block_sound

// TODO: minecraft:banner_patterns

// TODO: minecraft:base_color

// TODO: minecraft:pot_decorations

// TODO: minecraft:container

// TODO: minecraft:block_state

// TODO: minecraft:lock

// TODO: minecraft:container_loot