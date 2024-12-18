package net.totobirdcreations.voxidianprotocoldatagen.dumps;

import com.google.gson.GsonBuilder;
import com.google.gson.JsonObject;
import com.google.gson.JsonPrimitive;
import net.minecraft.util.Identifier;
import net.totobirdcreations.voxidianprotocoldatagen.PacketGroup;

import java.io.File;
import java.io.FileWriter;

public class PacketDump {
    public static void dump() {
        var packets_json = new JsonObject();
        final int[] packet_idx = {0};

        for (var packet_group : PacketGroup.values()) {
            packet_idx[0] = 0;
            packet_group.factory.forEachPacketType((type, ignored) -> {
                var id = Identifier.of(
                        type.id().getNamespace(),
                        (
                                packet_group.bound.toString().toLowerCase()
                                        + "/" + packet_group.stage.toString().toLowerCase()
                                        + "/" + type.id().getPath()
                        )
                                .replace("initialize" , "initialise" )
                                .replace("level"      , "world"      )
                                .replace("center"     , "centre"     )
                );
                var packet_json = new JsonObject();
                packet_json.add("prefix" , new JsonPrimitive(packet_idx[0]));
                packet_json.add("bound"  , new JsonPrimitive(packet_group.bound.toString() ));
                packet_json.add("stage"  , new JsonPrimitive(packet_group.stage.toString() ));
                packets_json.add(id.toString(), packet_json);
                packet_idx[0] += 1;
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
    }
}
