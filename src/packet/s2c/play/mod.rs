use super::*;
use config::{ ReportDetail, ServerLink };


#[packet( prefix = 0x00, bound = S2C, stage = Play )]
pub struct BundleDelimiterS2CPacket;


#[packet( prefix = 0x01, bound = S2C, stage = Play )]
pub struct SpawnEntityS2CPacket {
    pub id       : VarInt,
    pub uuid     : Uuid,
    pub kind     : RegEntry<EntityType>,
    pub x        : f64,
    pub y        : f64,
    pub z        : f64,
    pub pitch    : Angle,
    pub yaw      : Angle,
    pub head_yaw : Angle,
    pub data     : VarInt,
    pub vel_x    : f64,
    pub vel_y    : f64,
    pub vel_z    : f64
}


#[packet( prefix = 0x02, bound = S2C, stage = Play )]
pub struct SpawnXpOrbS2CPacket {
    pub id    : VarInt,
    pub x     : f64,
    pub y     : f64,
    pub z     : f64,
    pub count : i16
}


#[packet( prefix = 0x03, bound = S2C, stage = Play )]
pub struct EntityAnimationS2CPacket {
    pub id   : VarInt,
    pub anim : EntityAnimation
}
#[packet_part(u8)]
pub enum EntityAnimation {
    SwingMainHand   = 0,
    LeaveBed        = 2,
    SwingOffHand    = 3,
    CritEffect      = 4,
    MagicCritEffect = 5
}


#[packet( prefix = 0x04, bound = S2C, stage = Play )]
pub struct AwardStatS2CPacket {
    pub stats : LengthPrefixVec<VarInt, Stat>
}
#[packet_part]
pub struct Stat {
    pub category : StatCategory,
    pub kind     : VarInt,
    pub value    : VarInt
}
#[packet_part(VarInt)]
pub enum StatCategory {
    Mined    = 0,
    Crafted  = 1,
    Used     = 2,
    Broken   = 3,
    PickedUp = 4,
    Dropped  = 5,
    Killed   = 6,
    KilledBy = 7,
    Custom   = 8
}


#[packet( prefix = 0x05, bound = S2C, stage = Play )]
pub struct AcknowledgeBlockChangeS2CPacket(pub VarInt);


#[packet( prefix = 0x06, bound = S2C, stage = Play )]
pub struct SetBlockDestroyStageS2CPacket {
    pub entity_id : VarInt,
    pub pos       : BlockPos,
    pub stage     : u8
}


#[packet( prefix = 0x07, bound = S2C, stage = Play )]
pub struct BlockEntityDataS2CPacket {
    pub pos  : BlockPos,
    pub kind : VarInt,
    pub data : Nbt
}


#[packet( prefix = 0x08, bound = S2C, stage = Play )]
pub struct BlockActionS2CPacket {
    pub pos    : BlockPos,
    pub action : u8,
    pub param  : u8,
    pub block  : RegEntry<Block>
}


#[packet( prefix = 0x09, bound = S2C, stage = Play )]
pub struct BlockUpdateS2CPacket {
    pub pos   : BlockPos,
    pub block : RegEntry<BlockState>
}


#[packet( prefix = 0x0A, bound = S2C, stage = Play )]
pub struct BossBarS2CPacket {
    pub uuid   : Uuid,
    pub action : BossBarAction
}
#[packet_part(VarInt)]
pub enum BossBarAction {
    Add {
        title    : Text,
        health   : f32,
        colour   : BossBarColour,
        division : BossBarDivision,
        flags    : BossBarFlags
    },
    Remove,
    UpdateHealth {
        health : f32
    },
    UpdateTitle {
        title : Text
    },
    UpdateStyle {
        colour   : BossBarColour,
        division : BossBarDivision
    },
    UpdateFlags {
        flags : BossBarFlags
    }
}
#[packet_part(VarInt)]
pub enum BossBarColour {
    Pink   = 0,
    Blue   = 1,
    Red    = 2,
    Green  = 3,
    Yellow = 4,
    Purple = 5,
    White  = 6
}
#[packet_part(VarInt)]
pub enum BossBarDivision {
    None    = 0,
    Notch6  = 1,
    Notch10 = 2,
    Notch12 = 3,
    Notch20 = 4
}
packet_flags!{ pub struct BossBarFlags {
    pub darken_sky  : 0b00000001,
    pub end_music   : 0b00000010,
    pub thicken_fog : 0b00000100
} }


#[packet( prefix = 0x0B, bound = S2C, stage = Play )]
pub struct ChangeDifficultyS2CPacket {
    pub difficulty : Difficulty,
    pub locked     : bool
}
#[packet_part(u8)]
pub enum Difficulty {
    Peaceful = 0,
    Easy     = 1,
    Normal   = 2,
    Hard     = 3
}


