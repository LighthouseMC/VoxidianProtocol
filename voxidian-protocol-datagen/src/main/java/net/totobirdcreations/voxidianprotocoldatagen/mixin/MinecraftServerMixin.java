package net.totobirdcreations.voxidianprotocoldatagen.mixin;

import net.totobirdcreations.voxidianprotocoldatagen.PacketGroup;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonPrimitive;
import com.google.gson.JsonObject;
import net.minecraft.server.MinecraftServer;
import net.minecraft.util.Identifier;
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

	@Unique
	private static int i = 0;

	@Inject(at = @At("HEAD"), method = "startServer")
	private static <S extends MinecraftServer> void startServer(Function<Thread, S> factory, CallbackInfoReturnable<S> info) {
		var packets_json = new JsonObject();

		for (var packet_group : PacketGroup.values()) {
			MinecraftServerMixin.i = 0;
			packet_group.factory.forEachPacketType((type, ignored) -> {
				var id = Identifier.of(
					type.id().getNamespace(),
					packet_group.bound.toString().toLowerCase()
						+ "/" + packet_group.stage.toString().toLowerCase()
						+ "/" + type.id().getPath()
				);
				packets_json.add(id.toString(), new JsonPrimitive(MinecraftServerMixin.i));
				MinecraftServerMixin.i += 1;
			});
		}

		var gson = new GsonBuilder().disableHtmlEscaping().setPrettyPrinting().create();
		try {
			var file = new FileWriter(new File(new File(new File(System.getProperty("user.dir")).getParentFile().getParentFile(), "generated"), "packets.json"));
			gson.toJson(packets_json, file);
			file.close();
		} catch (Exception e) {
			e.printStackTrace();
			System.exit(1);
			return;
		}
		System.exit(0);
	}

}