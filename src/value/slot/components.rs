use crate::value::{EntityType, PacketEnumOrdinal, RegOr, SoundEvent};
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
pub struct CustomDataComp {
    pub data : Nbt
}

#[component("minecraft:max_stack_size")]
pub struct MaxStackSizeComp {
    pub amount : VarInt
}

#[component("minecraft:max_damage")]
pub struct MaxDamageComp {
    pub amount : VarInt
}

#[component("minecraft:unbreakable")]
pub struct UnbreakableComp {
    pub show_tooltip : bool
}

#[component("minecraft:custom_name")]
pub struct CustomNameComp {
    pub name : NbtText
}

#[component("minecraft:item_name")]
pub struct ItemNameComp {
    pub name : NbtText
}

#[component("minecraft:item_model")]
pub struct ItemModelComp {
    pub asset : Identifier
}

#[component("minecraft:lore")]
pub struct LoreComp {
    pub lines : LengthPrefixVec<VarInt, NbtText>
}

#[component("minecraft:rarity")]
pub struct RarityComp {
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
pub struct EnchantmentsComp {
    // TODO: enchantment registry data
    pub show_in_tooltip: bool
}

#[component("minecraft:can_place_on")]
pub struct CanPlaceOnComp {
    pub predicates: LengthPrefixVec<VarInt, BlockPredicate>,
    pub show_in_tooltip: bool
}
#[component("minecraft:can_break")]
pub struct CanBreakComp {
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
pub struct AttributeModifiersComp {
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
pub struct CustomModelDataComp {
    // TODO: probably outdated
    pub data: VarInt
}

#[component("minecraft:repair_cost")]
pub struct RepairCostComp {
    pub cost: VarInt
}

#[component("minecraft:creative_slot_lock")]
pub struct CreativeSlotLockComp {}

#[component("minecraft:enchantment_glint_override")]
pub struct EnchantmentGlintOverrideComp {
    pub has_glint : bool
}

#[component("minecraft:intangible_projectile")]
pub struct IntangibleProjectileComp {}

#[component("minecraft:food")]
pub struct FoodComp {
    pub food: VarInt,
    pub saturation: f32,
    pub can_always_eat: bool,
}

#[component("minecraft:consumable")]
pub struct ConsumableComp {
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
pub struct UseRemainderComp {
    pub remainder: SlotData
}

#[component("minecraft:use_cooldown")]
pub struct UseCooldownComp {
    pub seconds: f32,
    pub cooldown_group: Identifier
}

#[component("minecraft:damage_resistant")]
pub struct DamageResistantComp {
    pub damage_type_tag: String
}

#[component("minecraft:tool")]
pub struct ToolComp {
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
pub struct EnchantableComp {
    pub how_expensive: VarInt
}

#[component("minecraft:equippable")]
pub struct EquippableComp {
    pub slot: EquippableSlot,
    pub equip_sound: RegOr<SoundEvent, SoundEvent>,
    pub model: Option<Identifier>,
    pub camera_overlay: Option<Identifier>,
    pub allowed_entities: Option<IdSet<EntityType>>,
    pub dispensable: bool,
    pub swappable: bool,
    pub damage_on_hurt: bool,
}

#[packet_part(VarInt)]
pub enum EquippableSlot {
    MainHand = 0,
    Feet = 1,
    Legs = 2,
    Chest = 3,
    Head = 4,
    Offhand = 5,
    Body = 6,
    Saddle = 7
}

#[component("minecraft:repairable")]
pub struct RepairableComp(TODO);

#[component("minecraft:glider")]
pub struct GliderComp {}

#[component("minecraft:tooltip_style")]
pub struct TooltipStyleComp {
    pub style: Identifier
}

#[component("minecraft:death_protection")]
pub struct DeathProtectionComp(TODO);

#[component("minecraft:stored_enchantments")]
pub struct StoredEnchantmentsComp(TODO);

#[component("minecraft:dyed_color")]
pub struct DyedColorComp {
    pub colour : i32,
    pub show_in_tooltip : bool
}

#[component("minecraft:map_id")]
pub struct MapIdComp {
    pub id: VarInt
}

#[component("minecraft:map_decorations")]
pub struct MapDecorationsComp {
    pub decorations: Nbt
}

#[component("minecraft:map_post_processing")]
pub struct MapPostProcessingComp(TODO);

#[component("minecraft:charged_projectiles")]
pub struct ChargedProjectilesComp(TODO);

#[component("minecraft:bundle_contents")]
pub struct BundleContentsComp(TODO);

#[component("minecraft:potion_contents")]
pub struct PotionContentsComp(TODO);

#[component("minecraft:suspicious_stew_effects")]
pub struct SuspiciousStewEffectsComp(TODO);

#[component("minecraft:writable_book_content")]
pub struct WritableBookContentComp(TODO);

#[component("minecraft:written_book_content")]
pub struct WrittenBookContentComp {
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
pub struct TrimComp(TODO);

#[component("minecraft:debug_stick_state")]
pub struct DebugStickStateComp(TODO);

#[component("minecraft:entity_data")]
pub struct EntityDataComp(TODO);

#[component("minecraft:bucket_entity_data")]
pub struct BucketEntityDataComp(TODO);

#[component("minecraft:block_entity_data")]
pub struct BlockEntityDataComp(TODO);

#[component("minecraft:instrument")]
pub struct InstrumentComp(TODO);

#[component("minecraft:ominous_bottle_amplifier")]
pub struct OminousBottleAmplifierComp(TODO);

#[component("minecraft:jukebox_playable")]
pub struct JukeboxPlayableComp(TODO);

#[component("minecraft:recipes")]
pub struct RecipesComp(TODO);

#[component("minecraft:lodestone_tracker")]
pub struct LodestoneTrackerComp(TODO);

#[component("minecraft:firework_explosion")]
pub struct FireworkExplosionComp(TODO);

#[component("minecraft:fireworks")]
pub struct FireworksComp(TODO);

#[component("minecraft:profile")]
pub struct ProfileComp {
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
pub struct NoteBlockSoundComp(TODO);

#[component("minecraft:banner_patterns")]
pub struct BannerPatternsComp(TODO);

#[component("minecraft:base_color")]
pub struct BaseColorComp(TODO);

#[component("minecraft:pot_decorations")]
pub struct PotDecorationsComp(TODO);

#[component("minecraft:container")]
pub struct ContainerComp {
    pub contents: LengthPrefixVec<VarInt, SlotData>
}

#[component("minecraft:block_state")]
pub struct BlockStateComp {
    pub state: LengthPrefixVec<VarInt, (String, String)>
}

#[component("minecraft:lock")]
pub struct LockComp(TODO);

#[component("minecraft:bees")]
pub struct BeesComp(TODO);

#[component("minecraft:container_loot")]
pub struct ContainerLootComp(TODO);

#[component("minecraft:map_color")]
pub struct MapColorComp(TODO);

#[component("minecraft:damage")]
pub struct DamageComp {
    pub damage: VarInt
}

#[component("minecraft:blocks_attacks")]
pub struct BlocksAttacksComp(TODO);

#[component("minecraft:cat/variant")]
pub struct CatVariantComp(TODO);

#[component("minecraft:horse/variant")]
pub struct HorseVariantComp(TODO);

#[component("minecraft:break_sound")]
pub struct BreakSoundComp(TODO);

#[component("minecraft:fox/variant")]
pub struct FoxVariantComp(TODO);

#[component("minecraft:rabbit/variant")]
pub struct RabbitVariantComp(TODO);

#[component("minecraft:wolf/sound_variant")]
pub struct WolfSoundVariantComp(TODO);

#[component("minecraft:weapon")]
pub struct WeaponComp(TODO);

#[component("minecraft:cow/variant")]
pub struct CowVariantComp(TODO);

#[component("minecraft:provides_banner_patterns")]
pub struct ProvidesBannerPatternsComp(TODO);

#[component("minecraft:axolotl/variant")]
pub struct AxolotlVariantComp(TODO);

#[component("minecraft:salmon/size")]
pub struct SalmonSizeComp(TODO);

#[component("minecraft:parrot/variant")]
pub struct ParrotVariantComp(TODO);

#[component("minecraft:tropical_fish/base_color")]
pub struct TropicalFishBaseColorComp(TODO);

#[component("minecraft:potion_duration_scale")]
pub struct PotionDurationScaleComp(TODO);

#[component("minecraft:tropical_fish/pattern")]
pub struct TropicalFishPatternComp(TODO);

#[component("minecraft:pig/variant")]
pub struct PigVariantComp(TODO);

#[component("minecraft:tooltip_display")]
pub struct TooltipDisplayComp(TODO);

#[component("minecraft:chicken/variant")]
pub struct ChickenVariantComp(TODO);

#[component("minecraft:villager/variant")]
pub struct VillagerVariantComp(TODO);

#[component("minecraft:llama/variant")]
pub struct LlamaVariantComp(TODO);

#[component("minecraft:frog/variant")]
pub struct FrogVariantComp(TODO);

#[component("minecraft:tropical_fish/pattern_color")]
pub struct TropicalFishPatternColorComp(TODO);

#[component("minecraft:painting/variant")]
pub struct PaintingVariantComp(TODO);

#[component("minecraft:wolf/variant")]
pub struct WolfVariantComp(TODO);

#[component("minecraft:provides_trim_material")]
pub struct ProvidesTrimMaterialComp(TODO);

#[component("minecraft:shulker/color")]
pub struct ShulkerColorComp(TODO);

#[component("minecraft:wolf/collar")]
pub struct WolfCollarComp(TODO);

#[component("minecraft:mooshroom/variant")]
pub struct MooshroomVariantComp(TODO);

#[component("minecraft:cat/collar")]
pub struct CatCollarComp(TODO);

#[component("minecraft:sheep/color")]
pub struct SheepColorComp(TODO);

component_enum!();