#[packet( prefix = 0x0C, bound = S2C, stage = Play )]
pub struct ChunkBatchFinishS2CPacket {
    pub size : VarInt
}


#[packet( prefix = 0x0D, bound = S2C, stage = Play )]
pub struct ChunkBatchStartS2CPacket;


#[packet( prefix = 0x0E, bound = S2C, stage = Play )]
pub struct ChunkBiomesS2CPacket {
    pub chunks : LengthPrefixVec<VarInt, ChunkBiomeData>
}
#[packet_part]
pub struct ChunkBiomeData {
    pub chunk_x : i32,
    pub chunk_z : i32,
    pub data    : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x0F, bound = S2C, stage = Play )]
pub struct ClearTitlesS2CPacket {
    pub reset : bool
}


#[packet( prefix = 0x10, bound = S2C, stage = Play )]
pub struct CommandSuggestionsResponseS2CPacket {
    pub id          : VarInt,
    pub start       : VarInt,
    pub len         : VarInt,
    pub suggestions : LengthPrefixVec<VarInt, CommandSuggestion>
}
#[packet_part]
pub struct CommandSuggestion {
    pub matched : String,
    pub tooltip : Option<Text>
}


/*#[packet( prefix = 0x11, bound = S2C, stage = Play )]
pub struct CommandsS2CPacket {
    pub nodes    : LengthPrefixVec<VarInt, CommandNode>,
    pub root_idx : VarInt
}
#[packet_part]
pub struct CommandNode {
    // TODO
}*/


#[packet( prefix = 0x12, bound = S2C, stage = Play )]
pub struct CloseContainerS2CPacket {
    pub window : u8
}


// TODO: SetContainerContentS2CPacket


#[packet( prefix = 0x14, bound = S2C, stage = Play )]
pub struct SetContainerPropertyS2CPacket {
    pub window : u8,
    pub prop   : u16,
    pub value  : u16
}


// TODO: SetContainerSlotS2CPacket


#[packet( prefix = 0x16, bound = S2C, stage = Play )]
pub struct CookieRequestS2CPacket {
    pub key : Identifier
}


#[packet( prefix = 0x17, bound = S2C, stage = Play )]
pub struct SetCooldownS2CPacket {
    pub item  : RegEntry<Item>,
    pub ticks : VarInt
}


#[packet( prefix = 0x18, bound = S2C, stage = Play )]
pub struct ChatSuggestionsS2CPacket {
    pub action : ChatSuggestionAction
}
#[packet_part(VarInt)]
pub enum ChatSuggestionAction {
    Add    = 0,
    Remove = 1,
    Set    = 2
}


#[packet( prefix = 0x19, bound = S2C, stage = Play )]
pub struct PluginMessageS2CPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( prefix = 0x1A, bound = S2C, stage = Play )]
pub struct DamageEventS2CPacket {
    pub entity               : VarInt,
    pub source_type          : RegEntry<DamageType>,
    pub source_cause_entity  : OptionVarInt,
    pub source_direct_entity : OptionVarInt,
    pub source_pos           : Option<Vec3d>
}


#[packet( prefix = 0x1B, bound = S2C, stage = Play )]
pub struct DebugSampleS2CPacket {
    pub sample : LengthPrefixVec<VarInt, u64>,
    pub kind   : DebugSampleKind
}
#[packet_part(VarInt)]
pub enum DebugSampleKind {
    TickTime = 0
}


#[packet( prefix = 0x1C, bound = S2C, stage = Play )]
pub struct DeleteMessageS2CPacket {
    pub msg       : VarInt,
    pub signature : Option<[u8; 256]>
}


#[packet( prefix = 0x1D, bound = S2C, stage = Play )]
pub struct DisconnectS2CPacket {
    pub reason : Text
}


#[packet( prefix = 0x1E, bound = S2C, stage = Play )]
pub struct DisguisedChatMessageS2CPacket {
    pub msg       : Text,
    pub chat_type : RegEntry<ChatType>
}


#[packet( prefix = 0x1F, bound = S2C, stage = Play )]
pub struct EntityEventS2CPacket {
    pub entity : i32,
    pub status : u8
}


#[packet( prefix = 0x20, bound = S2C, stage = Play )]
pub struct ExplosionS2CPacket {
    pub x              : f64,
    pub y              : f64,
    pub z              : f64,
    pub strength       : f32,
    pub records        : LengthPrefixVec<VarInt, [u8; 3]>,
    pub player_mot_x   : f32,
    pub player_mot_y   : f32,
    pub player_mot_z   : f32,
    pub blocks         : ExplosionBlockInteraction,
    pub small_particle : Particle,
    pub large_particle : Particle,
    pub sound          : RegOr<SoundEvent, Sound>
}
#[packet_part(VarInt)]
pub enum ExplosionBlockInteraction {
    Keep         = 0,
    Destroy      = 1,
    DestroyDecay = 2,
    Trigger      = 3
}
#[packet_part]
pub struct Sound {
    pub name        : Identifier,
    pub fixed_range : Option<f32>
}


