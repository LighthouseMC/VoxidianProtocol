package net.totobirdcreations.voxidianprotocoldatagen.mixin;

import com.mojang.datafixers.DataFixer;
import net.minecraft.data.DataProvider;
import net.minecraft.data.DynamicRegistriesProvider;
import net.minecraft.entity.damage.DamageType;
import net.minecraft.registry.*;
import net.minecraft.resource.ResourcePackManager;
import net.minecraft.server.SaveLoader;
import net.minecraft.server.WorldGenerationProgressListenerFactory;
import net.minecraft.server.world.ServerWorld;
import net.minecraft.util.ApiServices;
import net.minecraft.world.World;
import net.minecraft.world.level.storage.LevelStorage;
import net.totobirdcreations.voxidianprotocoldatagen.PacketGroup;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonPrimitive;
import com.google.gson.JsonObject;
import net.minecraft.server.MinecraftServer;
import net.minecraft.util.Identifier;
import net.totobirdcreations.voxidianprotocoldatagen.dumps.PacketDump;
import net.totobirdcreations.voxidianprotocoldatagen.dumps.RegistryDump;
import org.spongepowered.asm.mixin.Final;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.Unique;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import java.io.File;
import java.io.FileWriter;
import java.net.Proxy;
import java.util.Arrays;
import java.util.List;
import java.util.function.Function;
import java.lang.Exception;


@Mixin(MinecraftServer.class)
class MinecraftServerMixin {

	@Shadow @Final private CombinedDynamicRegistries<ServerDynamicRegistryType> combinedDynamicRegistries;

	@Inject(at = @At("TAIL"), method = "<init>")
	private void ctor(
		Thread serverThread,
		LevelStorage.Session session,
		ResourcePackManager dataPackManager,
		SaveLoader saveLoader, Proxy proxy,
		DataFixer dataFixer,
		ApiServices apiServices,
		WorldGenerationProgressListenerFactory worldGenerationProgressListenerFactory,
		CallbackInfo ci
	) {
		PacketDump.dump();

		var rm = this.combinedDynamicRegistries.getCombinedRegistryManager();

		RegistryDump.dumpProtocol(
				"items",
				Registries.ITEM
		);
		RegistryDump.dumpProtocol(
				"attributes",
				Registries.ATTRIBUTE,
				(attr, obj) -> {
					obj.addProperty("default", attr.getDefaultValue());
				}
		);
		RegistryDump.dumpProtocol(
				"blocks",
				Registries.BLOCK
		);
		RegistryDump.dumpProtocol(
				"data_component_type",
				Registries.DATA_COMPONENT_TYPE
		);
		RegistryDump.dumpProtocol(
				"entity_types",
				Registries.ENTITY_TYPE,
				(entity, obj) -> {
					obj.addProperty("width", entity.getWidth());
					obj.addProperty("height", entity.getHeight());
					obj.addProperty("translation_key", entity.getTranslationKey());
				}
		);
		RegistryDump.dumpProtocol(
			"damage_types",
			rm.getOrThrow(RegistryKeys.DAMAGE_TYPE),
			(val, obj) -> {
				obj.addProperty("message_id", val.msgId());
				obj.addProperty("exhaustion", val.exhaustion());
				obj.addProperty("scaling", val.scaling().asString());
				obj.addProperty("msg_ty", val.deathMessageType().asString());
			}
		);
		System.exit(0);
	}

}