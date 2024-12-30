use super::*;
use config::{ ClientInfo, ResourcePackStatus };
use s2c::play::{ Hand, PlayerAbilityFlags };
use voxidian_protocol_macros::packet;

#[packet("minecraft:c2s/play/accept_teleportation")]
pub struct AcceptTeleportationC2SPlayPacket {
    teleport_id: VarInt,
}
#[packet("minecraft:c2s/play/block_entity_tag_query")]
pub struct BlockEntityTagQueryC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/bundle_item_selected")]
pub struct BundleItemSelectedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/change_difficulty")]
pub struct ChangeDifficultyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat_ack")]
pub struct ChatAckC2SPlayPacket {
    message_id: VarInt
}
#[packet("minecraft:c2s/play/chat_command")]
pub struct ChatCommandC2SPlayPacket {
    command: String
}
#[packet("minecraft:c2s/play/chat_command_signed")]
pub struct ChatCommandSignedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat")]
pub struct ChatC2SPlayPacket(TODO);
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
    window_id: VarInt,
    button_id: VarInt
}
#[packet("minecraft:c2s/play/container_click")]
pub struct ContainerClickC2SPlayPacket {
    window_id: VarInt,
    state_id: VarInt,
    slot: i16,
    button: u8,
    mode: VarInt,
    changed_slots: LengthPrefixVec<VarInt, ChangedSlot>,
    cursor_item: SlotData
}

#[packet_part]
pub struct ChangedSlot {
    pub slot: i16,
    pub data: SlotData
}

#[packet("minecraft:c2s/play/container_close")]
pub struct ContainerCloseC2SPlayPacket {
    window_id: VarInt
}
#[packet("minecraft:c2s/play/container_slot_state_changed")]
pub struct ContainerSlotStateChangedC2SPlayPacket {
    slot_id: VarInt,
    window_id: VarInt,
    new_state: bool
}
#[packet("minecraft:c2s/play/cookie_response")]
pub struct CookieResponseC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/custom_payload")]
pub struct CustomPayloadC2SPlayPacket {
    identifier: Identifier,
    data: ConsumeAllVec<u8>
}
#[packet("minecraft:c2s/play/debug_sample_subscription")]
pub struct DebugSampleSubscriptionC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/edit_book")]
pub struct EditBookC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/entity_tag_query")]
pub struct EntityTagQueryC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/interact")]
pub struct InteractC2SPlayPacket {
    entity_id: VarInt,
    action: InteractAction,
    x: f32,
    y: f32,
    z: f32,
    hand: Hand,
    sneaking: bool
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
    keep_alive_id: i64
}
#[packet("minecraft:c2s/play/lock_difficulty")]
pub struct LockDifficultyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_player_pos")]
pub struct MovePlayerPosC2SPlayPacket {
    x: f64,
    y: f64,
    z: f64,
    flags: PositionFlags
}
packet_flags!(struct PositionFlags<u8> {
    on_ground: 0b01,
    against_wall: 010
});

#[packet("minecraft:c2s/play/move_player_pos_rot")]
pub struct MovePlayerPosRotC2SPlayPacket {
    x: f64,
    y: f64,
    z: f64,
    pitch: f32,
    yaw: f32,
    flags: PositionFlags
}
#[packet("minecraft:c2s/play/move_player_rot")]
pub struct MovePlayerRotC2SPlayPacket {
    pitch: f32,
    yaw: f32,
    flags: PositionFlags
}
#[packet("minecraft:c2s/play/move_player_status_only")]
pub struct MovePlayerStatusOnlyC2SPlayPacket {
    flags: PositionFlags
}
#[packet("minecraft:c2s/play/move_vehicle")]
pub struct MoveVehicleC2SPlayPacket {
    x: f64,
    y: f64,
    z: f64,
    pitch: f32,
    yaw: f32,
}
#[packet("minecraft:c2s/play/paddle_boat")]
pub struct PaddleBoatC2SPlayPacket {
    left_paddle: bool,
    right_paddle: bool
}
#[packet("minecraft:c2s/play/pick_item_from_block")]
pub struct PickItemFromBlockC2SPlayPacket {
    location: BlockPos,
    include_data: bool
}
#[packet("minecraft:c2s/play/pick_item_from_entity")]
pub struct PickItemFromEntityC2SPlayPacket {
    entity_id: VarInt,
    include_data: bool
}
#[packet("minecraft:c2s/play/ping_request")]
pub struct PingRequestC2SPlayPacket {
    id: i64
}
#[packet("minecraft:c2s/play/place_recipe")]
pub struct PlaceRecipeC2SPlayPacket {
    window_id: VarInt,
    recipe_id: VarInt,
    make_all: bool
}
#[packet("minecraft:c2s/play/player_abilities")]
pub struct PlayerAbilitiesC2SPlayPacket {
    abilities: PlayerAbilityFlags
}

