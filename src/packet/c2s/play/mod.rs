use super::*;
use config::{ ClientInfo, ResourcePackStatus };
use s2c::play::{ Hand, PlayerAbilityFlags };
use voxidian_protocol_macros::packet;

#[packet("minecraft:c2s/play/accept_teleportation")]
pub struct AcceptTeleportationC2SPlayPacket {
    pub teleport_id: VarInt,
}
#[packet("minecraft:c2s/play/block_entity_tag_query")]
pub struct BlockEntityTagQueryC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/bundle_item_selected")]
pub struct BundleItemSelectedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/change_difficulty")]
pub struct ChangeDifficultyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat_ack")]
pub struct ChatAckC2SPlayPacket {
    pub message_id: VarInt
}
#[packet("minecraft:c2s/play/chat_command")]
pub struct ChatCommandC2SPlayPacket {
    pub command : String
}

#[packet("minecraft:c2s/play/chat_command_signed")]
pub struct ChatCommandSignedC2SPlayPacket {
    pub command   : String,
    pub timestamp : u64,
    pub salt      : u64,
    pub arg_sig   : LengthPrefixVec<VarInt, (String, [u8; 256])>,
    pub count     : VarInt,
    pub ack       : (u8, u8, u8)
}

#[packet("minecraft:c2s/play/chat")]
pub struct ChatC2SPlayPacket {
    pub message   : String,
    pub timestamp : u64,
    pub salt      : u64,
    pub sig       : Option<[u8; 256]>,
    pub count     : VarInt,
    pub ack       : (u8, u8, u8)
}

#[packet("minecraft:c2s/play/chat_session_update")]
pub struct ChatSessionUpdateC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chunk_batch_received")]
pub struct ChunkBatchReceivedC2SPlayPacket {
    pub chunks_per_second: f32
}

#[packet("minecraft:c2s/play/client_command")]
pub struct ClientCommandC2SPlayPacket {
    pub action: CommandAction
}

#[packet_part(VarInt)]
pub enum CommandAction {
    PerformRespawn = 0,
    RequestStats = 1
}
#[packet("minecraft:c2s/play/client_tick_end")]
pub struct ClientTickEndC2SPlayPacket;

#[packet("minecraft:c2s/play/client_information")]
pub struct ClientInformationC2SPlayPacket {
    pub info: ClientInfo
}
#[packet("minecraft:c2s/play/command_suggestion")]
pub struct CommandSuggestionC2SPlayPacket {
    transaction_id: VarInt,
    text: String
}
#[packet("minecraft:c2s/play/configuration_acknowledged")]
pub struct ConfigurationAcknowledgedC2SPlayPacket;
#[packet("minecraft:c2s/play/container_button_click")]
pub struct ContainerButtonClickC2SPlayPacket {
    pub window_id: VarInt,
    pub button_id: VarInt
}
#[packet("minecraft:c2s/play/container_click")]
pub struct ContainerClickC2SPlayPacket {
    pub window_id: VarInt,
    pub state_id: VarInt,
    pub slot: i16,
    pub button: u8,
    pub mode: VarInt,
    pub changed_slots: LengthPrefixVec<VarInt, ChangedSlot>,
    pub cursor_item: SlotData
}

#[packet_part]
pub struct ChangedSlot {
    pub slot: i16,
    pub data: SlotData
}

#[packet("minecraft:c2s/play/container_close")]
pub struct ContainerCloseC2SPlayPacket {
    pub window_id: VarInt
}
#[packet("minecraft:c2s/play/container_slot_state_changed")]
pub struct ContainerSlotStateChangedC2SPlayPacket {
    pub slot_id: VarInt,
    pub window_id: VarInt,
    pub new_state: bool
}
#[packet("minecraft:c2s/play/cookie_response")]
pub struct CookieResponseC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/custom_payload")]
pub struct CustomPayloadC2SPlayPacket {
    pub identifier: Identifier,
    pub data: ConsumeAllVec<u8>
}
#[packet("minecraft:c2s/play/debug_sample_subscription")]
pub struct DebugSampleSubscriptionC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/edit_book")]
pub struct EditBookC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/entity_tag_query")]
pub struct EntityTagQueryC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/interact")]
pub struct InteractC2SPlayPacket {
    pub entity_id: VarInt,
    pub action: InteractAction,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hand: Hand,
    pub sneaking: bool
}

#[packet_part(VarInt)]
pub enum InteractAction {
    Interact = 0,
    Attack = 1,
    InteractAt = 2
}

