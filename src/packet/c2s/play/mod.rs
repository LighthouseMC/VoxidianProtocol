use super::*;
use config::{ ClientInfo, ResourcePackStatus };
use s2c::play::{ DebugSampleKind, Difficulty, Hand };


#[packet( prefix = 0x00, bound = C2S, stage = Play )]
pub struct ConfirmTeleportC2SPacket {
    pub transaction : VarInt
}


#[packet( prefix = 0x01, bound = C2S, stage = Play )]
pub struct QueryBlockEntityTagC2SPacket {
    pub transaction : VarInt,
    pub pos         : BlockPos
}


#[packet( prefix = 0x02, bound = C2S, stage = Play )]
pub struct ChangeDifficultyC2SPacket {
    pub difficulty : Difficulty
}


#[packet( prefix = 0x03, bound = C2S, stage = Play )]
pub struct AcknowledgeMessageC2SPacket {
    pub count : VarInt
}


#[packet( prefix = 0x04, bound = C2S, stage = Play )]
pub struct ChatCommandC2SPacket {
    pub command : String
}


// TODO: SignedChatCommandC2SPacket


// TODO: ChatMessageC2SPacket


#[packet( prefix = 0x07, bound = C2S, stage = Play )]
pub struct PlayerSessionC2SPacket {
    pub session_id : Uuid,
    // In epoch milliseconds
    pub expires    : u64,
    pub public_key : LengthPrefixVec<VarInt, u8>,
    pub key_sig    : LengthPrefixVec<VarInt, u8>
}


#[packet( prefix = 0x08, bound = C2S, stage = Play )]
pub struct ChunkBatchReceivedC2SPacket {
    pub chunks_per_tick : f32
}


#[packet( prefix = 0x09, bound = C2S, stage = Play )]
pub struct ClientStatusC2SPacket {
    pub action : ClientStatusAction
}
#[packet_part(VarInt)]
pub enum ClientStatusAction {
    Respawn      = 0,
    RequestStats = 1
}


#[packet( prefix = 0x0A, bound = C2S, stage = Play )]
pub struct ClientInfoC2SPacket {
    pub info : ClientInfo
}


#[packet( prefix = 0x0B, bound = C2S, stage = Play )]
pub struct CommandSuggestionsRequestC2SPacket {
    pub transaction    : VarInt,
    pub left_of_cursor : String
}


#[packet( prefix = 0x0C, bound = C2S, stage = Play )]
pub struct AcknowledgeConfigC2SPacket;


#[packet( prefix = 0x0D, bound = C2S, stage = Play )]
pub struct ClickContainerButtonC2SPacket {
    pub window : u8,
    pub button : u8
}


/*#[packet( prefix = 0x0E, bound = C2S, stage = Play )]
pub struct ClickContainerSlotC2SPacket {
    /// 0 is player inventory
    pub window  : u8,
    pub state   : VarInt,
    pub slot    : u16,
    pub button  : u8,
    pub mode    : VarInt,
    pub changes : LengthPrefixVec<VarInt, SlotChange>
}
pub struct SlotChange {
    pub slot : u16,
    /// This is what the client thinks. Do not assume it's valid.
    pub data : // TODO
}*/


#[packet( prefix = 0x0F, bound = C2S, stage = Play )]
pub struct CloseContainerC2SPacket {
    /// 0 for player inventory
    pub window : u8
}


/// Used by crafters to lock and unlock slots.
#[packet( prefix = 0x10, bound = C2S, stage = Play )]
pub struct ChangeContainerSlotStateC2SPacket {
    pub slot   : VarInt,
    pub window : VarInt,
    pub state  : bool
}


#[packet( prefix = 0x11, bound = C2S, stage = Play )]
pub struct CookieResponseC2SPacket {
    pub key     : Identifier,
    pub payload : Option<LengthPrefixVec<VarInt, u8>>
}


#[packet( prefix = 0x12, bound = C2S, stage = Play )]
pub struct PluginMessageC2SPacket {
    pub channel : Identifier,
    pub data    : ConsumeAllVec<u8>
}


#[packet( prefix = 0x13, bound = C2S, stage = Play )]
pub struct DebugSampleSubscriptionC2SPacket {
    pub kind : DebugSampleKind
}


#[packet( prefix = 0x14, bound = C2S, stage = Play )]
pub struct EditBookC2SPacket {
    pub slot    : VarInt,
    pub entries : LengthPrefixVec<VarInt, String>,
    /// If this field is `Some`, the book is being signed.
    pub title   : Option<String>
}


#[packet( prefix = 0x15, bound = C2S, stage = Play )]
pub struct QueryEntityTagC2SPacket {
    pub transaction : VarInt,
    pub entity      : VarInt
}


#[packet( prefix = 0x16, bound = C2S, stage = Play )]
pub struct InteractEntityC2SPacket {
    pub entity : VarInt,
    pub kind   : InteractEntityKind,
    pub hand   : Option<Hand>,
    pub sneak  : bool
}
#[packet_part(VarInt)]
pub enum InteractEntityKind {
    Interact = 0,
    Attack   = 1,
    InteractAt {
        x : f32,
        y : f32,
        z : f32
    } = 2
}


