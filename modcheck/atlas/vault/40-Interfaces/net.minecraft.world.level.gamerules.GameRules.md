---
type: "interface"
fqcn: "net.minecraft.world.level.gamerules.GameRules"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gamerules.GameRules

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/Object;` | exact | invokevirtual@52 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/Object;` | exact | invokevirtual@18 in `EnumRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/Object;` | exact | invokevirtual@42 in `EnumRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getAsString` | `(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/String;` | exact | invokevirtual@78 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@59 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@74 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@89 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@104 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@21 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@51 in `DoubleRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/` | exact | invokevirtual@30 in `EnumRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `ADVANCE_TIME` | `Lnet/minecraft/world/level/gamerules/GameRule;` | exact | getstatic@51 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `ADVANCE_WEATHER` | `Lnet/minecraft/world/level/gamerules/GameRule;` | exact | getstatic@66 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `RESPAWN_RADIUS` | `Lnet/minecraft/world/level/gamerules/GameRule;` | exact | getstatic@96 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `SPAWN_MOBS` | `Lnet/minecraft/world/level/gamerules/GameRule;` | exact | getstatic@81 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (61 fields, 27 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final ADVANCE_TIME : Lnet/minecraft/world/level/gamerules/GameRule;
public static final ADVANCE_WEATHER : Lnet/minecraft/world/level/gamerules/GameRule;
public static final ALLOW_ENTERING_NETHER_USING_PORTALS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final BLOCK_DROPS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final BLOCK_EXPLOSION_DROP_DECAY : Lnet/minecraft/world/level/gamerules/GameRule;
public static final COMMAND_BLOCKS_WORK : Lnet/minecraft/world/level/gamerules/GameRule;
public static final COMMAND_BLOCK_OUTPUT : Lnet/minecraft/world/level/gamerules/GameRule;
public static final DROWNING_DAMAGE : Lnet/minecraft/world/level/gamerules/GameRule;
public static final ELYTRA_MOVEMENT_CHECK : Lnet/minecraft/world/level/gamerules/GameRule;
public static final ENDER_PEARLS_VANISH_ON_DEATH : Lnet/minecraft/world/level/gamerules/GameRule;
public static final ENTITY_DROPS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final FALL_DAMAGE : Lnet/minecraft/world/level/gamerules/GameRule;
public static final FIRE_DAMAGE : Lnet/minecraft/world/level/gamerules/GameRule;
public static final FIRE_SPREAD_RADIUS_AROUND_PLAYER : Lnet/minecraft/world/level/gamerules/GameRule;
public static final FORGIVE_DEAD_PLAYERS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final FREEZE_DAMAGE : Lnet/minecraft/world/level/gamerules/GameRule;
public static final GLOBAL_SOUND_EVENTS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final IMMEDIATE_RESPAWN : Lnet/minecraft/world/level/gamerules/GameRule;
public static final KEEP_INVENTORY : Lnet/minecraft/world/level/gamerules/GameRule;
public static final LAVA_SOURCE_CONVERSION : Lnet/minecraft/world/level/gamerules/GameRule;
public static final LIMITED_CRAFTING : Lnet/minecraft/world/level/gamerules/GameRule;
public static final LOCATOR_BAR : Lnet/minecraft/world/level/gamerules/GameRule;
public static final LOG_ADMIN_COMMANDS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MAX_BLOCK_MODIFICATIONS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MAX_COMMAND_FORKS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MAX_COMMAND_SEQUENCE_LENGTH : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MAX_ENTITY_CRAMMING : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MAX_MINECART_SPEED : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MAX_SNOW_ACCUMULATION_HEIGHT : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MOB_DROPS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MOB_EXPLOSION_DROP_DECAY : Lnet/minecraft/world/level/gamerules/GameRule;
public static final MOB_GRIEFING : Lnet/minecraft/world/level/gamerules/GameRule;
public static final NATURAL_HEALTH_REGENERATION : Lnet/minecraft/world/level/gamerules/GameRule;
public static final PLAYER_MOVEMENT_CHECK : Lnet/minecraft/world/level/gamerules/GameRule;
public static final PLAYERS_NETHER_PORTAL_CREATIVE_DELAY : Lnet/minecraft/world/level/gamerules/GameRule;
public static final PLAYERS_NETHER_PORTAL_DEFAULT_DELAY : Lnet/minecraft/world/level/gamerules/GameRule;
public static final PLAYERS_SLEEPING_PERCENTAGE : Lnet/minecraft/world/level/gamerules/GameRule;
public static final PROJECTILES_CAN_BREAK_BLOCKS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final PVP : Lnet/minecraft/world/level/gamerules/GameRule;
public static final RAIDS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final RANDOM_TICK_SPEED : Lnet/minecraft/world/level/gamerules/GameRule;
public static final REDUCED_DEBUG_INFO : Lnet/minecraft/world/level/gamerules/GameRule;
public static final RESPAWN_RADIUS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SEND_COMMAND_FEEDBACK : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SHOW_ADVANCEMENT_MESSAGES : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SHOW_DEATH_MESSAGES : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWNER_BLOCKS_WORK : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWN_MOBS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWN_MONSTERS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWN_PATROLS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWN_PHANTOMS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWN_WANDERING_TRADERS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPAWN_WARDENS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPECTATORS_GENERATE_CHUNKS : Lnet/minecraft/world/level/gamerules/GameRule;
public static final SPREAD_VINES : Lnet/minecraft/world/level/gamerules/GameRule;
public static final TNT_EXPLODES : Lnet/minecraft/world/level/gamerules/GameRule;
public static final TNT_EXPLOSION_DROP_DECAY : Lnet/minecraft/world/level/gamerules/GameRule;
public static final UNIVERSAL_ANGER : Lnet/minecraft/world/level/gamerules/GameRule;
public static final WATER_SOURCE_CONVERSION : Lnet/minecraft/world/level/gamerules/GameRule;
private final rules : Lnet/minecraft/world/level/gamerules/GameRuleMap;
public static codec(Lnet/minecraft/world/flag/FeatureFlagSet;)Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/gamerules/GameRuleMap;)V
public <init>(Lnet/minecraft/world/flag/FeatureFlagSet;)V
public <init>(Ljava/util/List;)V
public availableRules()Ljava/util/stream/Stream;
public get(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/Object;
public set(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/minecraft/server/MinecraftServer;)V
public copy(Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/world/level/gamerules/GameRules;
public setAll(Lnet/minecraft/world/level/gamerules/GameRules;Lnet/minecraft/server/MinecraftServer;)V
public setAll(Lnet/minecraft/world/level/gamerules/GameRuleMap;Lnet/minecraft/server/MinecraftServer;)V
private setFromOther(Lnet/minecraft/world/level/gamerules/GameRuleMap;Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/server/MinecraftServer;)V
public visitGameRuleTypes(Lnet/minecraft/world/level/gamerules/GameRuleTypeVisitor;)V
private static registerBoolean(Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRuleCategory;Z)Lnet/minecraft/world/level/gamerules/GameRule;
private static registerInteger(Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRuleCategory;II)Lnet/minecraft/world/level/gamerules/GameRule;
private static registerInteger(Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRuleCategory;III)Lnet/minecraft/world/level/gamerules/GameRule;
private static registerInteger(Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRuleCategory;IIILnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/world/level/gamerules/GameRule;
private static register(Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRuleCategory;Lnet/minecraft/world/level/gamerules/GameRuleType;Lcom/mojang/brigadier/arguments/ArgumentType;Lcom/mojang/serialization/Codec;Ljava/lang/Object;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/gamerules/GameRules$VisitorCaller;Ljava/util/function/ToIntFunction;)Lnet/minecraft/world/level/gamerules/GameRule;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/level/gamerules/GameRule;
public getAsString(Lnet/minecraft/world/level/gamerules/GameRule;)Ljava/lang/String;
private static synthetic lambda$registerInteger$0(Ljava/lang/Integer;)I
private static synthetic lambda$registerBoolean$0(Ljava/lang/Boolean;)I
private static synthetic lambda$visitGameRuleTypes$0(Lnet/minecraft/world/level/gamerules/GameRuleTypeVisitor;Lnet/minecraft/world/level/gamerules/GameRule;)V
private synthetic lambda$setAll$0(Lnet/minecraft/world/level/gamerules/GameRuleMap;Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/level/gamerules/GameRule;)V
private static synthetic lambda$new$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/gamerules/GameRuleMap;Lnet/minecraft/world/level/gamerules/GameRule;)V
private static synthetic lambda$codec$1(Lnet/minecraft/world/level/gamerules/GameRules;)Lnet/minecraft/world/level/gamerules/GameRuleMap;
private static synthetic lambda$codec$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/gamerules/GameRuleMap;)Lnet/minecraft/world/level/gamerules/GameRules;
static <clinit>()V
```