#[packet("minecraft:c2s/play/jigsaw_generate")]
pub struct JigsawGenerateC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/keep_alive")]
pub struct KeepAliveC2SPlayPacket {
    pub keep_alive_id: i64
}
#[packet("minecraft:c2s/play/lock_difficulty")]
pub struct LockDifficultyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_player_pos")]
pub struct MovePlayerPosC2SPlayPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub flags: PositionFlags
}
packet_flags!(pub struct PositionFlags<u8> {
    pub on_ground: 0b01,
    pub against_wall: 010
});

#[packet("minecraft:c2s/play/move_player_pos_rot")]
pub struct MovePlayerPosRotC2SPlayPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
    pub flags: PositionFlags
}
#[packet("minecraft:c2s/play/move_player_rot")]
pub struct MovePlayerRotC2SPlayPacket {
    pub pitch: f32,
    pub yaw: f32,
    pub flags: PositionFlags
}
#[packet("minecraft:c2s/play/move_player_status_only")]
pub struct MovePlayerStatusOnlyC2SPlayPacket {
    pub flags: PositionFlags
}
#[packet("minecraft:c2s/play/move_vehicle")]
pub struct MoveVehicleC2SPlayPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}
#[packet("minecraft:c2s/play/paddle_boat")]
pub struct PaddleBoatC2SPlayPacket {
    pub left_paddle: bool,
    pub right_paddle: bool
}
#[packet("minecraft:c2s/play/pick_item_from_block")]
pub struct PickItemFromBlockC2SPlayPacket {
    pub location: BlockPos,
    pub include_data: bool
}
#[packet("minecraft:c2s/play/pick_item_from_entity")]
pub struct PickItemFromEntityC2SPlayPacket {
    pub entity_id: VarInt,
    pub include_data: bool
}
#[packet("minecraft:c2s/play/ping_request")]
pub struct PingRequestC2SPlayPacket {
    pub id: i64
}
#[packet("minecraft:c2s/play/place_recipe")]
pub struct PlaceRecipeC2SPlayPacket {
    pub window_id: VarInt,
    pub recipe_id: VarInt,
    pub make_all: bool
}
#[packet("minecraft:c2s/play/player_abilities")]
pub struct PlayerAbilitiesC2SPlayPacket {
    pub abilities: PlayerAbilityFlags
}

#[packet("minecraft:c2s/play/player_action")]
pub struct PlayerActionC2SPlayPacket {
    pub status: PlayerStatus,
    pub location: BlockPos,
    pub face: BlockFace,
    pub sequence: VarInt
}

#[packet_part(VarInt)]
pub enum PlayerStatus {
    StartedDigging = 0,
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropItem,
    FinishUsingItem,
    SwapItems
}

#[packet_part(u8)]
pub enum BlockFace {
    Down = 0,
    Up,
    North,
    South,
    West,
    East
}

#[packet("minecraft:c2s/play/player_command")]
pub struct PlayerCommandC2SPlayPacket {
    pub player_id: VarInt,
    pub action_id: PlayerAction,
    pub jump_boost: VarInt
}

#[packet_part(VarInt)]
pub enum PlayerAction {
    StartSneaking,
    StopSneaking,
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartHorseJump,
    EndHorseJump,
    OpenVehicleInventory,
    StartFlyingWithElytra
}
#[packet("minecraft:c2s/play/player_input")]
pub struct PlayerInputC2SPlayPacket {
    pub flags: InputFlags
}

packet_flags!(pub struct InputFlags<u8> {
    pub forward: 0b1,
    pub backward: 0b01,
    pub left: 0b001,
    pub right: 0b0001,
    pub sneak: 0b00001,
    pub sprint: 0b000001
});
#[packet("minecraft:c2s/play/player_loaded")]
pub struct PlayerLoadedC2SPlayPacket;
#[packet("minecraft:c2s/play/pong")]
pub struct PongC2SPlayPacket {
    pub id: i32
}
#[packet("minecraft:c2s/play/recipe_book_change_settings")]
pub struct RecipeBookChangeSettingsC2SPlayPacket {
    recipe_book_type: RecipeBookType,
    book_open: bool,
    filter_active: bool
}

#[packet_part(VarInt)]
pub enum RecipeBookType {
    Crafting,
    Furnace,
    BlastFurnace,
    Smoker
}
#[packet("minecraft:c2s/play/recipe_book_seen_recipe")]
pub struct RecipeBookSeenRecipeC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/rename_item")]
pub struct RenameItemC2SPlayPacket {
    pub name: String
}
#[packet("minecraft:c2s/play/resource_pack")]
pub struct ResourcePackC2SPlayPacket {
    pub uuid: Uuid,
    pub status: ResourcePackStatus
}