#[packet( prefix = 0x21, bound = S2C, stage = Play )]
pub struct UnloadChunkS2CPacket {
    pub chunk_z : i32,
    pub chunk_x : i32
}


#[packet( prefix = 0x22, bound = S2C, stage = Play )]
pub struct GameEventS2CPacket {
    pub event : GameEvent,
    pub value : f32
}
#[packet_part(u8)]
pub enum GameEvent {
    NoRespawnBlock   = 0,
    BeginRain        = 1,
    EndRain          = 2,
    ChangeGameMode   = 3,
    WinGame          = 4,
    DemoEvent        = 5,
    ArrowHitPlayer   = 6,
    RainLevel        = 7,
    ThunderLevel     = 8,
    PufferfishSting  = 9,
    GuardianGhost    = 10,
    ImmediateRespawn = 11,
    LimitedCrafting  = 12,
    WaitForChunks    = 13
}


#[packet( prefix = 0x23, bound = S2C, stage = Play )]
pub struct OpenHorseScreenS2CPacket {
    pub window : u8,
    pub slots  : VarInt,
    pub entity : i32
}


#[packet( prefix = 0x24, bound = S2C, stage = Play )]
pub struct HurtAnimationS2CPacket {
    pub entity : VarInt,
    pub dyaw   : f32
}


#[packet( prefix = 0x25, bound = S2C, stage = Play )]
pub struct InitialiseWorldBorderS2CPacket {
    pub x               : f64,
    pub z               : f64,
    pub old_diameter    : f64,
    pub new_diameter    : f64,
    /// In milliseconds
    pub transition      : VarLong,
    pub portal_boundary : VarInt,
    /// In blocks
    pub warning_dist    : VarInt,
    /// In seconds
    pub warning_time    : VarInt
}


#[packet( prefix = 0x26, bound = S2C, stage = Play )]
pub struct KeepAliveS2CPacket(pub u64);


// TODO: ChunkDataUpdateS2CPacket


// TODO: WorldEventS2CPacket


#[packet( prefix = 0x29, bound = S2C, stage = Play )]
pub struct ParticleS2CPacket {
    pub long_distance : bool,
    pub x             : f64,
    pub y             : f64,
    pub z             : f64,
    /// Gaussian distribution
    pub spread_x      : f32,
    /// Gaussian distribution
    pub spread_y      : f32,
    /// Gaussian distribution
    pub spread_z      : f32,
    pub max_speed     : f32,
    pub count         : u32,
    pub particle      : Particle
}


// TODO: UpdateLightS2CPacket


#[packet( prefix = 0x2B, bound = S2C, stage = Play )]
pub struct LoginS2CPacket {
    pub entity               : i32,
    pub hardcore             : bool,
    pub dims                 : LengthPrefixVec<VarInt, Identifier>,
    pub max_players          : VarInt,
    pub view_dist            : VarInt,
    pub sim_dist             : VarInt,
    pub reduced_debug        : bool,
    /// Inverse of doImmediateRespawn
    pub respawn_screen       : bool,
    pub limited_crafting     : bool,
    pub dim                  : RegEntry<DimType>,
    pub dim_name             : Identifier,
    /// Hashed
    pub seed                 : u64,
    pub gamemode             : Gamemode,
    pub old_gamemode         : Gamemode,
    pub is_debug             : bool,
    pub is_flat              : bool,
    pub death_loc            : Option<DeathLocation>,
    pub portal_cooldown      : VarInt,
    pub enforce_chat_reports : bool
}
#[packet_part(u8)]
pub enum Gamemode {
    None      = (-1isize) as usize,
    Survival  = 0,
    Creative  = 1,
    Adventure = 2,
    Spectator = 3
}
#[packet_part]
pub struct DeathLocation {
    pub dim_name : Identifier,
    pub pos      : BlockPos
}


