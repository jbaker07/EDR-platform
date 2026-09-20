---
type: "interface"
fqcn: "net.minecraft.util.debug.ServerDebugSubscribers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.debug.ServerDebugSubscribers

System: [[20-Systems/net.minecraft.util.debug|net.minecraft.util.debug]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `hasRequiredPermissions` | `(Lnet/minecraft/server/level/ServerPlayer;)Z` | name_only | @WrapOperation at ['MIXINEXTRAS:EXPRESSION'] | both | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final server : Lnet/minecraft/server/MinecraftServer;
private final enabledSubscriptions : Ljava/util/Map;
public <init>(Lnet/minecraft/server/MinecraftServer;)V
private getSubscribersFor(Lnet/minecraft/util/debug/DebugSubscription;)Ljava/util/List;
public tick()V
public broadcastToAll(Lnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/network/protocol/Packet;)V
public enabledSubscriptions()Ljava/util/Set;
public hasAnySubscriberFor(Lnet/minecraft/util/debug/DebugSubscription;)Z
public hasRequiredPermissions(Lnet/minecraft/server/level/ServerPlayer;)Z
private static synthetic lambda$tick$0(Lnet/minecraft/util/debug/DebugSubscription;)Ljava/util/List;
```
