use super::*;
//use config::{ ReportDetail, ServerLink };


#[packet( "minecraft:s2c/play/bundle_delimiter" )]
pub struct BundleDelimiterS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/add_entity" )]
pub struct AddEntityS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/add_experience_orb" )]
pub struct AddExperienceOrbS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/animate" )]
pub struct AnimateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/award_stats" )]
pub struct AwardStatsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/block_changed_ack" )]
pub struct BlockChangedAckS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/block_destruction" )]
pub struct BlockDestructionS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/block_entity_data" )]
pub struct BlockEntityDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/block_event" )]
pub struct BlockEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/block_update" )]
pub struct BlockUpdateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/boss_event" )]
pub struct BossEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/change_difficulty" )]
pub struct ChangeDifficultyS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/chunk_batch_finished" )]
pub struct ChunkBatchFinishedS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/chunk_batch_start" )]
pub struct ChunkBatchStartS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/chunks_biomes" )]
pub struct ChunksBiomesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/clear_titles" )]
pub struct ClearTitlesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/command_suggestions" )]
pub struct CommandSuggestionsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/commands" )]
pub struct CommandsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/container_close" )]
pub struct ContainerCloseS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/container_set_content" )]
pub struct ContainerSetContentS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/container_set_data" )]
pub struct ContainerSetDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/container_set_slot" )]
pub struct ContainerSetSlotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/cookie_request" )]
pub struct CookieRequestS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/cooldown" )]
pub struct CooldownS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/custom_chat_completions" )]
pub struct CustomChatCompletionsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/custom_payload" )]
pub struct CustomPayloadS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/damage_event" )]
pub struct DamageEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/debug_sample" )]
pub struct DebugSampleS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/delete_chat" )]
pub struct DeleteChatS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/disconnect" )]
pub struct DisconnectS2CPlayPacket {
    pub reason : NbtText
}


