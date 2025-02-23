use crate::value::PacketEnumOrdinal;
use crate::value::{Identifier, LengthPrefixVec, Nbt, PacketEncode, NbtText};
use crate::value::PacketDecode;
use crate::value::PacketBuf;
use crate::value::EncodeError;
use crate::value::DecodeError;
use crate::value::VarInt;
use crate::value::Uuid;
use crate::packet::TODO;
use crate::mojang::auth_verify::MojAuthProperty;
use super::{AttributeType, Block, Either, IdSet, RegEntry, SlotData};
use voxidian_protocol_macros::{component, component_enum, packet_part};


#[component("minecraft:custom_data")]
pub struct CustomData {
    pub data : Nbt
}

#[component("minecraft:max_stack_size")]
pub struct MaxStackSize {
    pub amount : VarInt
}

#[component("minecraft:max_damage")]
pub struct MaxDamage {
    pub amount : VarInt
}

#[component("minecraft:unbreakable")]
pub struct Unbreakable {
    pub show_tooltip : bool
}

#[component("minecraft:custom_name")]
pub struct CustomName {
    pub name : NbtText
}

#[component("minecraft:item_name")]
pub struct ItemName {
    pub name : NbtText
}

#[component("minecraft:item_model")]
pub struct ItemModel {
    pub asset : Identifier
}

#[component("minecraft:lore")]
pub struct Lore {
    pub lines : LengthPrefixVec<VarInt, NbtText>
}

#[component("minecraft:rarity")]
pub struct Rarity {
    pub rarity : RarityLevel
}
#[packet_part(VarInt)]
pub enum RarityLevel {
    Common   = 0,
    Uncommon = 1,
    Rare     = 2,
    Epic     = 3
}

#[component("minecraft:enchantments")]
pub struct Enchantments {
    // TODO: enchantment registry data
    pub show_in_tooltip: bool
}

#[component("minecraft:can_place_on")]
pub struct CanPlaceOn {
    pub predicates: LengthPrefixVec<VarInt, BlockPredicate>,
    pub show_in_tooltip: bool
}
#[component("minecraft:can_break")]
pub struct CanBreak {
    pub predicates: LengthPrefixVec<VarInt, BlockPredicate>,
    pub show_in_tooltip: bool
}

#[packet_part]
#[derive(PartialEq)]
pub struct BlockPredicate {
    pub blocks: Option<IdSet<Block>>,
    pub properties: Option<LengthPrefixVec<VarInt, BlockPredicateProperty>>,
    pub nbt: Option<Nbt>
}

#[packet_part]
#[derive(PartialEq)]
pub struct BlockPredicateProperty {
    pub property: String,
    pub values: Either<String, (String, String)>
}

#[component("minecraft:attribute_modifiers")]
pub struct AttributeModifiers {
    pub modifiers: LengthPrefixVec<VarInt, AttributeModifier>,
    pub show_in_tooltip: bool
}

#[packet_part]
#[derive(PartialEq)]
pub struct AttributeModifier {
    pub type_id: RegEntry<AttributeType>,
    pub unique_id: Identifier,
    pub value: f64,
    pub operation: AttributeOperation,
}

#[packet_part(VarInt)]
pub enum AttributeOperation {
    Add = 0,
    MultiplyBase,
    MultiplyTotal
}

#[packet_part(VarInt)]
pub enum AttributeSlot {
    Any = 0,
    MainHand,
    OffHand,
    Hand,
    Feet,
    Legs,
    Chest,
    Head,
    Armor,
    Body
}

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
    pub has_glint : bool
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

#[component("minecraft:tool")]
pub struct Tool {
    pub rules: LengthPrefixVec<VarInt, ToolRule>,
    pub default_mining_speed: f32,
    pub damage_per_block: i32
}

#[packet_part]
#[derive(PartialEq)]
pub struct ToolRule {
    pub blocks: IdSet<Block>,
    pub speed: Option<f32>,
    pub correct_drops_for_blocks: Option<bool>
}

#[component("minecraft:enchantable")]
pub struct Enchantable {
    pub how_expensive: VarInt
}

#[component("minecraft:equippable")]
pub struct Equippable(TODO);

#[component("minecraft:repairable")]
pub struct Repairable(TODO);

#[component("minecraft:glider")]
pub struct Glider {}

