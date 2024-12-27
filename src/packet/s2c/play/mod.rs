use crate::packet::s2c::config::{ReportDetail, ServerLink};
use super::*;
//use config::{ ReportDetail, ServerLink };


#[packet( "minecraft:s2c/play/bundle_delimiter" )]
pub struct BundleDelimiterS2CPlayPacket;

#[packet( "minecraft:s2c/play/add_entity" )]
pub struct AddEntityS2CPlayPacket {
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


#[packet( "minecraft:s2c/play/add_experience_orb" )]
pub struct AddExperienceOrbS2CPlayPacket {
    pub id    : VarInt,
    pub x     : f64,
    pub y     : f64,
    pub z     : f64,
    pub count : i16
}


#[packet( "minecraft:s2c/play/animate" )]
pub struct AnimateS2CPlayPacket {
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

#[packet( "minecraft:s2c/play/award_stats" )]
pub struct AwardStatsS2CPlayPacket {
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

#[packet( "minecraft:s2c/play/block_changed_ack" )]
pub struct BlockChangedAckS2CPlayPacket(pub VarInt);


#[packet( "minecraft:s2c/play/block_destruction" )]
pub struct BlockDestructionS2CPlayPacket {
    pub entity_id : VarInt,
    pub pos       : BlockPos,
    pub stage     : u8
}


#[packet( "minecraft:s2c/play/block_entity_data" )]
pub struct BlockEntityDataS2CPlayPacket {
    pub pos  : BlockPos,
    pub kind : VarInt,
    pub data : Nbt
}


#[packet( "minecraft:s2c/play/block_event" )]
pub struct BlockEventS2CPlayPacket {
    pub pos    : BlockPos,
    pub action : u8,
    pub param  : u8,
    pub block  : RegEntry<Block>
}


#[packet( "minecraft:s2c/play/block_update" )]
pub struct BlockUpdateS2CPlayPacket {
    pub pos   : BlockPos,
    pub block : RegEntry<BlockState>
}


#[packet( "minecraft:s2c/play/boss_event" )]
pub struct BossEventS2CPlayPacket {
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
packet_flags!{ pub struct BossBarFlags<u8> {
    pub darken_sky  : 0b00000001,
    pub end_music   : 0b00000010,
    pub thicken_fog : 0b00000100
} }



#[packet( "minecraft:s2c/play/change_difficulty" )]
pub struct ChangeDifficultyS2CPlayPacket {
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


#[packet( "minecraft:s2c/play/chunk_batch_finished" )]
pub struct ChunkBatchFinishedS2CPlayPacket {
    pub size : VarInt
}


#[packet( "minecraft:s2c/play/chunk_batch_start" )]
pub struct ChunkBatchStartS2CPlayPacket;


#[packet( "minecraft:s2c/play/chunks_biomes" )]
pub struct ChunksBiomesS2CPlayPacket {
    pub chunks : LengthPrefixVec<VarInt, ChunkBiomeData>
}
#[packet_part]
pub struct ChunkBiomeData {
    pub chunk_x : i32,
    pub chunk_z : i32,
    pub data    : LengthPrefixVec<VarInt, u8>
}



#[packet( "minecraft:s2c/play/clear_titles" )]
pub struct ClearTitlesS2CPlayPacket {
    pub reset : bool
}


#[packet( "minecraft:s2c/play/command_suggestions" )]
pub struct CommandSuggestionsS2CPlayPacket {
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


#[packet( "minecraft:s2c/play/commands" )]
pub struct CommandsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/container_close" )]
pub struct ContainerCloseS2CPlayPacket {
    pub window : VarInt
}


#[packet( "minecraft:s2c/play/container_set_content" )]
pub struct ContainerSetContentS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/container_set_data" )]
pub struct ContainerSetDataS2CPlayPacket {
    pub window : VarInt,
    pub prop   : u16,
    pub value  : u16
}


#[packet( "minecraft:s2c/play/container_set_slot" )]
pub struct ContainerSetSlotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/cookie_request" )]
pub struct CookieRequestS2CPlayPacket {
    pub key : Identifier
}



#[packet( "minecraft:s2c/play/cooldown" )]
pub struct CooldownS2CPlayPacket {
    pub item  : Identifier,
    pub ticks : VarInt
}

#[packet( "minecraft:s2c/play/custom_chat_completions" )]
pub struct CustomChatCompletionsS2CPlayPacket {
    pub action : ChatSuggestionAction
}
#[packet_part(VarInt)]
pub enum ChatSuggestionAction {
    Add    = 0,
    Remove = 1,
    Set    = 2
}



#[packet( "minecraft:s2c/play/custom_payload" )]
pub struct CustomPayloadS2CPlayPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( "minecraft:s2c/play/damage_event" )]
pub struct DamageEventS2CPlayPacket {
    pub entity               : VarInt,
    pub source_type          : RegEntry<DamageType>,
    pub source_cause_entity  : OptionVarInt,
    pub source_direct_entity : OptionVarInt,
    pub source_pos           : Option<Vec3d>
}



#[packet( "minecraft:s2c/play/debug_sample" )]
pub struct DebugSampleS2CPlayPacket {
    pub sample : LengthPrefixVec<VarInt, u64>,
    pub kind   : DebugSampleKind
}
#[packet_part(VarInt)]
pub enum DebugSampleKind {
    TickTime = 0
}


#[packet( "minecraft:s2c/play/delete_chat" )]
pub struct DeleteChatS2CPlayPacket {
    pub msg       : VarInt,
    pub signature : Option<[u8; 256]>
}


#[packet( "minecraft:s2c/play/disconnect" )]
pub struct DisconnectS2CPlayPacket {
    pub reason : NbtText
}


#[packet( "minecraft:s2c/play/disguised_chat" )]
pub struct DisguisedChatS2CPlayPacket {
    pub msg       : Text,
    pub chat_type : RegEntry<ChatType>
}


#[packet( "minecraft:s2c/play/entity_event" )]
pub struct EntityEventS2CPlayPacket {
    pub entity : i32,
    pub status : u8
}


#[packet( "minecraft:s2c/play/entity_position_sync" )]
pub struct EntityPositionSyncS2CPlayPacket {
    entity_id: VarInt,
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool
}


#[packet( "minecraft:s2c/play/explode" )]
pub struct ExplodeS2CPlayPacket(TODO);

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

#[packet( "minecraft:s2c/play/forget_world_chunk" )]
pub struct ForgetWorldChunkS2CPlayPacket  {
    pub chunk_z : i32,
    pub chunk_x : i32
}


#[packet( "minecraft:s2c/play/game_event" )]
pub struct GameEventS2CPlayPacket  {
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



#[packet( "minecraft:s2c/play/horse_screen_open" )]
pub struct HorseScreenOpenS2CPlayPacket  {
    pub window : VarInt,
    pub slots  : VarInt,
    pub entity : i32
}



#[packet( "minecraft:s2c/play/hurt_animation" )]
pub struct HurtAnimationS2CPlayPacket  {
    pub entity : VarInt,
    pub dyaw   : f32
}


#[packet( "minecraft:s2c/play/initialise_border" )]
pub struct InitialiseBorderS2CPlayPacket  {
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



#[packet( "minecraft:s2c/play/keep_alive" )]
pub struct KeepAliveS2CPlayPacket(pub u64);


#[packet( "minecraft:s2c/play/world_chunk_with_light" )]
pub struct WorldChunkWithLightS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/world_event" )]
pub struct WorldEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/world_particles" )]
pub struct WorldParticlesS2CPlayPacket  {
    pub long_distance : bool,
    pub always_visible: bool,
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



#[packet( "minecraft:s2c/play/light_update" )]
pub struct LightUpdateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/login" )]
pub struct LoginS2CPlayPacket  {
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
    pub sea_level            : VarInt,
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




#[packet( "minecraft:s2c/play/map_item_data" )]
pub struct MapItemDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/merchant_offers" )]
pub struct MerchantOffersS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_entity_pos" )]
pub struct MoveEntityPosS2CPlayPacket {
    pub entity : VarInt,
    pub dx     : i16,
    pub dy     : i16,
    pub dz     : i16,
    pub ground : bool
}



#[packet( "minecraft:s2c/play/move_entity_pos_rot" )]
pub struct MoveEntityPosRotS2CPlayPacket {
    pub entity : VarInt,
    pub yaw    : Angle,
    pub pitch  : Angle,
    pub ground : bool
}


#[packet( "minecraft:s2c/play/move_minecart_along_track" )]
pub struct MoveMinecartAlongTrackS2CPlayPacket {
    pub entity_id: VarInt,
    pub steps: LengthPrefixVec<VarInt, MinecartStep>
}

#[packet_part]
pub struct MinecartStep {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub weight: f32,
}


#[packet( "minecraft:s2c/play/move_entity_rot" )]
pub struct MoveEntityRotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_vehicle" )]
pub struct MoveVehicleS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/open_book" )]
pub struct OpenBookS2CPlayPacket {
    pub hand : Hand
}
#[packet_part(VarInt)]
pub enum Hand {
    Mainhand = 0,
    Offhand  = 1
}




#[packet( "minecraft:s2c/play/open_screen" )]
pub struct OpenScreenS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/open_sign_editor" )]
pub struct OpenSignEditorS2CPlayPacket {
    pub pos   : BlockPos,
    /// False means back
    pub front : bool
}



#[packet( "minecraft:s2c/play/ping" )]
pub struct PingS2CPlayPacket(pub u32);


#[packet( "minecraft:s2c/play/pong_response" )]
pub struct PongResponseS2CPlayPacket(pub u64);


#[packet( "minecraft:s2c/play/place_ghost_recipe" )]
pub struct PlaceGhostRecipeS2CPlayPacket {
    pub window : u8,
    pub recipe : Identifier
}


#[packet( "minecraft:s2c/play/player_abilities" )]
pub struct PlayerAbilitiesS2CPlayPacket {
    pub flags     : PlayerAbilityFlags,
    /// Default is 0.05
    pub fly_speed : f32,
    pub fov_fac   : f32
}
packet_flags!{ pub struct PlayerAbilityFlags<u8> {
    pub invul      : 0b00000001,
    pub flying     : 0b00000010,
    pub allow_fly  : 0b00000100,
    pub instabreak : 0b00001000
} }


#[packet( "minecraft:s2c/play/player_chat" )]
pub struct PlayerChatS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_combat_end" )]
pub struct PlayerCombatEndS2CPlayPacket {
    pub duration : VarInt
}



#[packet( "minecraft:s2c/play/player_combat_enter" )]
pub struct PlayerCombatEnterS2CPlayPacket;


#[packet( "minecraft:s2c/play/player_combat_kill" )]
pub struct PlayerCombatKillS2CPlayPacket {
    /// Should match the client's entity id.
    pub entity  : VarInt,
    pub message : Text
}


#[packet( "minecraft:s2c/play/player_info_remove" )]
pub struct PlayerInfoRemoveS2CPlayPacket {
    pub uuids : LengthPrefixVec<VarInt, Uuid>
}


#[packet( "minecraft:s2c/play/player_info_update" )]
pub struct PlayerInfoUpdateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_look_at" )]
pub struct PlayerLookAtS2CPlayPacket {
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



#[packet( "minecraft:s2c/play/player_position" )]
pub struct PlayerPositionS2CPlayPacket {
    pub teleport_id : VarInt,
    pub x           : f64,
    pub y           : f64,
    pub z           : f64,
    pub vx          : f64,
    pub vy          : f64,
    pub vz          : f64,
    pub adyaw_deg   : f32,
    pub adpitch_deg : f32,
    pub flags       : TeleportFlags,
    pub transaction : VarInt
}

packet_flags! {
    pub struct TeleportFlags<i32> {
        pub relative_x : 0b1,
        pub relative_y : 0b10,
        pub relative_z : 0b100,
        pub relative_pitch : 0b1000,
        pub relative_yaw : 0b10000,
        pub relative_vx : 0b100000,
        pub relative_vy : 0b1000000,
        pub relative_vz : 0b10000000,
        pub rotate_velocity : 0b100000000,
    }
}

#[packet( "minecraft:s2c/play/player_rotation" )]
pub struct PlayerRotationS2CPlayPacket {
    yaw: f64,
    pitch: f64
}


#[packet( "minecraft:s2c/play/recipe_book_add" )]
pub struct RecipeBookAddS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/recipe_book_remove" )]
pub struct RecipeBookRemoveS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/recipe_book_settings" )]
pub struct RecipeBookSettingsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/remove_entities" )]
pub struct RemoveEntitiesS2CPlayPacket {
    pub entities : LengthPrefixVec<VarInt, VarInt>
}


#[packet( "minecraft:s2c/play/remove_mob_effect" )]
pub struct RemoveMobEffectS2CPlayPacket {
    pub entity : VarInt,
    pub effect : RegEntry<StatusEffect>
}


#[packet( "minecraft:s2c/play/reset_score" )]
pub struct ResetScoreS2CPlayPacket {
    pub entity_name    : String,
    pub objective_name : Option<String>
}


#[packet( "minecraft:s2c/play/resource_pack_pop" )]
pub struct ResourcePackPopS2CPlayPacket {
    /// No uuid will remove all packs.
    pub uuid : Option<Uuid>
}


#[packet( "minecraft:s2c/play/resource_pack_push" )]
pub struct ResourcePackPushS2CPlayPacket {
    pub uuid   : Uuid,
    pub url    : String,
    pub hash   : String,
    pub force  : bool,
    pub prompt : Option<Text>
}


#[packet( "minecraft:s2c/play/respawn" )]
pub struct RespawnS2CPlayPacket {
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
    pub sea_level       : VarInt,
    pub data_kept       : u8
}


#[packet( "minecraft:s2c/play/rotate_head" )]
pub struct RotateHeadS2CPlayPacket {
    pub entity : VarInt,
    pub yaw    : Angle
}


#[packet( "minecraft:s2c/play/section_blocks_update" )]
pub struct SectionBlocksUpdateS2CPlayPacket {
    pub chunk_section : ChunkSection,
    pub blocks        : LengthPrefixVec<VarInt, VarLong>
}


#[packet( "minecraft:s2c/play/select_advancements_tab" )]
pub struct SelectAdvancementsTabS2CPlayPacket {
    pub tab : Option<Identifier>
}


#[packet( "minecraft:s2c/play/server_data" )]
pub struct ServerDataS2CPlayPacket {
    pub motd : Text,
    pub icon : Option<LengthPrefixVec<VarInt, u8>>
}


#[packet( "minecraft:s2c/play/set_action_bar_text" )]
pub struct SetActionBarTextS2CPlayPacket {
    pub text : Text
}


#[packet( "minecraft:s2c/play/set_border_centre" )]
pub struct SetBorderCentreS2CPlayPacket {
    pub x : f64,
    pub z : f64
}


#[packet( "minecraft:s2c/play/set_border_lerp_size" )]
pub struct SetBorderLerpSizeS2CPlayPacket {
    pub old_diameter : f64,
    pub new_diameter : f64,
    /// In milliseconds
    pub transition   : VarLong
}


#[packet( "minecraft:s2c/play/set_border_size" )]
pub struct SetBorderSizeS2CPlayPacket {
    pub diameter : f64
}



#[packet( "minecraft:s2c/play/set_border_warning_delay" )]
pub struct SetBorderWarningDelayS2CPlayPacket {
    pub warning_time : VarInt
}


#[packet( "minecraft:s2c/play/set_border_warning_distance" )]
pub struct SetBorderWarningDistanceS2CPlayPacket {
    /// In blocks
    pub warning_dist : VarInt
}



#[packet( "minecraft:s2c/play/set_camera" )]
pub struct SetCameraS2CPlayPacket {
    /// Use the entity of the client to return control.
    pub entity : VarInt
}


#[packet( "minecraft:s2c/play/set_chunk_cache_centre" )]
pub struct SetChunkCacheCentreS2CPlayPacket {
    pub chunk_x : VarInt,
    pub chunk_z : VarInt
}


#[packet( "minecraft:s2c/play/set_chunk_cache_radius" )]
pub struct SetChunkCacheRadiusS2CPlayPacket {
    /// 2~32
    pub view_dist : VarInt
}


#[packet( "minecraft:s2c/play/set_cursor_item" )]
pub struct SetCursorItemS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_default_spawn_position" )]
pub struct SetDefaultSpawnPositionS2CPlayPacket {
    pub pos     : BlockPos,
    pub yaw_deg : f32
}


#[packet( "minecraft:s2c/play/set_display_objective" )]
pub struct SetDisplayObjectiveS2CPlayPacket {
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

#[packet( "minecraft:s2c/play/set_entity_data" )]
pub struct SetEntityDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_entity_link" )]
pub struct SetEntityLinkS2CPlayPacket {
    pub attached_entity : i32,
    /// Set to -1 to detach.
    pub holding_entity  : i32
}


#[packet( "minecraft:s2c/play/set_entity_motion" )]
pub struct SetEntityMotionS2CPlayPacket {
    pub entity : VarInt,
    pub vel_x  : u16,
    pub vel_y  : u16,
    pub vel_z  : u16
}


#[packet( "minecraft:s2c/play/set_equipment" )]
pub struct SetEquipmentS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_experience" )]
pub struct SetExperienceS2CPlayPacket {
    /// 0~1
    pub frac  : f32,
    pub level : VarInt,
    pub total : VarInt
}


#[packet( "minecraft:s2c/play/set_health" )]
pub struct SetHealthS2CPlayPacket {
    pub hp   : f32,
    /// 0~20
    pub food : VarInt,
    /// 0~5
    pub sat  : f32
}


#[packet( "minecraft:s2c/play/set_held_slot" )]
pub struct SetHeldSlotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_objective" )]
pub struct SetObjectiveS2CPlayPacket {
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



#[packet( "minecraft:s2c/play/set_passengers" )]
pub struct SetPassengersS2CPlayPacket {
    pub entity     : VarInt,
    pub passengers : LengthPrefixVec<VarInt, VarInt>
}


#[packet( "minecraft:s2c/play/set_player_inventory" )]
pub struct SetPlayerInventoryS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_player_team" )]
pub struct SetPlayerTeamS2CPlayPacket {
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
packet_flags!{ pub struct TeamFriendlyFlags<u8> {
    pub friendly_fire  : 0b00000001,
    pub see_invisibles : 0b00000010
} }



#[packet( "minecraft:s2c/play/set_score" )]
pub struct SetScoreS2CPlayPacket {
    pub entity_name    : String,
    pub objective_name : String,
    pub value          : VarInt,
    pub display_name   : Option<Text>,
    pub number_format  : Option<NumberFormat>
}


#[packet( "minecraft:s2c/play/set_simulation_distance" )]
pub struct SetSimulationDistanceS2CPlayPacket {
    pub sim_dist : VarInt
}


#[packet( "minecraft:s2c/play/set_subtitle_text" )]
pub struct SetSubtitleTextS2CPlayPacket {
    pub subtitle : Text
}


#[packet( "minecraft:s2c/play/set_time" )]
pub struct SetTimeS2CPlayPacket {
    pub world_age              : u64,
    pub day_time               : u64,
    pub time_of_day_increasing : bool,
}


#[packet( "minecraft:s2c/play/set_title_text" )]
pub struct SetTitleTextS2CPlayPacket {
    pub title : Text
}



#[packet( "minecraft:s2c/play/set_titles_animation" )]
pub struct SetTitlesAnimationS2CPlayPacket {
    pub fade_in  : u32,
    pub stay     : u32,
    pub fade_out : u32
}


#[packet( "minecraft:s2c/play/sound_entity" )]
pub struct SoundEntityS2CPlayPacket {
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


#[packet( "minecraft:s2c/play/sound" )]
pub struct SoundS2CPlayPacket {
    pub sound    : Sound,
    pub category : SoundCategory,
    pub entity   : VarInt,
    pub volume   : f32,
    pub pitch    : f32,
    pub seed     : u64
}



#[packet( "minecraft:s2c/play/start_configuration" )]
pub struct StartConfigurationS2CPlayPacket;


#[packet( "minecraft:s2c/play/stop_sound" )]
pub struct StopSoundS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/store_cookie" )]
pub struct StoreCookieS2CPlayPacket {
    pub key     : Identifier,
    pub payload : LengthPrefixVec<VarInt, u8>
}


#[packet( "minecraft:s2c/play/system_chat" )]
pub struct SystemChatS2CPlayPacket {
    pub content      : Text,
    pub is_actionbar : bool
}


#[packet( "minecraft:s2c/play/tab_list" )]
pub struct TabListS2CPlayPacket {
    pub header : Text,
    pub footer : Text
}


#[packet( "minecraft:s2c/play/tag_query" )]
pub struct TagQueryS2CPlayPacket {
    pub transaction : VarInt,
    pub nbt         : Nbt
}


#[packet( "minecraft:s2c/play/take_item_entity" )]
pub struct TakeItemEntityS2CPlayPacket {
    pub collected_entity : VarInt,
    pub collector_entity : VarInt,
    pub item_count       : VarInt
}


#[packet( "minecraft:s2c/play/teleport_entity" )]
pub struct TeleportEntityS2CPlayPacket {
    pub entity : VarInt,
    pub x      : f64,
    pub y      : f64,
    pub z      : f64,
    pub vx     : f64,
    pub vy     : f64,
    pub vz     : f64,
    pub yaw    : f32,
    pub pitch  : f32,
    pub flags  : TeleportFlags,
    pub on_g   : bool
}

#[packet( "minecraft:s2c/play/ticking_state" )]
pub struct TickingStateS2CPlayPacket {
    pub tick_rate : f32,
    pub frozen    : bool
}


#[packet( "minecraft:s2c/play/ticking_step" )]
pub struct TickingStepS2CPlayPacket {
    pub steps : VarInt
}


#[packet( "minecraft:s2c/play/transfer" )]
pub struct TransferS2CPlayPacket {
    pub host : String,
    pub port : VarInt
}


#[packet( "minecraft:s2c/play/update_advancements" )]
pub struct UpdateAdvancementsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_attributes" )]
pub struct UpdateAttributesS2CPlayPacket {
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


#[packet( "minecraft:s2c/play/update_mob_effect" )]
pub struct UpdateMobEffectS2CPlayPacket {
    pub entity : VarInt,
    pub effect : RegEntry<StatusEffect>,
    pub amp    : VarInt,
    /// -1 for infinite
    pub dur    : VarInt,
    pub flags  : EffectFlags
}
packet_flags!{ pub struct EffectFlags<u8> {
    pub ambient   : 0b00000001,
    pub particles : 0b00000010,
    pub icon      : 0b00000100,
    pub blend     : 0b00001000
} }



#[packet( "minecraft:s2c/play/update_recipes" )]
pub struct UpdateRecipesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_tags" )]
pub struct UpdateTagsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/projectile_power" )]
pub struct ProjectilePowerS2CPlayPacket {
    pub entity : VarInt,
    pub power  : f64
}



#[packet( "minecraft:s2c/play/custom_report_details" )]
pub struct CustomReportDetailsS2CPlayPacket {
    pub details : LengthPrefixVec<VarInt, ReportDetail>
}


#[packet( "minecraft:s2c/play/server_links" )]
pub struct ServerLinksS2CPlayPacket {
    pub links : LengthPrefixVec<VarInt, ServerLink>
}


packet_full_decode!{ S2C Play }
