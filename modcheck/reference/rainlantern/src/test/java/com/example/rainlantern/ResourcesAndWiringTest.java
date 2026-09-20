package com.example.rainlantern;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

/**
 * The files and wiring a block needs to exist at all.
 *
 * <p>Every failure here is silent in game rather than loud: a block with no
 * blockstate renders as the missing-texture cube, one with no loot table drops
 * nothing when broken, and a registration method nothing calls leaves a mod
 * that loads and does nothing. None of those produce a compile error, and the
 * build passes in every case.
 *
 * <p>These read the actual resource and source files rather than asserting on
 * strings we also wrote elsewhere.
 */
class ResourcesAndWiringTest {
    private static final String MOD_ID = "rainlantern";
    private static final String BLOCK_ID = "rain_lantern";
    private static final Path SOURCE_ROOT = Path.of("src", "main", "java",
                                                     "com", "example", "rainlantern");

    private static String resource(String path) {
        try (InputStream in = ResourcesAndWiringTest.class.getClassLoader()
                .getResourceAsStream(path)) {
            if (in == null) {
                fail("missing resource: " + path);
            }
            return new String(in.readAllBytes(), StandardCharsets.UTF_8);
        } catch (IOException e) {
            throw new AssertionError("could not read " + path, e);
        }
    }

    private static String source(String fileName) {
        try {
            return Files.readString(SOURCE_ROOT.resolve(fileName));
        } catch (IOException e) {
            throw new AssertionError("could not read " + fileName, e);
        }
    }

    // -- resources -----------------------------------------------------------

    @Test
    @DisplayName("the manifest declares both entrypoints")
    void manifestEntrypoints() {
        String manifest = resource("fabric.mod.json");
        assertTrue(manifest.contains("\"main\""), "no main entrypoint declared");
        assertTrue(manifest.contains("com.example.rainlantern.Rainlantern"));
        assertTrue(manifest.contains("\"client\""),
                   "no client entrypoint: the HUD and the receiver would never register");
        assertTrue(manifest.contains("com.example.rainlantern.client.RainLanternClient"));
        assertTrue(manifest.contains("fabric-api"),
                   "the mod uses fabric-api modules and must depend on them");
    }

    @Test
    @DisplayName("the manifest declares no mixin file that does not exist")
    void noDanglingMixinDeclaration() {
        String manifest = resource("fabric.mod.json");
        if (manifest.contains("\"mixins\"")) {
            assertTrue(getClass().getClassLoader()
                               .getResource(MOD_ID + ".mixins.json") != null,
                       "the manifest declares a mixin file that is not on the classpath");
        }
    }

    @Test
    @DisplayName("the block has a blockstate, a model and an item model")
    void blockResources() {
        String blockstate = resource("assets/" + MOD_ID + "/blockstates/" + BLOCK_ID + ".json");
        assertTrue(blockstate.contains(MOD_ID + ":block/" + BLOCK_ID),
                   "the blockstate must point at a model in this namespace");
        resource("assets/" + MOD_ID + "/models/block/" + BLOCK_ID + ".json");
        String itemModel = resource("assets/" + MOD_ID + "/models/item/" + BLOCK_ID + ".json");
        assertTrue(itemModel.contains(MOD_ID + ":block/" + BLOCK_ID),
                   "the item model must parent the block model");
    }

    @Test
    @DisplayName("the block drops itself when broken")
    void lootTable() {
        String loot = resource("data/" + MOD_ID + "/loot_table/blocks/" + BLOCK_ID + ".json");
        assertTrue(loot.contains(MOD_ID + ":" + BLOCK_ID),
                   "without this the block vanishes when broken");
    }

    @Test
    @DisplayName("the block, the item and the gamerule are all translated")
    void translations() {
        String lang = resource("assets/" + MOD_ID + "/lang/en_us.json");
        for (String key : List.of("block." + MOD_ID + "." + BLOCK_ID,
                                  "gamerule." + MOD_ID + ".rain_lantern_charge_rate")) {
            assertTrue(lang.contains(key), "untranslated: " + key);
        }
    }

    // -- registration is actually called ------------------------------------

    @Test
    @DisplayName("the main entrypoint calls both registration paths")
    void mainEntrypointWiring() {
        String main = source("Rainlantern.java");
        assertTrue(main.contains("RainLanternNetwork.registerCommon()"),
                   "the payload type must be registered on the server too");
        assertTrue(main.contains("RainLanternContent.register()"),
                   "a registration method nothing calls is dead code the compiler likes");
    }

    @Test
    @DisplayName("content registration covers block, item and block entity type")
    void contentRegistration() {
        String content = source("RainLanternContent.java");
        for (String needle : List.of("BuiltInRegistries.BLOCK",
                                     "BuiltInRegistries.ITEM",
                                     "BuiltInRegistries.BLOCK_ENTITY_TYPE")) {
            assertTrue(content.contains(needle), "not registered: " + needle);
        }
    }

    @Test
    @DisplayName("the client entrypoint registers the receiver and the HUD")
    void clientWiring() {
        String client = Path.of("src", "main", "java", "com", "example",
                                "rainlantern", "client", "RainLanternClient.java")
                .toFile().exists()
                ? readClient() : fail("client entrypoint missing");
        assertTrue(client.contains("registerGlobalReceiver"),
                   "a payload with no receiver is a payload that arrives nowhere");
        assertTrue(client.contains("LanternHud.register()"),
                   "the HUD element must be registered");
        assertTrue(client.contains("DISCONNECT"),
                   "one world's charges must not survive into another");
    }

    private static String readClient() {
        try {
            return Files.readString(Path.of("src", "main", "java", "com", "example",
                                            "rainlantern", "client",
                                            "RainLanternClient.java"));
        } catch (IOException e) {
            throw new AssertionError(e);
        }
    }

    @Test
    @DisplayName("the tick handler is installed for the block entity")
    void tickerInstalled() {
        String block = source("LanternBlock.java");
        assertTrue(block.contains("createTickerHelper"),
                   "without a ticker nothing ever charges");
        assertTrue(block.contains("LanternBlockEntity::serverTick"));
        assertTrue(block.contains("isClientSide"),
                   "a client-side ticker would compute a second, drifting charge");
    }

    @Test
    @DisplayName("the charge is written as well as held")
    void persistenceWiring() {
        String entity = source("LanternBlockEntity.java");
        assertTrue(entity.contains("saveAdditional"));
        assertTrue(entity.contains("loadAdditional"));
        assertTrue(entity.contains("setChanged()"),
                   "without setChanged the value is never written and the symptom "
                   + "is indistinguishable from a reload bug");
    }

    @Test
    @DisplayName("no required behaviour is left as a seam")
    void noEmptySeams() {
        for (String file : List.of("LanternBlock.java", "LanternBlockEntity.java",
                                   "RainLanternContent.java", "RainLanternNetwork.java",
                                   "Rainlantern.java")) {
            String body = source(file);
            assertFalse(body.contains(">>> your logic here"),
                        file + " still contains a generated seam");
            assertFalse(body.contains("TODO"), file + " contains a TODO");
        }
    }
}
