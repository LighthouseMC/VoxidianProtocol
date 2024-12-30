use super::*;
use config::{ ClientInfo, ResourcePackStatus };
use s2c::play::{ DebugSampleKind, Difficulty, Hand, PlayerAbilityFlags };
use voxidian_protocol_macros::packet;

#[packet("minecraft:c2s/play/accept_teleportation")]
pub struct AcceptTeleportationC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/block_entity_tag_query")]
pub struct BlockEntityTagQueryC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/bundle_item_selected")]
pub struct BundleItemSelectedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/change_difficulty")]
pub struct ChangeDifficultyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat_ack")]
pub struct ChatAckC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat_command")]
pub struct ChatCommandC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat_command_signed")]
pub struct ChatCommandSignedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat")]
pub struct ChatC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chat_session_update")]
pub struct ChatSessionUpdateC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/chunk_batch_received")]
pub struct ChunkBatchReceivedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/client_command")]
pub struct ClientCommandC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/client_tick_end")]
pub struct ClientTickEndC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/client_information")]
pub struct ClientInformationC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/command_suggestion")]
pub struct CommandSuggestionC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/configuration_acknowledged")]
pub struct ConfigurationAcknowledgedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/container_button_click")]
pub struct ContainerButtonClickC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/container_click")]
pub struct ContainerClickC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/container_close")]
pub struct ContainerCloseC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/container_slot_state_changed")]
pub struct ContainerSlotStateChangedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/cookie_response")]
pub struct CookieResponseC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/custom_payload")]
pub struct CustomPayloadC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/debug_sample_subscription")]
pub struct DebugSampleSubscriptionC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/edit_book")]
pub struct EditBookC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/entity_tag_query")]
pub struct EntityTagQueryC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/interact")]
pub struct InteractC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/jigsaw_generate")]
pub struct JigsawGenerateC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/keep_alive")]
pub struct KeepAliveC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/lock_difficulty")]
pub struct LockDifficultyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_player_pos")]
pub struct MovePlayerPosC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_player_pos_rot")]
pub struct MovePlayerPosRotC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_player_rot")]
pub struct MovePlayerRotC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_player_status_only")]
pub struct MovePlayerStatusOnlyC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/move_vehicle")]
pub struct MoveVehicleC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/paddle_boat")]
pub struct PaddleBoatC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/pick_item_from_block")]
pub struct PickItemFromBlockC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/pick_item_from_entity")]
pub struct PickItemFromEntityC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/ping_request")]
pub struct PingRequestC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/place_recipe")]
pub struct PlaceRecipeC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/player_abilities")]
pub struct PlayerAbilitiesC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/player_action")]
pub struct PlayerActionC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/player_command")]
pub struct PlayerCommandC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/player_input")]
pub struct PlayerInputC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/player_loaded")]
pub struct PlayerLoadedC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/pong")]
pub struct PongC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/recipe_book_change_settings")]
pub struct RecipeBookChangeSettingsC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/recipe_book_seen_recipe")]
pub struct RecipeBookSeenRecipeC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/rename_item")]
pub struct RenameItemC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/resource_pack")]
pub struct ResourcePackC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/seen_advancements")]
pub struct SeenAdvancementsC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/select_trade")]
pub struct SelectTradeC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_beacon")]
pub struct SetBeaconC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_carried_item")]
pub struct SetCarriedItemC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_command_block")]
pub struct SetCommandBlockC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_command_minecart")]
pub struct SetCommandMinecartC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_creative_mode_slot")]
pub struct SetCreativeModeSlotC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_jigsaw_block")]
pub struct SetJigsawBlockC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/set_structure_block")]
pub struct SetStructureBlockC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/sign_update")]
pub struct SignUpdateC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/swing")]
pub struct SwingC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/teleport_to_entity")]
pub struct TeleportToEntityC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/use_item_on")]
pub struct UseItemOnC2SPlayPacket(TODO);
#[packet("minecraft:c2s/play/use_item")]
pub struct UseItemC2SPlayPacket(TODO);


packet_full_decode!{ C2S Play }