/*#[packet( prefix = 0x2C, bound = S2C, stage = Play )]
pub struct MapDataS2CPacket {
    pub id     : VarInt,
    /// 0~4
    pub scale  : u8,
    pub locked : bool,
    pub icons  : Option<LengthPrefixVec<VarInt, MapIcon>>,
    pub patch  : // TODO
}
#[packet_part]
pub struct MapIcon {
    pub kind : MapIconKind,
    pub x    : i8,
    pub z    : i8,
    /// 0~15
    pub dir  : u8,
    pub name : Option<Text>
}
#[packet_part(VarInt)]
pub enum MapIconKind {
    /// Players
    WhiteArrow       = 0,
    /// Item frames
    GreenArrow       = 1,
    RedArrow         = 2,
    BlueArrow        = 3,
    WhiteCross       = 4,
    RedPointer       = 5,
    /// Off-map players
    WhiteCircle      = 6,
    /// Far-off-map players
    SmallWhiteCircle = 7,
    Mansion          = 8,
    Monument         = 9,
    WhiteBanner      = 10,
    OrangeBanner     = 11,
    MagentaBanner    = 12,
    LightBlueBanner  = 13,
    YellowBanner     = 14,
    LimeBanner       = 15,
    PinkBanner       = 16,
    GreyBanner       = 17,
    LightGreyBanner  = 18,
    CyanBanner       = 19,
    PurpleBanner     = 20,
    BlueBanner       = 21,
    BrownBanner      = 22,
    GreenBanner      = 23,
    RedBanner        = 24,
    BlackBanner      = 25,
    Treasure         = 26
}*/


// TODO: MerchantOffersS2CPacket


/// Deltas are calculated using `(current * 4096) - (prev * 4096)`.
#[packet( prefix = 0x2E, bound = S2C, stage = Play )]
pub struct UpdateEntityPosS2CPacket {
    pub entity : VarInt,
    pub dx     : i16,
    pub dy     : i16,
    pub dz     : i16,
    pub ground : bool
}


/// Deltas are calculated using `(current * 4096) - (prev * 4096)`.
#[packet( prefix = 0x2F, bound = S2C, stage = Play )]
pub struct UpdateEntityPosAndRotS2CPacket {
    pub entity : VarInt,
    pub dx     : i16,
    pub dy     : i16,
    pub dz     : i16,
    pub yaw    : Angle,
    pub pitch  : Angle,
    pub ground : bool
}


#[packet( prefix = 0x30, bound = S2C, stage = Play )]
pub struct UpdateEntityRotS2CPacket {
    pub entity : VarInt,
    pub yaw    : Angle,
    pub pitch  : Angle,
    pub ground : bool
}


#[packet( prefix = 0x31, bound = S2C, stage = Play )]
pub struct MoveVehicleS2CPacket {
    pub x         : f64,
    pub y         : f64,
    pub z         : f64,
    pub yaw_deg   : f32,
    pub pitch_deg : f32
}


#[packet( prefix = 0x32, bound = S2C, stage = Play )]
pub struct OpenBookS2CPacket {
    pub hand : Hand
}
#[packet_part(VarInt)]
pub enum Hand {
    Mainhand = 0,
    Offhand  = 1
}


#[packet( prefix = 0x33, bound = S2C, stage = Play )]
pub struct OpenScreenS2CPacket {
    pub window : VarInt,
    pub kind   : RegEntry<Screen>,
    pub title  : Text
}


#[packet( prefix = 0x34, bound = S2C, stage = Play )]
pub struct OpenSignEditorS2CPacket {
    pub pos   : BlockPos,
    /// False means back
    pub front : bool
}


#[packet( prefix = 0x35, bound = S2C, stage = Play )]
pub struct PingS2CPacket(pub u32);


#[packet( prefix = 0x36, bound = S2C, stage = Play )]
pub struct PingResponseS2CPacket(pub u64);


#[packet( prefix = 0x37, bound = S2C, stage = Play )]
pub struct PlaceGhostRecipeS2CPacket {
    pub window : u8,
    pub recipe : Identifier
}


#[packet( prefix = 0x38, bound = S2C, stage = Play )]
pub struct PlayerAbilitiesS2CPacket {
    pub flags     : PlayerAbilityFlags,
    /// Default is 0.05
    pub fly_speed : f32,
    pub fov_fac   : f32
}
packet_flags!{ pub struct PlayerAbilityFlags {
    pub invul      : 0b00000001,
    pub flying     : 0b00000010,
    pub allow_fly  : 0b00000100,
    pub instabreak : 0b00001000
} }


/*#[packet( prefix = 0x39, bound = S2C, stage = Play )]
pub struct PlayerChatMessageS2CPacket {
    /// If zero, the message will bypass the client's `disableChat` option.
    pub sender        : Uuid,
    pub index         : VarInt,
    pub sig           : Option<[u8; 256]>,
    pub msg           : String,
    pub timestamp     : u64,
    pub salt          : u8,
    pub prev_msgs     : LengthPrefixVec<VarInt, PreviousMessage>,
    pub unsig_content : Option<Text>,
    pub filter, // TODO
    pub filter_bits, // TODO
    pub chat_type     : RegEntry<ChatType>,
    pub sender_name   : Text,
    pub target_name   : Option<Text>
}
#[packet_part]
pub struct PreviousMessage {
    // TODO
}*/


#[packet( prefix = 0x3A, bound = S2C, stage = Play )]
pub struct EndCombatS2CPacket {
    pub duration : VarInt
}


#[packet( prefix = 0x3B, bound = S2C, stage = Play )]
pub struct EnterCombatS2CPacket;


