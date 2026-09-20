---
type: "interface"
fqcn: "net.minecraft.SharedConstants"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.SharedConstants

System: [[20-Systems/net.minecraft|net.minecraft]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getCurrentVersion` | `()Lnet/minecraft/WorldVersion;` | exact | invokestatic@2 in `FabricDataGenerator.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getCurrentVersion` | `()Lnet/minecraft/WorldVersion;` | exact | invokestatic@114 in `ModPackResourcesUtil.openDefault` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `IS_RUNNING_IN_IDE` | `Z` | exact | getstatic@0 in `CommandsMixin.init` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (126 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SNAPSHOT : Z
public static final WORLD_VERSION : I
public static final SERIES : Ljava/lang/String;
public static final RELEASE_NETWORK_PROTOCOL_VERSION : I
public static final SNAPSHOT_NETWORK_PROTOCOL_VERSION : I
public static final SNBT_NAG_VERSION : I
private static final SNAPSHOT_PROTOCOL_BIT : I
public static final CRASH_EAGERLY : Z
public static final RESOURCE_PACK_FORMAT_MAJOR : I
public static final RESOURCE_PACK_FORMAT_MINOR : I
public static final DATA_PACK_FORMAT_MAJOR : I
public static final DATA_PACK_FORMAT_MINOR : I
public static final RPC_MANAGEMENT_SERVER_API_VERSION : Ljava/lang/String;
public static final LANGUAGE_FORMAT : I
public static final REPORT_FORMAT_VERSION : I
public static final DATA_VERSION_TAG : Ljava/lang/String;
public static final DEBUG_FLAG_PREFIX : Ljava/lang/String;
public static final DEBUG_ENABLED : Z
private static final DEBUG_PRINT_PROPERTIES : Z
public static final FIX_TNT_DUPE : Z
public static final FIX_SAND_DUPE : Z
public static final DEBUG_OPEN_INCOMPATIBLE_WORLDS : Z
public static final DEBUG_ALLOW_LOW_SIM_DISTANCE : Z
public static final DEBUG_HOTKEYS : Z
public static final DEBUG_UI_NARRATION : Z
public static final DEBUG_SHUFFLE_UI_RENDERING_ORDER : Z
public static final DEBUG_SHUFFLE_MODELS : Z
public static final DEBUG_RENDER_UI_LAYERING_RECTANGLES : Z
public static final DEBUG_PATHFINDING : Z
public static final DEBUG_SHOW_LOCAL_SERVER_ENTITY_HIT_BOXES : Z
public static final DEBUG_SHAPES : Z
public static final DEBUG_NEIGHBORSUPDATE : Z
public static final DEBUG_EXPERIMENTAL_REDSTONEWIRE_UPDATE_ORDER : Z
public static final DEBUG_STRUCTURES : Z
public static final DEBUG_GAME_EVENT_LISTENERS : Z
public static final DEBUG_DUMP_TEXTURE_ATLAS : Z
public static final DEBUG_STRUCTURE_EDIT_MODE : Z
public static final DEBUG_SAVE_STRUCTURES_AS_SNBT : Z
public static final DEBUG_SYNCHRONOUS_GL_LOGS : Z
public static final DEBUG_VERBOSE_SERVER_EVENTS : Z
public static final DEBUG_NAMED_RUNNABLES : Z
public static final DEBUG_GOAL_SELECTOR : Z
public static final DEBUG_VILLAGE_SECTIONS : Z
public static final DEBUG_BRAIN : Z
public static final DEBUG_POI : Z
public static final DEBUG_BEES : Z
public static final DEBUG_RAIDS : Z
public static final DEBUG_BLOCK_BREAK : Z
public static final DEBUG_MONITOR_TICK_TIMES : Z
public static final DEBUG_KEEP_JIGSAW_BLOCKS_DURING_STRUCTURE_GEN : Z
public static final DEBUG_DONT_SAVE_WORLD : Z
public static final DEBUG_LARGE_DRIPSTONE : Z
public static final DEBUG_ORE_VEINS : Z
public static final DEBUG_SCULK_CATALYST : Z
public static final DEBUG_BYPASS_REALMS_VERSION_CHECK : Z
public static final DEBUG_SOCIAL_INTERACTIONS : Z
public static final DEBUG_CHAT_DISABLED : Z
public static final DEBUG_CHAT_FRIENDS_ONLY : Z
public static final DEBUG_VALIDATE_RESOURCE_PATH_CASE : Z
public static final DEBUG_UNLOCK_ALL_TRADES : Z
public static final DEBUG_BREEZE_MOB : Z
public static final DEBUG_TRIAL_SPAWNER_DETECTS_SHEEP_AS_PLAYERS : Z
public static final DEBUG_VAULT_DETECTS_SHEEP_AS_PLAYERS : Z
public static final DEBUG_FORCE_ONBOARDING_SCREEN : Z
public static final DEBUG_CURSOR_POS : Z
public static final DEBUG_DEFAULT_SKIN_OVERRIDE : Z
public static final DEBUG_PANORAMA_SCREENSHOT : Z
public static final DEBUG_CHASE_COMMAND : Z
public static final DEBUG_VERBOSE_COMMAND_ERRORS : Z
public static final DEBUG_DEV_COMMANDS : Z
public static final DEBUG_ACTIVE_TEXT_AREAS : Z
public static final DEBUG_SIMULATE_LIBRARY_LOAD_FAILURE : Z
public static final DEBUG_IGNORE_LOCAL_MOB_CAP : Z
public static final DEBUG_DISABLE_LIQUID_SPREADING : Z
public static final DEBUG_AQUIFERS : Z
public static final DEBUG_JFR_PROFILING_ENABLE_LEVEL_LOADING : Z
public static final DEBUG_ENTITY_BLOCK_INTERSECTION : Z
public static debugGenerateSquareTerrainWithoutNoise : Z
public static final DEBUG_ONLY_GENERATE_HALF_THE_WORLD : Z
public static final DEBUG_DISABLE_FLUID_GENERATION : Z
public static final DEBUG_DISABLE_AQUIFERS : Z
public static final DEBUG_DISABLE_SURFACE : Z
public static final DEBUG_DISABLE_CARVERS : Z
public static final DEBUG_DISABLE_STRUCTURES : Z
public static final DEBUG_DISABLE_FEATURES : Z
public static final DEBUG_DISABLE_ORE_VEINS : Z
public static final DEBUG_DISABLE_BLENDING : Z
public static final DEBUG_DISABLE_BELOW_ZERO_RETROGENERATION : Z
public static final DEFAULT_MINECRAFT_PORT : I
public static final DEBUG_SUBTITLES : Z
public static final DEBUG_FAKE_LATENCY_MS : I
public static final DEBUG_FAKE_JITTER_MS : I
public static final NETTY_LEAK_DETECTION : Lio/netty/util/ResourceLeakDetector$Level;
public static final COMMAND_STACK_TRACES : Z
public static final DEBUG_WORLD_RECREATE : Z
public static final DEBUG_SHOW_SERVER_DEBUG_VALUES : Z
public static final DEBUG_FEATURE_COUNT : Z
public static final DEBUG_CALCULATE_SOLID : Z
public static final DEBUG_FORCE_TELEMETRY : Z
public static final DEBUG_DONT_SEND_TELEMETRY_TO_BACKEND : Z
public static final DEBUG_ENABLE_FARLANDS : Z
public static final MAXIMUM_TICK_TIME_NANOS : J
public static final MAXIMUM_BLOCK_EXPLOSION_RESISTANCE : F
public static final USE_DEVONLY : Z
public static CHECK_DATA_FIXER_SCHEMA : Z
public static IS_RUNNING_IN_IDE : Z
public static IS_RENDERDOC_ATTACHED : Z
public static final WORLD_RESOLUTION : I
public static final MAX_CHAT_LENGTH : I
public static final MAX_USER_INPUT_COMMAND_LENGTH : I
public static final MAX_FUNCTION_COMMAND_LENGTH : I
public static final MAX_PLAYER_NAME_LENGTH : I
public static final MAX_CHAINED_NEIGHBOR_UPDATES : I
public static final MAX_RENDER_DISTANCE : I
public static final MAX_CLOUD_DISTANCE : I
public static final ILLEGAL_FILE_CHARACTERS : [C
public static final TICKS_PER_SECOND : I
public static final MILLIS_PER_TICK : I
public static final TICKS_PER_MINUTE : I
public static final TICKS_PER_GAME_DAY : I
public static final DEFAULT_RANDOM_TICK_SPEED : I
public static final AVERAGE_GAME_TICKS_PER_RANDOM_TICK_PER_BLOCK : F
public static final AVERAGE_RANDOM_TICKS_PER_BLOCK_PER_MINUTE : F
public static final AVERAGE_RANDOM_TICKS_PER_BLOCK_PER_GAME_DAY : F
public static final WORLD_ICON_SIZE : I
private static CURRENT_VERSION : Lnet/minecraft/WorldVersion;
public <init>()V
private static prefixDebugFlagName(Ljava/lang/String;)Ljava/lang/String;
private static booleanProperty(Ljava/lang/String;)Z
private static debugFlag(Ljava/lang/String;)Z
private static debugIntValue(Ljava/lang/String;)I
public static setVersion(Lnet/minecraft/WorldVersion;)V
public static tryDetectVersion()V
public static getCurrentVersion()Lnet/minecraft/WorldVersion;
public static getProtocolVersion()I
public static debugVoidTerrain(Lnet/minecraft/world/level/ChunkPos;)Z
static <clinit>()V
```