#[packet("minecraft:c2s/play/seen_advancements")]
pub struct SeenAdvancementsC2SPlayPacket {
    pub action: SeenAdvancementsAction,
    pub tab_id: Option<Identifier>
}
#[packet_part(VarInt)]
pub enum SeenAdvancementsAction {
    OpenedTab = 0,
    ClosedScreen
}
#[packet("minecraft:c2s/play/select_trade")]
pub struct SelectTradeC2SPlayPacket {
    pub selected_slot: VarInt
}
#[packet("minecraft:c2s/play/set_beacon")]
pub struct SetBeaconC2SPlayPacket {
    pub primary: Option<VarInt>,
    pub secondary: Option<VarInt>
}
#[packet("minecraft:c2s/play/set_carried_item")]
pub struct SetCarriedItemC2SPlayPacket {
    pub slot: i16
}
#[packet("minecraft:c2s/play/set_command_block")]
pub struct SetCommandBlockC2SPlayPacket {
    pub location: BlockPos, // requires Position bit flag
    pub command: String,
    pub mode: CommandBlockMode,
    pub flags: CommandBlockFlags
}

#[packet_part(VarInt)]
pub enum CommandBlockMode {
    Sequence = 0,
    Auto,
    Redstone
}

packet_flags!(pub struct CommandBlockFlags<u8> {
    pub track_output: 0b1,
    pub is_conditional: 0b01,
    pub automatic: 0b001
});

#[packet("minecraft:c2s/play/set_command_minecart")]
pub struct SetCommandMinecartC2SPlayPacket {
    pub entity_id: VarInt,
    pub command: String,
    pub track_output: bool
}
#[packet("minecraft:c2s/play/set_creative_mode_slot")]
pub struct SetCreativeModeSlotC2SPlayPacket {
    pub slot: i16,
    pub new_item: SlotData
}
#[packet("minecraft:c2s/play/set_jigsaw_block")]
pub struct SetJigsawBlockC2SPlayPacket {
    pub location: BlockPos,
    pub name: String,
    pub target: String,
    pub pool: String,
    pub final_state: String,
    pub joint_type: String,
    pub selection_priority: VarInt,
    pub placement_priority: VarInt
}
#[packet("minecraft:c2s/play/set_structure_block")]
pub struct SetStructureBlockC2SPlayPacket {
    pub location: BlockPos,
    pub action: StructureBlockAction,
    pub mode: StructureBlockMode,
    pub name: String,
    pub offset_x: i8,
    pub offset_y: i8,
    pub offset_z: i8,
    pub size_x: i8,
    pub size_y: i8,
    pub size_z: i8,
    pub mirror: StructureBlockMirroredData,
    pub rotation: StructureBlockRotationData,
    pub metadata: String,
    pub integrity: f32,
    pub seed: VarInt,
    pub flags: StructureBlockFlags
}

#[packet_part(VarInt)]
pub enum StructureBlockAction {
    UpdateData = 0,
    SaveStructure,
    LoadStructure,
    DetectSize
}

#[packet_part(VarInt)]
pub enum StructureBlockMode {
    Save = 0,
    Load,
    Corner,
    Data
}

#[packet_part(VarInt)]
pub enum StructureBlockMirroredData {
    None = 0,
    LeftRight = 1,
    FrontBack = 2
}

#[packet_part(VarInt)]
pub enum StructureBlockRotationData {
    None = 0,
    Clockwise90 = 1,
    Clockwise180 = 2,
    CounterClockwise90 = 3
}

packet_flags!(pub struct StructureBlockFlags<u8> {
    pub ignore_entities: 0b1,
    pub show_air: 0b01,
    pub show_bounding_box: 0b001
});

#[packet("minecraft:c2s/play/sign_update")]
pub struct SignUpdateC2SPlayPacket {
    pub location: BlockPos,
    pub is_front_text: bool,
    pub line1: String,
    pub line2: String,
    pub line3: String,
    pub line4: String
}
#[packet("minecraft:c2s/play/swing")]
pub struct SwingC2SPlayPacket {
    pub hand: Hand
}
#[packet("minecraft:c2s/play/teleport_to_entity")]
pub struct TeleportToEntityC2SPlayPacket {
    target_entity: Uuid
}
#[packet("minecraft:c2s/play/use_item_on")]
pub struct UseItemOnC2SPlayPacket {
    pub hand: Hand,
    pub target: BlockPos,
    pub face: BlockFace,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub cursor_z: f32,
    pub inside_block: bool,
    pub world_border_hit: bool,
    pub sequence: VarInt
}
#[packet("minecraft:c2s/play/use_item")]
pub struct UseItemC2SPlayPacket {
    pub hand: Hand,
    pub sequence: VarInt,
    pub yaw: f32,
    pub pitch: f32
}


packet_full_decode!{ C2S Play }