#[packet( prefix = 0x3C, bound = S2C, stage = Play )]
pub struct CombatDeathS2CPacket {
    /// Should match the client's entity id.
    pub entity  : VarInt,
    pub message : Text
}


#[packet( prefix = 0x3D, bound = S2C, stage = Play )]
pub struct PlayerInfoRemoveS2CPacket {
    pub uuids : LengthPrefixVec<VarInt, Uuid>
}


// TODO: PlayerInfoUpdateS2CPacket


#[packet( prefix = 0x3F, bound = S2C, stage = Play )]
pub struct LookAtS2CPacket {
    pub from   : LookAtPart,
    pub x      : f64,
    pub y      : f64,
    pub z      : f64,
    pub entity : Option<LookAtEntity>
}
#[packet_part(VarInt)]
pub enum LookAtPart {
    Feet = 0,
    Eyes = 1
}
#[packet_part]
pub struct LookAtEntity {
    pub entity : VarInt,
    pub at     : LookAtPart
}


#[packet( prefix = 0x40, bound = S2C, stage = Play )]
pub struct SyncPlayerPosS2CPacket {
    pub adx         : f64,
    pub ady         : f64,
    pub adz         : f64,
    pub adyaw_deg   : f32,
    pub adpitch_deg : f32,
    pub flags       : SyncPosFlags,
    pub transaction : VarInt
}
packet_flags!{ pub struct SyncPosFlags {
    pub x     : 0b00000001,
    pub y     : 0b00000010,
    pub z     : 0b00000100,
    pub pitch : 0b00001000,
    pub yaw   : 0b00010000
} }


/*#[packet( prefix = 0x41, bound = S2C, stage = Play )]
pub struct UpdateRecipeBookS2CPacket {
    pub action            : RecipeBookAction,
    pub craft_book_open   : bool,
    pub craft_book_filter : bool,
    pub smelt_book_open   : bool,
    pub smelt_book_filter : bool,
    pub blast_book_open   : bool,
    pub blast_book_filter : bool,
    pub smoke_book_open   : bool,
    pub smoke_book_filter : bool,
    // TODO
}
#[packet_part(VarInt)]
pub enum RecipeBookAction {
    Init   = 0,
    Add    = 1,
    Remove = 2
}*/


#[packet( prefix = 0x42, bound = S2C, stage = Play )]
pub struct RemoveEntitiesS2CPacket {
    pub entities : LengthPrefixVec<VarInt, VarInt>
}


#[packet( prefix = 0x43, bound = S2C, stage = Play )]
pub struct RemoveEntityEffectS2CPacket {
    pub entity : VarInt,
    pub effect : RegEntry<StatusEffect>
}


#[packet( prefix = 0x44, bound = S2C, stage = Play )]
pub struct ResetScoreS2CPacket {
    pub entity_name    : String,
    pub objective_name : Option<String>
}


#[packet( prefix = 0x45, bound = S2C, stage = Play )]
pub struct RemoveResourcePackS2CPacket {
    /// No uuid will remove all packs.
    pub uuid : Option<Uuid>
}


#[packet( prefix = 0x46, bound = S2C, stage = Play )]
pub struct AddResourcePackS2CPacket {
    pub uuid   : Uuid,
    pub url    : String,
    pub hash   : String,
    pub force  : bool,
    pub prompt : Option<Text>
}


#[packet( prefix = 0x47, bound = S2C, stage = Play )]
pub struct RespawnS2CPacket {
    pub dim             : RegEntry<DimType>,
    pub dim_name        : Identifier,
    /// Hashed
    pub seed            : u64,
    pub gamemode        : Gamemode,
    pub prev_gamemode   : Gamemode,
    pub is_debug        : bool,
    pub is_flat         : bool,
    pub death_loc       : Option<DeathLocation>,
    pub portal_cooldown : VarInt,
    pub data_kept       : u8
}


#[packet( prefix = 0x48, bound = S2C, stage = Play )]
pub struct SetHeadRotS2CPacket {
    pub entity : VarInt,
    pub yaw    : Angle
}


#[packet( prefix = 0x49, bound = S2C, stage = Play )]
pub struct UpdateSectionBlocksS2CPacket {
    pub chunk_section : ChunkSection,
    pub blocks        : LengthPrefixVec<VarInt, VarLong>
}


#[packet( prefix = 0x4A, bound = S2C, stage = Play )]
pub struct SelectAdvTabS2CPacket {
    pub tab : Option<Identifier>
}


#[packet( prefix = 0x4B, bound = S2C, stage = Play )]
pub struct ServerDataS2CPacket {
    pub motd : Text,
    pub icon : Option<LengthPrefixVec<VarInt, u8>>
}


#[packet( prefix = 0x4C, bound = S2C, stage = Play )]
pub struct ActionBarS2CPacket {
    pub text : Text
}