#[packet( "minecraft:s2c/play/disguised_chat" )]
pub struct DisguisedChatS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/entity_event" )]
pub struct EntityEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/entity_position_sync" )]
pub struct EntityPositionSyncS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/explode" )]
pub struct ExplodeS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/forget_world_chunk" )]
pub struct ForgetWorldChunkS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/game_event" )]
pub struct GameEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/horse_screen_open" )]
pub struct HorseScreenOpenS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/hurt_animation" )]
pub struct HurtAnimationS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/initialise_border" )]
pub struct InitialiseBorderS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/keep_alive" )]
pub struct KeepAliveS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/world_chunk_with_light" )]
pub struct WorldChunkWithLightS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/world_event" )]
pub struct WorldEventS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/world_particles" )]
pub struct WorldParticlesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/light_update" )]
pub struct LightUpdateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/login" )]
pub struct LoginS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/map_item_data" )]
pub struct MapItemDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/merchant_offers" )]
pub struct MerchantOffersS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_entity_pos" )]
pub struct MoveEntityPosS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_entity_pos_rot" )]
pub struct MoveEntityPosRotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_minecart_along_track" )]
pub struct MoveMinecartAlongTrackS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_entity_rot" )]
pub struct MoveEntityRotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/move_vehicle" )]
pub struct MoveVehicleS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/open_book" )]
pub struct OpenBookS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/open_screen" )]
pub struct OpenScreenS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/open_sign_editor" )]
pub struct OpenSignEditorS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/ping" )]
pub struct PingS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/pong_response" )]
pub struct PongResponseS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/place_ghost_recipe" )]
pub struct PlaceGhostRecipeS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_abilities" )]
pub struct PlayerAbilitiesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_chat" )]
pub struct PlayerChatS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_combat_end" )]
pub struct PlayerCombatEndS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_combat_enter" )]
pub struct PlayerCombatEnterS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_combat_kill" )]
pub struct PlayerCombatKillS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_info_remove" )]
pub struct PlayerInfoRemoveS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_info_update" )]
pub struct PlayerInfoUpdateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_look_at" )]
pub struct PlayerLookAtS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_position" )]
pub struct PlayerPositionS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/player_rotation" )]
pub struct PlayerRotationS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/recipe_book_add" )]
pub struct RecipeBookAddS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/recipe_book_remove" )]
pub struct RecipeBookRemoveS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/recipe_book_settings" )]
pub struct RecipeBookSettingsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/remove_entities" )]
pub struct RemoveEntitiesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/remove_mob_effect" )]
pub struct RemoveMobEffectS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/reset_score" )]
pub struct ResetScoreS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/resource_pack_pop" )]
pub struct ResourcePackPopS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/resource_pack_push" )]
pub struct ResourcePackPushS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/respawn" )]
pub struct RespawnS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/rotate_head" )]
pub struct RotateHeadS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/section_blocks_update" )]
pub struct SectionBlocksUpdateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/select_advancements_tab" )]
pub struct SelectAdvancementsTabS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/server_data" )]
pub struct ServerDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_action_bar_text" )]
pub struct SetActionBarTextS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_border_centre" )]
pub struct SetBorderCentreS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_border_lerp_size" )]
pub struct SetBorderLerpSizeS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_border_size" )]
pub struct SetBorderSizeS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_border_warning_delay" )]
pub struct SetBorderWarningDelayS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_border_warning_distance" )]
pub struct SetBorderWarningDistanceS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_camera" )]
pub struct SetCameraS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_chunk_cache_centre" )]
pub struct SetChunkCacheCentreS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_chunk_cache_radius" )]
pub struct SetChunkCacheRadiusS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_cursor_item" )]
pub struct SetCursorItemS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_default_spawn_position" )]
pub struct SetDefaultSpawnPositionS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_display_objective" )]
pub struct SetDisplayObjectiveS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_entity_data" )]
pub struct SetEntityDataS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_entity_link" )]
pub struct SetEntityLinkS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_entity_motion" )]
pub struct SetEntityMotionS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_equipment" )]
pub struct SetEquipmentS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_experience" )]
pub struct SetExperienceS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_health" )]
pub struct SetHealthS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_held_slot" )]
pub struct SetHeldSlotS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_objective" )]
pub struct SetObjectiveS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_passengers" )]
pub struct SetPassengersS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_player_inventory" )]
pub struct SetPlayerInventoryS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_player_team" )]
pub struct SetPlayerTeamS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_score" )]
pub struct SetScoreS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_simulation_distance" )]
pub struct SetSimulationDistanceS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_subtitle_text" )]
pub struct SetSubtitleTextS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_time" )]
pub struct SetTimeS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_title_text" )]
pub struct SetTitleTextS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/set_titles_animation" )]
pub struct SetTitlesAnimationS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/sound_entity" )]
pub struct SoundEntityS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/sound" )]
pub struct SoundS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/start_configuration" )]
pub struct StartConfigurationS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/stop_sound" )]
pub struct StopSoundS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/store_cookie" )]
pub struct StoreCookieS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/system_chat" )]
pub struct SystemChatS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/tab_list" )]
pub struct TabListS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/tag_query" )]
pub struct TagQueryS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/take_item_entity" )]
pub struct TakeItemEntityS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/teleport_entity" )]
pub struct TeleportEntityS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/ticking_state" )]
pub struct TickingStateS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/ticking_step" )]
pub struct TickingStepS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/transfer" )]
pub struct TransferS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_advancements" )]
pub struct UpdateAdvancementsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_attributes" )]
pub struct UpdateAttributesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_mob_effect" )]
pub struct UpdateMobEffectS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_recipes" )]
pub struct UpdateRecipesS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/update_tags" )]
pub struct UpdateTagsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/projectile_power" )]
pub struct ProjectilePowerS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/custom_report_details" )]
pub struct CustomReportDetailsS2CPlayPacket(TODO);


#[packet( "minecraft:s2c/play/server_links" )]
pub struct ServerLinksS2CPlayPacket(TODO);


packet_full_decode!{ S2C Play }
