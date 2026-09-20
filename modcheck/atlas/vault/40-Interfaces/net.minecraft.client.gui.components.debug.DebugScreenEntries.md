---
type: "interface"
fqcn: "net.minecraft.client.gui.components.debug.DebugScreenEntries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.debug.DebugScreenEntries

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `register` | `(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/compone` | exact | invokestatic@14 in `DebugOverlayClient.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (48 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final ENTRIES_BY_ID : Ljava/util/Map;
public static final GAME_VERSION : Lnet/minecraft/resources/Identifier;
public static final FPS : Lnet/minecraft/resources/Identifier;
public static final TPS : Lnet/minecraft/resources/Identifier;
public static final MEMORY : Lnet/minecraft/resources/Identifier;
public static final DETAILED_MEMORY : Lnet/minecraft/resources/Identifier;
public static final SYSTEM_SPECS : Lnet/minecraft/resources/Identifier;
public static final LOOKING_AT_BLOCK_STATE : Lnet/minecraft/resources/Identifier;
public static final LOOKING_AT_BLOCK_TAGS : Lnet/minecraft/resources/Identifier;
public static final LOOKING_AT_FLUID_STATE : Lnet/minecraft/resources/Identifier;
public static final LOOKING_AT_FLUID_TAGS : Lnet/minecraft/resources/Identifier;
public static final LOOKING_AT_ENTITY : Lnet/minecraft/resources/Identifier;
public static final LOOKING_AT_ENTITY_TAGS : Lnet/minecraft/resources/Identifier;
public static final CHUNK_RENDER_STATS : Lnet/minecraft/resources/Identifier;
public static final CHUNK_GENERATION_STATS : Lnet/minecraft/resources/Identifier;
public static final ENTITY_RENDER_STATS : Lnet/minecraft/resources/Identifier;
public static final PARTICLE_RENDER_STATS : Lnet/minecraft/resources/Identifier;
public static final CHUNK_SOURCE_STATS : Lnet/minecraft/resources/Identifier;
public static final PLAYER_POSITION : Lnet/minecraft/resources/Identifier;
public static final PLAYER_SECTION_POSITION : Lnet/minecraft/resources/Identifier;
public static final PLAYER_SPEED : Lnet/minecraft/resources/Identifier;
public static final LIGHT_LEVELS : Lnet/minecraft/resources/Identifier;
public static final HEIGHTMAP : Lnet/minecraft/resources/Identifier;
public static final BIOME : Lnet/minecraft/resources/Identifier;
public static final LOCAL_DIFFICULTY : Lnet/minecraft/resources/Identifier;
public static final DAY_COUNT : Lnet/minecraft/resources/Identifier;
public static final ENTITY_SPAWN_COUNTS : Lnet/minecraft/resources/Identifier;
public static final SOUND_MOOD : Lnet/minecraft/resources/Identifier;
public static final SOUND_CACHE : Lnet/minecraft/resources/Identifier;
public static final POST_EFFECTS : Lnet/minecraft/resources/Identifier;
public static final ENTITY_HITBOXES : Lnet/minecraft/resources/Identifier;
public static final CHUNK_BORDERS : Lnet/minecraft/resources/Identifier;
public static final THREE_DIMENSIONAL_CROSSHAIR : Lnet/minecraft/resources/Identifier;
public static final CHUNK_SECTION_PATHS : Lnet/minecraft/resources/Identifier;
public static final GPU_UTILIZATION : Lnet/minecraft/resources/Identifier;
public static final SIMPLE_PERFORMANCE_IMPACTORS : Lnet/minecraft/resources/Identifier;
public static final CHUNK_SECTION_OCTREE : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_WATER_LEVELS : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_HEIGHTMAP : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_COLLISION_BOXES : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_ENTITY_SUPPORTING_BLOCKS : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_BLOCK_LIGHT_LEVELS : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_SKY_LIGHT_LEVELS : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_SOLID_FACES : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_CHUNKS_ON_SERVER : Lnet/minecraft/resources/Identifier;
public static final VISUALIZE_SKY_LIGHT_SECTIONS : Lnet/minecraft/resources/Identifier;
public static final CHUNK_SECTION_VISIBILITY : Lnet/minecraft/resources/Identifier;
public static final PROFILES : Ljava/util/Map;
public <init>()V
private static register(Ljava/lang/String;Lnet/minecraft/client/gui/components/debug/DebugScreenEntry;)Lnet/minecraft/resources/Identifier;
public static register(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/components/debug/DebugScreenEntry;)Lnet/minecraft/resources/Identifier;
public static allEntries()Ljava/util/Map;
public static getEntry(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/gui/components/debug/DebugScreenEntry;
static <clinit>()V
```