#[packet( prefix = 0x4D, bound = S2C, stage = Play )]
pub struct SetBorderCentreS2CPacket {
    pub x : f64,
    pub z : f64
}


#[packet( prefix = 0x4E, bound = S2C, stage = Play )]
pub struct SetBorderLerpSizeS2CPacket {
    pub old_diameter : f64,
    pub new_diameter : f64,
    /// In milliseconds
    pub transition   : VarLong
}


#[packet( prefix = 0x4F, bound = S2C, stage = Play )]
pub struct SetBorderSizeS2CPacket {
    pub diameter : f64
}


#[packet( prefix = 0x50, bound = S2C, stage = Play )]
pub struct SetBorderWarningDelayS2CPacket {
    pub warning_time : VarInt
}


#[packet( prefix = 0x51, bound = S2C, stage = Play )]
pub struct SetBorderWarningDistS2CPacket {
    /// In blocks
    pub warning_dist : VarInt
}


#[packet( prefix = 0x52, bound = S2C, stage = Play )]
pub struct SetCameraS2CPacket {
    /// Use the entity of the client to return control.
    pub entity : VarInt
}


#[packet( prefix = 0x53, bound = S2C, stage = Play )]
pub struct SetHotbarSlotS2CPacket {
    /// 0~8
    pub slot : u8
}


#[packet( prefix = 0x54, bound = S2C, stage = Play )]
pub struct SetChunkCentreS2CPacket {
    pub chunk_x : VarInt,
    pub chunk_z : VarInt
}


#[packet( prefix = 0x55, bound = S2C, stage = Play )]
pub struct SetRenderDistS2CPacket {
    /// 2~32
    pub view_dist : VarInt
}


#[packet( prefix = 0x56, bound = S2C, stage = Play )]
pub struct SetDefaultSpawnPositionS2CPacket {
    pub pos     : BlockPos,
    pub yaw_deg : f32
}


#[packet( prefix = 0x57, bound = S2C, stage = Play )]
pub struct DisplayObjectiveS2CPacket {
    pub to   : ObjectiveLocation,
    pub name : String
}
#[packet_part(VarInt)]
pub enum ObjectiveLocation {
    List      = 0,
    Sidebar   = 1,
    BelowName = 2,
    // TODO: Team-specific sidebars
}


/*#[packet( prefix = 0x58, bound = S2C, stage = Play )]
pub struct SetEntityMetaS2CPacket {
    pub entity : VarInt,
    pub meta   : // TODO
}*/


#[packet( prefix = 0x59, bound = S2C, stage = Play )]
pub struct LinkEntitiesS2CPacket {
    pub attached_entity : i32,
    /// Set to -1 to detach.
    pub holding_entity  : i32
}


/// 1/8000 of a block per tick.
#[packet( prefix = 0x5A, bound = S2C, stage = Play )]
pub struct SetEntityVelS2CPacket {
    pub entity : VarInt,
    pub vel_x  : u16,
    pub vel_y  : u16,
    pub vel_z  : u16
}


// TODO: SetEquipmentS2CPacket


#[packet( prefix = 0x5C, bound = S2C, stage = Play )]
pub struct SetXpS2CPacket {
    /// 0~1
    pub frac  : f32,
    pub level : VarInt,
    pub total : VarInt
}


#[packet( prefix = 0x5D, bound = S2C, stage = Play )]
pub struct SetHpS2CPacket {
    pub hp   : f32,
    /// 0~20
    pub food : VarInt,
    /// 0~5
    pub sat  : f32
}


#[packet( prefix = 0x5E, bound = S2C, stage = Play )]
pub struct UpdateObjectiveS2CPacket {
    pub name   : String,
    pub action : UpdateObjectiveAction
}
#[packet_part(u8)]
pub enum UpdateObjectiveAction {
    Create {
        value  : Text,
        kind   : ObjectiveKind,
        format : Option<NumberFormat>
    } = 0,
    Remove = 1,
    Update {
        value : Text,
        kind  : ObjectiveKind,
        format : Option<NumberFormat>
    } = 2
}
#[packet_part(VarInt)]
pub enum ObjectiveKind {
    Integer = 0,
    Hearts  = 1
}
#[packet_part(VarInt)]
pub enum NumberFormat {
    Blank  = 0,
    Styled {
        styling : Nbt
    } = 1,
    Fixed {
        content : Text
    } = 2
}


#[packet( prefix = 0x5F, bound = S2C, stage = Play )]
pub struct SetPassengersS2CPacket {
    pub entity     : VarInt,
    pub passengers : LengthPrefixVec<VarInt, VarInt>
}