#[packet("minecraft:c2s/play/player_action")]
pub struct PlayerActionC2SPlayPacket {
    status: PlayerStatus,
    location: BlockPos,
    face: BlockFace,
    sequence: VarInt
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
    player_id: VarInt,
    action_id: PlayerAction,
    jump_boost: VarInt
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
    flags: InputFlags
}

packet_flags!(struct InputFlags<u8> {
    forward: 0b1,
    backward: 0b01,
    left: 0b001,
    right: 0b0001,
    sneak: 0b00001,
    sprint: 0b000001
});
#[packet("minecraft:c2s/play/player_loaded")]
pub struct PlayerLoadedC2SPlayPacket;
#[packet("minecraft:c2s/play/pong")]
pub struct PongC2SPlayPacket {
    id: i32
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
    name: String
}
#[packet("minecraft:c2s/play/resource_pack")]
pub struct ResourcePackC2SPlayPacket {
    uuid: Uuid,
    status: ResourcePackStatus
}

#[packet("minecraft:c2s/play/seen_advancements")]
pub struct SeenAdvancementsC2SPlayPacket {
    action: SeenAdvancementsAction,
    tab_id: Option<Identifier>
}
#[packet_part(VarInt)]
pub enum SeenAdvancementsAction {
    OpenedTab = 0,
    ClosedScreen
}
#[packet("minecraft:c2s/play/select_trade")]
pub struct SelectTradeC2SPlayPacket {
    selected_slot: VarInt
}
#[packet("minecraft:c2s/play/set_beacon")]
pub struct SetBeaconC2SPlayPacket {
    primary: Option<VarInt>,
    secondary: Option<VarInt>
}
#[packet("minecraft:c2s/play/set_carried_item")]
pub struct SetCarriedItemC2SPlayPacket {
    slot: i16
}
#[packet("minecraft:c2s/play/set_command_block")]
pub struct SetCommandBlockC2SPlayPacket {
    location: BlockPos, // requires Position bit flag
    command: String,
    mode: CommandBlockMode,
    flags: CommandBlockFlags
}

#[packet_part(VarInt)]
pub enum CommandBlockMode {
    Sequence = 0,
    Auto,
    Redstone
}

packet_flags!(struct CommandBlockFlags<u8> {
    track_output: 0b1,
    is_conditional: 0b01,
    automatic: 0b001
});

#[packet("minecraft:c2s/play/set_command_minecart")]
pub struct SetCommandMinecartC2SPlayPacket {
    entity_id: VarInt,
    command: String,
    track_output: bool
}
#[packet("minecraft:c2s/play/set_creative_mode_slot")]
pub struct SetCreativeModeSlotC2SPlayPacket {
    slot: i16,
    new_item: SlotData
}
#[packet("minecraft:c2s/play/set_jigsaw_block")]
pub struct SetJigsawBlockC2SPlayPacket {
    location: BlockPos,
    name: String,
    target: String,
    pool: String,
    final_state: String,
    joint_type: String,
    selection_priority: VarInt,
    placement_priority: VarInt
}
#[packet("minecraft:c2s/play/set_structure_block")]
pub struct SetStructureBlockC2SPlayPacket {
    location: BlockPos,
    action: StructureBlockAction,
    mode: StructureBlockMode,
    name: String,
    offset_x: i8,
    offset_y: i8,
    offset_z: i8,
    size_x: i8,
    size_y: i8,
    size_z: i8,
    mirror: StructureBlockMirroredData,
    rotation: StructureBlockRotationData,
    metadata: String,
    integrity: f32,
    seed: VarInt,
    flags: StructureBlockFlags
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

packet_flags!(struct StructureBlockFlags<u8> {
    ignore_entities: 0b1,
    show_air: 0b01,
    show_bounding_box: 0b001
});

#[packet("minecraft:c2s/play/sign_update")]
pub struct SignUpdateC2SPlayPacket {
    location: BlockPos,
    is_front_text: bool,
    line1: String,
    line2: String,
    line3: String,
    line4: String

}
#[packet("minecraft:c2s/play/swing")]
pub struct SwingC2SPlayPacket {
    hand: Hand
}
#[packet("minecraft:c2s/play/teleport_to_entity")]
pub struct TeleportToEntityC2SPlayPacket {
    target_entity: Uuid
}
#[packet("minecraft:c2s/play/use_item_on")]
pub struct UseItemOnC2SPlayPacket {
    hand: Hand,
    target: BlockPos,
    face: BlockFace,
    cursor_x: f32,
    cursor_y: f32,
    cursor_z: f32,
    inside_block: bool,
    world_border_hit: bool,
    sequence: VarInt
}
#[packet("minecraft:c2s/play/use_item")]
pub struct UseItemC2SPlayPacket {
    hand: Hand,
    sequence: VarInt,
    yaw: f32,
    pitch: f32
}


packet_full_decode!{ C2S Play }
