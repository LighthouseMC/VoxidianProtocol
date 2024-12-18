package net.totobirdcreations.voxidianprotocoldatagen.dumps;

import com.google.gson.GsonBuilder;
import com.google.gson.JsonObject;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;

import java.io.File;
import java.io.FileWriter;
import java.util.function.BiConsumer;
import java.util.function.BiFunction;
import java.util.function.Consumer;
import java.util.function.Function;

public class RegistryDump {
    public static<T> void dumpProtocol(String fileName, Registry<T> registry) {
        var top = new JsonObject();
        var keys = registry.getKeys();
        var idx = 0;
        for(var key : keys) {
            idx += 1;

            var obj = new JsonObject();
            obj.addProperty("protocol_id", registry.getRawId(registry.get(key)));

            top.add(String.valueOf(key.getValue()), obj);
        }

        var gson = new GsonBuilder().disableHtmlEscaping().setPrettyPrinting().create();
        try {
            var file = new FileWriter(new File(new File(new File(System.getProperty("user.dir")).getParentFile().getParentFile(), "generated"), fileName + ".json"));
            gson.toJson(top, file);
            file.close();
        } catch (Exception e) {
            e.printStackTrace();
            System.exit(1);
            return;
        }
    }

    public static<T> void dumpProtocol(String fileName, Registry<T> registry, BiConsumer<T, JsonObject> metadata) {
        var top = new JsonObject();
        var keys = registry.getKeys();
        var idx = 0;
        for(var key : keys) {
            idx += 1;

            var obj = new JsonObject();
            obj.addProperty("protocol_id", registry.getRawId(registry.get(key)));
            metadata.accept(registry.get(key), obj);

            top.add(String.valueOf(key.getValue()), obj);
        }

        var gson = new GsonBuilder().disableHtmlEscaping().setPrettyPrinting().create();
        try {
            var file = new FileWriter(new File(new File(new File(System.getProperty("user.dir")).getParentFile().getParentFile(), "generated"), fileName + ".json"));
            gson.toJson(top, file);
            file.close();
        } catch (Exception e) {
            e.printStackTrace();
            System.exit(1);
            return;
        }
    }
}