#[packet( prefix = 0x17, bound = C2S, stage = Play )]
pub struct JigsawGenC2SPacket {
    pub pos          : BlockPos,
    pub levels       : VarInt,
    pub keep_jigsaws : bool
}


#[packet( prefix = 0x18, bound = C2S, stage = Play )]
pub struct KeepAliveC2SPacket(pub u64);


#[packet( prefix = 0x19, bound = C2S, stage = Play )]
pub struct LockDifficultyC2SPacket {
    pub lockde : bool
}


#[packet( prefix = 0x1A, bound = C2S, stage = Play )]
pub struct SetPlayerPosC2SPacket {
    pub x      : f64,
    pub feet_y : f64,
    pub z      : f64,
    pub ground : bool
}


#[packet( prefix = 0x1B, bound = C2S, stage = Play )]
pub struct SetPlayerPosAndRotC2SPacket {
    pub x         : f64,
    pub feet_y    : f64,
    pub z         : f64,
    pub yaw_deg   : f32,
    pub pitch_deg : f32,
    pub ground    : bool
}


#[packet( prefix = 0x1C, bound = C2S, stage = Play )]
pub struct SetPlayerRotC2SPacket {
    pub yaw_deg   : f32,
    pub pitch_deg : f32,
    pub ground    : bool
}


#[packet( prefix = 0x1D, bound = C2S, stage = Play )]
pub struct SetPlayerOnGroundC2SPacket {
    pub ground : bool
}


#[packet( prefix = 0x1E, bound = C2S, stage = Play )]
pub struct MoveVehicleC2SPacket {
    pub x         : f64,
    pub y         : f64,
    pub z         : f64,
    pub yaw_deg   : f32,
    pub pitch_deg : f32,
}


#[packet( prefix = 0x1F, bound = C2S, stage = Play )]
pub struct PaddleBoatC2SPacket {
    pub paddle_left  : bool,
    pub paddle_right : bool
}


#[packet( prefix = 0x20, bound = C2S, stage = Play )]
pub struct PickItemC2SPacket {
    pub slot : VarInt
}


#[packet( prefix = 0x21, bound = C2S, stage = Play )]
pub struct PingRequestC2SPacket(pub u64);


#[packet( prefix = 0x22, bound = C2S, stage = Play )]
pub struct PlaceRecipeC2SPacket {
    pub window   : u8,
    pub recipe   : Identifier,
    pub make_all : bool
}


#[packet( prefix = 0x23, bound = C2S, stage = Play )]
pub struct PlayerAbilitiesC2SPacket {
    pub flags : u8
}


#[packet( prefix = 0x24, bound = C2S, stage = Play )]
pub struct PlayerActionC2SPacket {
    pub action : PlayerAction,
    pub pos    : BlockPos,
    pub face   : BlockFace,
    pub seq    : VarInt
}
#[packet_part(VarInt)]
pub enum PlayerAction {
    StartDigging           = 0,
    CancelDigging          = 1,
    FinishDigging          = 2,
    DropItemStack          = 3,
    DropItem               = 4,
    ShootArrowFinishEating = 5,
    SwapHands              = 6
}
#[packet_part(u8)]
pub enum BlockFace {
    Bottom = 0,
    Top    = 1,
    North  = 2,
    South  = 3,
    East   = 4,
    West   = 5
}


#[packet( prefix = 0x25, bound = C2S, stage = Play )]
pub struct PlayerCommandC2SPacket {
    /// The id of the player.
    pub entity     : VarInt,
    pub action     : PlayerCommand,
    pub jump_boost : VarInt
}
#[packet_part(VarInt)]
pub enum PlayerCommand {
    StartSneak     = 0,
    StopSneak      = 1,
    LeaveBed       = 2,
    StartSprint    = 3,
    StopSprint     = 4,
    StartJumpHorse = 5,
    StopJumpHorse  = 6,
    OpenVehicleInv = 7,
    StartElytraFly  = 8
}


#[packet( prefix = 0x26, bound = C2S, stage = Play )]
pub struct PlayerInputC2SPacket {
    pub left    : f32,
    pub forward : f32,
    pub flags   : u8
}


#[packet( prefix = 0x27, bound = C2S, stage = Play )]
pub struct PongC2SPacket(pub u32);


#[packet( prefix = 0x28, bound = C2S, stage = Play )]
pub struct ChangeRecipeBookSettingsC2SPacket {
    pub book   : RecipeBook,
    pub open   : bool,
    pub filter : bool
}
#[packet_part(VarInt)]
pub enum RecipeBook {
    Craft = 0,
    Smelt = 1,
    Blast = 2,
    Smoke = 3
}


#[packet( prefix = 0x29, bound = C2S, stage = Play )]
pub struct SetSeenRecipeC2SPacket {
    pub recipe : Identifier
}


#[packet( prefix = 0x2A, bound = C2S, stage = Play )]
pub struct RenameItemC2SPacket {
    pub item_name : String
}


