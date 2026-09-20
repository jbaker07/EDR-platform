---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientDebugSubscriber"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientDebugSubscriber

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `requestedSubscriptions` | `()Ljava/util/Set;` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (4 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final connection : Lnet/minecraft/client/multiplayer/ClientPacketListener;
private final debugScreenOverlay : Lnet/minecraft/client/gui/components/DebugScreenOverlay;
private remoteSubscriptions : Ljava/util/Set;
private final valuesBySubscription : Ljava/util/Map;
public <init>(Lnet/minecraft/client/multiplayer/ClientPacketListener;Lnet/minecraft/client/gui/components/DebugScreenOverlay;)V
private static addFlag(Ljava/util/Set;Lnet/minecraft/util/debug/DebugSubscription;Z)V
private requestedSubscriptions()Ljava/util/Set;
public clear()V
public tick(J)V
private onSubscriptionsChanged(Ljava/util/Set;)V
private initializeSubscriptions(Ljava/util/Set;)V
private getValueMaps(Lnet/minecraft/util/debug/DebugSubscription;)Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMaps;
private getValueMap(Lnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;)Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMap;
private getValue(Lnet/minecraft/util/debug/DebugSubscription;Ljava/lang/Object;Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;)Ljava/lang/Object;
public createDebugValueAccess(Lnet/minecraft/world/level/Level;)Lnet/minecraft/util/debug/DebugValueAccess;
public updateChunk(JLnet/minecraft/world/level/ChunkPos;Lnet/minecraft/util/debug/DebugSubscription$Update;)V
public updateBlock(JLnet/minecraft/core/BlockPos;Lnet/minecraft/util/debug/DebugSubscription$Update;)V
public updateEntity(JLnet/minecraft/world/entity/Entity;Lnet/minecraft/util/debug/DebugSubscription$Update;)V
public pushEvent(JLnet/minecraft/util/debug/DebugSubscription$Event;)V
private updateMap(JLjava/lang/Object;Lnet/minecraft/util/debug/DebugSubscription$Update;Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;)V
private forEachValue(Lnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;Ljava/util/function/BiConsumer;)V
public dropLevel()V
public dropChunk(Lnet/minecraft/world/level/ChunkPos;)V
public dropEntity(Lnet/minecraft/world/entity/Entity;)V
private static entities()Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;
private static blocks()Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;
private static chunks()Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMapType;
private static synthetic lambda$chunks$0(Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMaps;)Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMap;
private static synthetic lambda$blocks$0(Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMaps;)Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMap;
private static synthetic lambda$entities$0(Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMaps;)Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMap;
private static synthetic lambda$initializeSubscriptions$0(Lnet/minecraft/util/debug/DebugSubscription;)Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMaps;
private static synthetic lambda$tick$0(JLnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/client/multiplayer/ClientDebugSubscriber$ValueMaps;)V
```