#[packet( prefix = 0x60, bound = S2C, stage = Play )]
pub struct UpdateTeamS2CPacket {
    pub name   : String,
    pub action : UpdateTeamAction
}
#[packet_part(u8)]
pub enum UpdateTeamAction {
    Create {
        display_name   : Text,
        friendly_flags : TeamFriendlyFlags,
        /// `always`, `hideForOtherTeams`, `hideForOwnTeam`, or `never`
        name_tag_vis   : String,
        /// `always`, `pushOtherTeams`, `pushOwnTeam`, or `never`
        collision_rule : String,
        colour         : VanillaFormatting,
        prefix         : Text,
        suffix         : Text,
        entities       : LengthPrefixVec<VarInt, String>
    } = 0,
    Remove = 1,
    Update {
        display_name   : Text,
        friendly_flags : TeamFriendlyFlags,
        /// `always`, `hideForOtherTeams`, `hideForOwnTeam`, or `never`
        name_tag_vis   : String,
        /// `always`, `pushOtherTeams`, `pushOwnTeam`, or `never`
        collision_rule : String,
        colour         : VanillaFormatting,
        prefix         : Text,
        suffix         : Text
    } = 2,
    AddEntities {
        entities : LengthPrefixVec<VarInt, String>
    } = 3,
    RemoveEntities {
        entities : LengthPrefixVec<VarInt, String>
    } = 4
}
#[packet_part(VarInt)]
pub enum VanillaFormatting {
    Black         = 0,
    DarkBlue      = 1,
    DarkGreen     = 2,
    DarkAqua      = 3,
    DarkRed       = 4,
    DarkPurple    = 5,
    Gold          = 6,
    Grey          = 7,
    DarkGray      = 8,
    Blue          = 9,
    Green         = 10,
    Aqua          = 11,
    Red           = 12,
    Pink          = 13,
    Yellow        = 14,
    White         = 15,
    Obfuscate     = 16,
    Bold          = 17,
    Strikethrough = 18,
    Underline     = 19,
    Italic        = 20,
    Reset         = 21
}
packet_flags!{ pub struct TeamFriendlyFlags {
    pub friendly_fire  : 0b00000001,
    pub see_invisibles : 0b00000010
} }


#[packet( prefix = 0x61, bound = S2C, stage = Play )]
pub struct UpdateScoreS2CPacket {
    pub entity_name    : String,
    pub objective_name : String,
    pub value          : VarInt,
    pub display_name   : Option<Text>,
    pub number_format  : Option<NumberFormat>
}


#[packet( prefix = 0x62, bound = S2C, stage = Play )]
pub struct SetSimDistS2CPacket {
    pub sim_dist : VarInt
}


#[packet( prefix = 0x63, bound = S2C, stage = Play )]
pub struct SetSubtitleTextS2CPacket {
    pub subtitle : Text
}


#[packet( prefix = 0x64, bound = S2C, stage = Play )]
pub struct UpdateTimeS2CPacket {
    pub world_age : u64,
    pub day_time  : u64
}


#[packet( prefix = 0x65, bound = S2C, stage = Play )]
pub struct SetTitleTextS2CPacket {
    pub title : Text
}


#[packet( prefix = 0x66, bound = S2C, stage = Play )]
pub struct SetTitleTimesS2CPacket {
    pub fade_in  : u32,
    pub stay     : u32,
    pub fade_out : u32
}


#[packet( prefix = 0x67, bound = S2C, stage = Play )]
pub struct EntitySoundEffectS2CPacket {
    pub sound    : Sound,
    pub category : SoundCategory,
    pub entity   : VarInt,
    pub volume   : f32,
    pub pitch    : f32,
    pub seed     : u64
}
#[packet_part(VarInt)]
pub enum SoundCategory {
    Master  = 0,
    Music   = 1,
    Records = 2,
    Weather = 3,
    Blocks  = 4,
    Hostile = 5,
    Neutral = 6,
    Player  = 7,
    Ambient = 8,
    Voice   = 9
}


#[packet( prefix = 0x68, bound = S2C, stage = Play )]
pub struct SoundEffectS2CPacket {
    pub sound    : Sound,
    pub category : SoundCategory,
    pub entity   : VarInt,
    pub volume   : f32,
    pub pitch    : f32,
    pub seed     : u64
}


#[packet( prefix = 0x69, bound = S2C, stage = Play )]
pub struct StartConfigS2CPacket;


// TODO: StopSoundS2CPacket {}


#[packet( prefix = 0x6B, bound = S2C, stage = Play )]
pub struct StoreCookieS2CPacket {
    pub key     : Identifier,
    pub payload : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x6C, bound = S2C, stage = Play )]
pub struct SystemChatMessageS2CPacket {
    pub content      : Text,
    pub is_actionbar : bool
}


#[packet( prefix = 0x6D, bound = S2C, stage = Play )]
pub struct SetTabListHeaderS2CPacket {
    pub header : Text,
    pub footer : Text
}


