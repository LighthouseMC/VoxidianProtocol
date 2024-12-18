package net.totobirdcreations.voxidianprotocoldatagen.dumps;

import com.google.gson.GsonBuilder;
import com.google.gson.JsonObject;
import net.minecraft.registry.Registry;
import net.minecraft.registry.RegistryKey;

import java.io.File;
import java.io.FileWriter;
import java.util.function.BiFunction;
import java.util.function.Function;

public class RegistryDump {
    public static<T> void dumpProtocol(String fileName, Registry<T> registry) {
        var obj = new JsonObject();
        var keys = registry.getKeys();
        var idx = 0;
        for(var key : keys) {
            idx += 1;
            obj.addProperty(String.valueOf(key.getValue()), registry.getRawId(registry.get(key)));
        }

        var gson = new GsonBuilder().disableHtmlEscaping().setPrettyPrinting().create();
        try {
            var file = new FileWriter(new File(new File(new File(System.getProperty("user.dir")).getParentFile().getParentFile(), "generated"), fileName + ".json"));
            gson.toJson(obj, file);
            file.close();
        } catch (Exception e) {
            e.printStackTrace();
            System.exit(1);
            return;
        }
    }
}