#[component("minecraft:tooltip_style")]
pub struct TooltipStyle {
    pub style: Identifier
}

#[component("minecraft:death_protection")]
pub struct DeathProtection(TODO);

#[component("minecraft:stored_enchantments")]
pub struct StoredEnchantments(TODO);

#[component("minecraft:dyed_color")]
pub struct DyedColor {
    pub colour : i32,
    pub show_in_tooltip : bool
}

#[component("minecraft:map_id")]
pub struct MapId {
    pub id: VarInt
}

#[component("minecraft:map_decorations")]
pub struct MapDecorations {
    pub decorations: Nbt
}

#[component("minecraft:map_post_processing")]
pub struct MapPostProcessing(TODO);

#[component("minecraft:charged_projectiles")]
pub struct ChargedProjectiles(TODO);

#[component("minecraft:bundle_contents")]
pub struct BundleContents(TODO);

#[component("minecraft:potion_contents")]
pub struct PotionContents(TODO);

#[component("minecraft:suspicious_stew_effects")]
pub struct SuspiciousStewEffects(TODO);

#[component("minecraft:writable_book_content")]
pub struct WritableBookContent(TODO);

#[component("minecraft:written_book_content")]
pub struct WrittenBookContent {
    pub title          : String,
    pub filtered_title : Option<String>,
    pub author         : String,
    pub generation     : VarInt,
    pub pages          : LengthPrefixVec<VarInt, BookPage>,
    pub resolved       : bool
}
#[packet_part]
#[derive(PartialEq)]
pub struct BookPage {
    pub content          : NbtText,
    pub filtered_content : Option<NbtText>
}

#[component("minecraft:trim")]
pub struct Trim(TODO);

#[component("minecraft:debug_stick_state")]
pub struct DebugStickState(TODO);

#[component("minecraft:entity_data")]
pub struct EntityData(TODO);

#[component("minecraft:bucket_entity_data")]
pub struct BucketEntityData(TODO);

#[component("minecraft:block_entity_data")]
pub struct BlockEntityData(TODO);

#[component("minecraft:instrument")]
pub struct Instrument(TODO);

#[component("minecraft:ominous_bottle_amplifier")]
pub struct OminousBottleAmplifier(TODO);

#[component("minecraft:jukebox_playable")]
pub struct JukeboxPlayable(TODO);

#[component("minecraft:recipes")]
pub struct Recipes(TODO);

#[component("minecraft:lodestone_tracker")]
pub struct LodestoneTracker(TODO);

#[component("minecraft:firework_explosion")]
pub struct FireworkExplosion(TODO);

#[component("minecraft:fireworks")]
pub struct Fireworks(TODO);

#[component("minecraft:profile")]
pub struct Profile {
    pub name       : Option<String>,
    pub uuid       : Option<Uuid>,
    pub properties : LengthPrefixVec<VarInt, ProfileProperty>
}
#[packet_part]
#[derive(PartialEq)]
pub struct ProfileProperty {
    pub name  : String,
    pub value : String,
    pub sig   : Option<String>
}
impl From<MojAuthProperty> for ProfileProperty {
    fn from(from : MojAuthProperty) -> Self {
        Self {
            name  : from.name,
            value : from.value,
            sig   : Some(from.sig)
        }
    }
}

#[component("minecraft:note_block_sound")]
pub struct NoteBlockSound(TODO);

#[component("minecraft:banner_patterns")]
pub struct BannerPatterns(TODO);

#[component("minecraft:base_color")]
pub struct BaseColor(TODO);

#[component("minecraft:pot_decorations")]
pub struct PotDecorations(TODO);

#[component("minecraft:container")]
pub struct Container {
    pub contents: LengthPrefixVec<VarInt, SlotData>
}

#[component("minecraft:block_state")]
pub struct BlockState {
    pub state: LengthPrefixVec<VarInt, (String, String)>
}

#[component("minecraft:lock")]
pub struct Lock(TODO);

#[component("minecraft:bees")]
pub struct Bees(TODO);

#[component("minecraft:container_loot")]
pub struct ContainerLoot(TODO);

#[component("minecraft:map_color")]
pub struct MapColor(TODO);

#[component("minecraft:damage")]
pub struct Damage {
    pub damage: VarInt
}


component_enum!();
