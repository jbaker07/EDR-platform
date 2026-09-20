---
type: "interface"
fqcn: "net.minecraft.server.PlayerAdvancements"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.PlayerAdvancements

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `award` | `(Lnet/minecraft/advancements/AdvancementHolder;Ljava/lang/String;)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `setPlayer` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |

## Declared members (14 fields, 31 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final GSON : Lcom/google/gson/Gson;
private final playerList : Lnet/minecraft/server/players/PlayerList;
private final playerSavePath : Ljava/nio/file/Path;
private tree : Lnet/minecraft/advancements/AdvancementTree;
private final progress : Ljava/util/Map;
private final visible : Ljava/util/Set;
private final progressChanged : Ljava/util/Set;
private final rootsToUpdate : Ljava/util/Set;
private player : Lnet/minecraft/server/level/ServerPlayer;
private lastSelectedTab : Lnet/minecraft/advancements/AdvancementHolder;
private isFirstPacket : Z
private final codec : Lcom/mojang/serialization/Codec;
private final activeTriggers : Ljava/util/Map;
public <init>(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/players/PlayerList;Lnet/minecraft/server/ServerAdvancementManager;Ljava/nio/file/Path;Lnet/minecraft/server/level/ServerPlayer;)V
public setPlayer(Lnet/minecraft/server/level/ServerPlayer;)V
public clearTriggers()V
public reload(Lnet/minecraft/server/ServerAdvancementManager;)V
private registerListeners(Lnet/minecraft/server/ServerAdvancementManager;)V
private checkForAutomaticTriggers(Lnet/minecraft/server/ServerAdvancementManager;)V
private load(Lnet/minecraft/server/ServerAdvancementManager;)V
public save()V
private applyFrom(Lnet/minecraft/server/ServerAdvancementManager;Lnet/minecraft/server/PlayerAdvancements$Data;)V
private asData()Lnet/minecraft/server/PlayerAdvancements$Data;
public award(Lnet/minecraft/advancements/AdvancementHolder;Ljava/lang/String;)Z
public revoke(Lnet/minecraft/advancements/AdvancementHolder;Ljava/lang/String;)Z
private markForVisibilityUpdate(Lnet/minecraft/advancements/AdvancementHolder;)V
private registerListeners(Lnet/minecraft/advancements/AdvancementHolder;)V
private unregisterListeners(Lnet/minecraft/advancements/AdvancementHolder;)V
public flushDirty(Lnet/minecraft/server/level/ServerPlayer;Z)V
public setSelectedTab(Lnet/minecraft/advancements/AdvancementHolder;)V
public getOrStartProgress(Lnet/minecraft/advancements/AdvancementHolder;)Lnet/minecraft/advancements/AdvancementProgress;
private startProgress(Lnet/minecraft/advancements/AdvancementHolder;Lnet/minecraft/advancements/AdvancementProgress;)V
private updateTreeVisibility(Lnet/minecraft/advancements/AdvancementNode;Ljava/util/Set;Ljava/util/Set;)V
private getOrCreateTriggerMapForType(Lnet/minecraft/advancements/triggers/CriterionTrigger;)Ljava/util/Map;
private addListener(Lnet/minecraft/advancements/triggers/Criterion;Lnet/minecraft/server/PlayerAdvancements$TriggerInstanceKey;)V
public getTriggerMapForType(Lnet/minecraft/advancements/triggers/CriterionTrigger;)Ljava/util/Map;
private removeListener(Lnet/minecraft/advancements/triggers/CriterionTrigger;Lnet/minecraft/server/PlayerAdvancements$TriggerInstanceKey;)V
private static synthetic lambda$getOrCreateTriggerMapForType$0(Lnet/minecraft/advancements/triggers/CriterionTrigger;)Ljava/util/Map;
private synthetic lambda$updateTreeVisibility$1(Ljava/util/Set;Ljava/util/Set;Lnet/minecraft/advancements/AdvancementNode;Z)V
private synthetic lambda$updateTreeVisibility$0(Lnet/minecraft/advancements/AdvancementNode;)Z
private synthetic lambda$award$0(Lnet/minecraft/advancements/AdvancementHolder;Lnet/minecraft/advancements/DisplayInfo;)V
private static synthetic lambda$asData$0(Ljava/util/Map;Lnet/minecraft/advancements/AdvancementHolder;Lnet/minecraft/advancements/AdvancementProgress;)V
private synthetic lambda$applyFrom$0(Lnet/minecraft/server/ServerAdvancementManager;Lnet/minecraft/resources/Identifier;Lnet/minecraft/advancements/AdvancementProgress;)V
static <clinit>()V
```