#[packet( prefix = 0x2B, bound = C2S, stage = Play )]
pub struct ResourcePackResponseC2SPacket {
    pub uuid   : Uuid,
    pub status : ResourcePackStatus
}


#[packet( prefix = 0x2C, bound = C2S, stage = Play )]
pub struct SeenAdvC2SPacket {
    pub action : AdvTabAction,
    pub tab    : Option<Identifier>
}
#[packet_part(VarInt)]
pub enum AdvTabAction {
    OpenTag     = 0,
    CloseScreen = 1
}


#[packet( prefix = 0x2D, bound = C2S, stage = Play )]
pub struct SelectTradeC2SPacket {
    pub slot : VarInt
}


#[packet( prefix = 0x2E, bound = C2S, stage = Play )]
pub struct SetBeaconEffectC2SPacket {
    pub primary_effect   : Option<RegEntry<StatusEffect>>,
    pub secondary_effect : Option<RegEntry<StatusEffect>>
}


#[packet( prefix = 0x2F, bound = C2S, stage = Play )]
pub struct SetHotbarSlotC2SPacket {
    // 0~8
    pub slot : u16
}


#[packet( prefix = 0x30, bound = C2S, stage = Play )]
pub struct ProgramCommandBlockC2SPacket {
    pub pos     : BlockPos,
    pub command : String,
    pub mode    : CommandBlockMode,
    pub flags   : u8
}
#[packet_part(VarInt)]
pub enum CommandBlockMode {
    Sequence = 0,
    Auto     = 1,
    Redstone = 2
}


#[packet( prefix = 0x31, bound = C2S, stage = Play )]
pub struct ProgramCommandBlockMinecartC2SPacket {
    pub entity       : VarInt,
    pub command      : String,
    pub track_output : bool
}


// TODO: SetCreativeModeSlotC2SPacket


#[packet( prefix = 0x33, bound = C2S, stage = Play )]
pub struct ProgramJigsawBlockC2SPacket {
    pub pos            : BlockPos,
    pub name           : Identifier,
    pub target         : Identifier,
    pub pool           : Identifier,
    pub final_state    : String,
    /// `rollable` or `aligned`
    pub joint_type     : String,
    pub sel_priority   : VarInt,
    pub place_priority : VarInt
}


#[packet( prefix = 0x34, bound = C2S, stage = Play )]
pub struct ProgramStructureBlockC2SPacket {
    pub pos       : BlockPos,
    pub action    : StructureBlockAction,
    pub mode      : StructureBlockMode,
    pub name      : String,
    /// -48~48
    pub offset_x  : u8,
    /// -48~48
    pub offset_y  : u8,
    /// -48~48
    pub offset_z  : u8,
    /// 0~48
    pub size_x    : u8,
    /// 0~48
    pub size_y    : u8,
    /// 0~48
    pub size_z    : u8,
    pub mirror    : StructureBlockMirror,
    pub rot       : StructureBlockRot,
    pub meta      : String,
    /// 0~1
    pub integrity : f32,
    pub seed      : VarLong,
    pub flags     : u8
}
#[packet_part(VarInt)]
pub enum StructureBlockAction {
    UpdateData    = 0,
    SaveStructure = 1,
    LoadStructure = 2,
    DetectSize    = 3
}
#[packet_part(VarInt)]
pub enum StructureBlockMode {
    Save   = 0,
    Load   = 1,
    Corner = 2,
    Data   = 3
}
#[packet_part(VarInt)]
pub enum StructureBlockMirror {
    None      = 0,
    LeftRight = 1,
    FrontBack = 2
}
#[packet_part(VarInt)]
pub enum StructureBlockRot {
    None               = 0,
    Clockwise90        = 1,
    Clockwise180       = 2,
    CounterClockwise90 = 3
}


#[packet( prefix = 0x35, bound = C2S, stage = Play )]
pub struct UpdateSignC2SPacket {
    pub pos   : BlockPos,
    pub front : bool,
    pub line0 : String,
    pub line1 : String,
    pub line2 : String,
    pub line3 : String
}


#[packet( prefix = 0x36, bound = C2S, stage = Play )]
pub struct SwingHandC2SPacket {
    pub hand : Hand
}


#[packet( prefix = 0x37, bound = C2S, stage = Play )]
pub struct TeleportToEntityC2SPacket {
    pub player : Uuid
}


#[packet( prefix = 0x38, bound = C2S, stage = Play )]
pub struct UseItemOnC2SPacket {
    pub hand         : Hand,
    pub pos          : BlockPos,
    pub face         : BlockFace,
    /// 0~1
    pub cursor_x     : f32,
    /// 0~1
    pub cursor_y     : f32,
    /// 0~1
    pub cursor_z     : f32,
    pub inside_block : bool,
    pub seq          : VarInt
}


#[packet( prefix = 0x39, bound = C2S, stage = Play )]
pub struct UseItemC2SPacket {
    pub hand      : Hand,
    pub seq       : VarInt,
    pub yaw_deg   : f32,
    pub pitch_deg : f32
}


packet_full_decode!( bound = C2S, stage = Play );
