package net.totobirdcreations.voxidianprotocoldatagen.mixin;

import net.minecraft.registry.BuiltinRegistries;
import net.minecraft.registry.Registries;
import net.totobirdcreations.voxidianprotocoldatagen.PacketGroup;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonPrimitive;
import com.google.gson.JsonObject;
import net.minecraft.server.MinecraftServer;
import net.minecraft.util.Identifier;
import net.totobirdcreations.voxidianprotocoldatagen.dumps.PacketDump;
import net.totobirdcreations.voxidianprotocoldatagen.dumps.RegistryDump;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Unique;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import java.io.File;
import java.io.FileWriter;
import java.util.Arrays;
import java.util.List;
import java.util.function.Function;
import java.lang.Exception;


@Mixin(MinecraftServer.class)
class MinecraftServerMixin {

	@Inject(at = @At("HEAD"), method = "startServer")
	private static <S extends MinecraftServer> void startServer(Function<Thread, S> factory, CallbackInfoReturnable<S> info) {
		PacketDump.dump();
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
		System.exit(0);
	}

}