#[packet( prefix = 0x6E, bound = S2C, stage = Play )]
pub struct TagQueryResponseS2CPacket {
    pub transaction : VarInt,
    pub nbt         : Nbt
}


#[packet( prefix = 0x6F, bound = S2C, stage = Play )]
pub struct PickupItemS2CPacket {
    pub collected_entity : VarInt,
    pub collector_entity : VarInt,
    pub item_count       : VarInt
}


#[packet( prefix = 0x70, bound = S2C, stage = Play )]
pub struct TeleportEntityS2CPacket {
    pub entity : VarInt,
    pub x      : f64,
    pub y      : f64,
    pub z      : f64,
    pub yaw    : Angle,
    pub pitch  : Angle,
    pub ground : bool
}


#[packet( prefix = 0x71, bound = S2C, stage = Play )]
pub struct SetTickingS2CPacket {
    pub tick_rate : f32,
    pub frozen    : bool
}


#[packet( prefix = 0x72, bound = S2C, stage = Play )]
pub struct StepTickS2CPacket {
    pub steps : VarInt
}


#[packet( prefix = 0x73, bound = S2C, stage = Play )]
pub struct TransferS2CPacket {
    pub host : String,
    pub port : VarInt
}


/*#[packet( prefix = 0x74, bound = S2C, stage = Play )]
pub struct UpdateAdvsS2CPacket {
    pub clear_current : bool,
    pub advs          : LengthPrefixHashMap<VarInt, Identifier, Advancement>,
    pub removing      : LengthPrefixVec<VarInt, Identifier>,
    pub progress      : LengthPrefixHashMap<VarInt, Identifier, LengthPrefixHashMap<VarInt, Identifier, AdvCriterionProgress>>
}
#[packet_part]
pub struct Advancement {
    pub has_parent : bool,
    pub parent     : Option<Identifier>,
    pub display    : Option<AdvDisplay>,
    pub reqs       : LengthPrefixVec<VarInt, String>,
    pub telemetry  : bool
}
#[packet_part]
pub struct AdvDisplay {
    pub title      : Text,
    pub desc       : Text,
    pub icon       : // TODO,
    pub frame      : AdvFrame,
    pub flags      : u32, // TODO packet_flags!
    // TODO
    pub x_coord : f32,
    pub y_coord : f32
}
#[packet_part(VarInt)]
pub enum AdvFrame {
    Task      = 0,
    Challenge = 1,
    Goal      = 2
}
#[packet_part]
pub struct AdvCriterionProgress {
    pub achieved : bool,
    pub date     : Option<u64>
}*/


#[packet( prefix = 0x75, bound = S2C, stage = Play )]
pub struct UpdateAttrsS2CPacket {
    pub entity     : VarInt,
    pub properties : LengthPrefixVec<VarInt, Attribute>
}
#[packet_part]
pub struct Attribute {
    pub id    : RegEntry<AttributeType>,
    pub value : f64,
    pub mods  : LengthPrefixVec<VarInt, AttrModifier>
}
#[packet_part]
pub struct AttrModifier {
    pub id     : Identifier,
    pub amount : f64,
    pub op     : AttrModOp
}
#[packet_part(u8)]
pub enum AttrModOp {
    Add        = 0,
    AddPercent = 1,
    MulPercent = 2
}


#[packet( prefix = 0x76, bound = S2C, stage = Play )]
pub struct EntityEffectS2CPacket {
    pub entity : VarInt,
    pub effect : RegEntry<StatusEffect>,
    pub amp    : VarInt,
    /// -1 for infinite
    pub dur    : VarInt,
    pub flags  : EffectFlags
}
packet_flags!{ pub struct EffectFlags {
    pub ambient   : 0b00000001,
    pub particles : 0b00000010,
    pub icon      : 0b00000100,
    pub blend     : 0b00001000
} }


/*#[packet( prefix = 0x77, bound = S2C, stage = Play )]
pub struct UpdateRecipesS2CPacket {
    pub recipes : LengthPrefixVec<VarInt, Recipe>
}
#[packet_part]
pub struct Recipe {
    pub id   : Identifier,
    // TODO
}*/


// TODO: UpdateTagsS2CPacket


#[packet( prefix = 0x79, bound = S2C, stage = Play )]
pub struct ProjPowerS2CPacket {
    pub entity : VarInt,
    pub power  : f64
}


#[packet( prefix = 0x7A, bound = S2C, stage = Play )]
pub struct CustomReportDetailsS2CPacket {
    pub details : LengthPrefixVec<VarInt, ReportDetail>
}


#[packet( prefix = 0x7B, bound = S2C, stage = Play )]
pub struct ServerLinksS2CPacket {
    pub links : LengthPrefixVec<VarInt, ServerLink>
}


packet_full_decode!( bound = S2C, stage = Play );
