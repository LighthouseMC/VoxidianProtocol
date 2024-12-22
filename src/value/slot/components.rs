
use crate::value::PacketEnumOrdinal;
use crate::value::{Identifier, LengthPrefixVec, Nbt, PacketEncode, Text, TextComponent};
use crate::value::PacketDecode;
use crate::value::PacketBuf;
use crate::value::EncodeError;
use crate::value::DecodeError;
use crate::value::VarInt;
use voxidian_protocol_macros::{component, component_enum, packet_part};
use crate::registry::RegEntry;

#[component("minecraft:custom_data")]
#[packet_part]
pub struct CustomData {
    pub data: Nbt
}

#[component("minecraft:max_stack_size")]
#[packet_part]
pub struct MaxStackSize {
    pub amount: VarInt
}

#[component("minecraft:max_damage")]
#[packet_part]
pub struct MaxDamage {
    pub amount: VarInt
}

#[component("minecraft:unbreakable")]
#[packet_part]
pub struct Unbreakable {
    pub show_tooltip: bool
}

#[component("minecraft:custom_name")]
#[packet_part]
pub struct CustomName {
    pub amount: TextComponent
}

#[component("minecraft:item_name")]
#[packet_part]
pub struct ItemName {
    pub amount: TextComponent
}

#[component("minecraft:item_model")]
#[packet_part]
pub struct ItemModel {
    pub amount: Identifier
}

#[component("minecraft:lore")]
#[packet_part]
pub struct Lore {
    pub amount: LengthPrefixVec<VarInt, TextComponent>
}

#[component("minecraft:rarity")]
#[packet_part]
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
#[packet_part]
pub struct Enchantments {
    // TODO: enchantment registry data
    pub show_in_tooltip: bool
}

// TODO: can_place_on and can_break need Block Predicates

// TODO: attribute_modifiers

#[component("minecraft:custom_model_data")]
#[packet_part]
pub struct CustomModelData {
    // TODO: probably outdated
    pub data: VarInt
}

#[component("minecraft:hide_additional_tooltip")]
#[packet_part]
pub struct HideAdditionalTooltip {}

#[component("minecraft:hide_tooltip")]
#[packet_part]
pub struct HideTooltip {}

#[component("minecraft:repair_cost")]
#[packet_part]
pub struct RepairCost {
    pub cost: VarInt
}

#[component("minecraft:creative_slot_lock")]
#[packet_part]
pub struct CreativeSlotLock {}

#[component("minecraft:enchantment_glint_override")]
#[packet_part]
pub struct EnchantmentGlintOverride {
    pub has_glint: bool
}

#[component("minecraft:intangible_projectile")]
#[packet_part]
pub struct IntangibleProjectile {}

// TODO: minecraft:food
// TODO: minecraft:consumable
// TODO: minecraft:use_remainder
// TODO: minecraft:use_cooldown

#[component("minecraft:damage_resistant")]
#[packet_part]
pub struct DamageResistant {
    pub damage_type_tag: String
}

// TODO: minecraft:tool

#[component("minecraft:enchantable")]
#[packet_part]
pub struct Enchantable {
    pub how_expensive: VarInt
}

// TODO: minecraft:equippable

// TODO: minecraft:repairable (what is ID Set?)

#[component("minecraft:glider")]
#[packet_part]
pub struct Glider {}

#[component("minecraft:tooltip_style")]
#[packet_part]
pub struct TooltipStyle {
    pub style: Identifier
}

// TODO: minecraft:death_protection

// TODO: minecraft:stored_enchantments

// TODO: minecraft:dyed_color - make custom Color type

#[component("minecraft:map_id")]
#[packet_part]
pub struct MapId {
    pub id: VarInt
}

#[component("minecraft:map_decorations")]
#[packet_part]
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

// TODO: minecraft:container_loo