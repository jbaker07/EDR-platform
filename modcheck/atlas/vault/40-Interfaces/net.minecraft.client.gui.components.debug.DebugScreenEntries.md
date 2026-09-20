---
type: "interface"
fqcn: "net.minecraft.client.gui.components.debug.DebugScreenEntries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.debug.DebugScreenEntries

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `register(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/g` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (54, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.components.debug.DebugScreenEntries {
    private static final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.gui.components.debug.DebugScreenEntry> ENTRIES_BY_ID;
    public static final net.minecraft.resources.Identifier GAME_VERSION;
    public static final net.minecraft.resources.Identifier FPS;
    public static final net.minecraft.resources.Identifier TPS;
    public static final net.minecraft.resources.Identifier MEMORY;
    public static final net.minecraft.resources.Identifier DETAILED_MEMORY;
    public static final net.minecraft.resources.Identifier SYSTEM_SPECS;
    public static final net.minecraft.resources.Identifier LOOKING_AT_BLOCK_STATE;
    public static final net.minecraft.resources.Identifier LOOKING_AT_BLOCK_TAGS;
    public static final net.minecraft.resources.Identifier LOOKING_AT_FLUID_STATE;
    public static final net.minecraft.resources.Identifier LOOKING_AT_FLUID_TAGS;
    public static final net.minecraft.resources.Identifier LOOKING_AT_ENTITY;
    public static final net.minecraft.resources.Identifier LOOKING_AT_ENTITY_TAGS;
    public static final net.minecraft.resources.Identifier CHUNK_RENDER_STATS;
    public static final net.minecraft.resources.Identifier CHUNK_GENERATION_STATS;
    public static final net.minecraft.resources.Identifier ENTITY_RENDER_STATS;
    public static final net.minecraft.resources.Identifier PARTICLE_RENDER_STATS;
    public static final net.minecraft.resources.Identifier CHUNK_SOURCE_STATS;
    public static final net.minecraft.resources.Identifier PLAYER_POSITION;
    public static final net.minecraft.resources.Identifier PLAYER_SECTION_POSITION;
    public static final net.minecraft.resources.Identifier PLAYER_SPEED;
    public static final net.minecraft.resources.Identifier LIGHT_LEVELS;
    public static final net.minecraft.resources.Identifier HEIGHTMAP;
    public static final net.minecraft.resources.Identifier BIOME;
    public static final net.minecraft.resources.Identifier LOCAL_DIFFICULTY;
    public static final net.minecraft.resources.Identifier DAY_COUNT;
    public static final net.minecraft.resources.Identifier ENTITY_SPAWN_COUNTS;
    public static final net.minecraft.resources.Identifier SOUND_MOOD;
    public static final net.minecraft.resources.Identifier SOUND_CACHE;
    public static final net.minecraft.resources.Identifier POST_EFFECTS;
    public static final net.minecraft.resources.Identifier ENTITY_HITBOXES;
    public static final net.minecraft.resources.Identifier CHUNK_BORDERS;
    public static final net.minecraft.resources.Identifier THREE_DIMENSIONAL_CROSSHAIR;
    public static final net.minecraft.resources.Identifier CHUNK_SECTION_PATHS;
    public static final net.minecraft.resources.Identifier GPU_UTILIZATION;
    public static final net.minecraft.resources.Identifier SIMPLE_PERFORMANCE_IMPACTORS;
    public static final net.minecraft.resources.Identifier CHUNK_SECTION_OCTREE;
    public static final net.minecraft.resources.Identifier VISUALIZE_WATER_LEVELS;
    public static final net.minecraft.resources.Identifier VISUALIZE_HEIGHTMAP;
    public static final net.minecraft.resources.Identifier VISUALIZE_COLLISION_BOXES;
    public static final net.minecraft.resources.Identifier VISUALIZE_ENTITY_SUPPORTING_BLOCKS;
    public static final net.minecraft.resources.Identifier VISUALIZE_BLOCK_LIGHT_LEVELS;
    public static final net.minecraft.resources.Identifier VISUALIZE_SKY_LIGHT_LEVELS;
    public static final net.minecraft.resources.Identifier VISUALIZE_SOLID_FACES;
    public static final net.minecraft.resources.Identifier VISUALIZE_CHUNKS_ON_SERVER;
    public static final net.minecraft.resources.Identifier VISUALIZE_SKY_LIGHT_SECTIONS;
    public static final net.minecraft.resources.Identifier CHUNK_SECTION_VISIBILITY;
    public static final java.util.Map<net.minecraft.client.gui.components.debug.DebugScreenProfile, java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.gui.components.debug.DebugScreenEntryStatus>> PROFILES;
    public net.minecraft.client.gui.components.debug.DebugScreenEntries();
    private static net.minecraft.resources.Identifier register(java.lang.String, net.minecraft.client.gui.components.debug.DebugScreenEntry);
    private static net.minecraft.resources.Identifier register(net.minecraft.resources.Identifier, net.minecraft.client.gui.components.debug.DebugScreenEntry);
    public static java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.gui.components.debug.DebugScreenEntry> allEntries();
    public static net.minecraft.client.gui.components.debug.DebugScreenEntry getEntry(net.minecraft.resources.Identifier);
    static {};
}
```
