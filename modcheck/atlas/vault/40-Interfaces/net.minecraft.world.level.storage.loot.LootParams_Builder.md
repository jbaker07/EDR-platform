---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootParams$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootParams$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/server/level/ServerLevel;)V` | exact | invokespecial@9 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/world/level` | exact | invokevirtual@32 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `withParameter` | `(Lnet/minecraft/util/context/ContextKey;Ljava/lang/Object;)Lnet/minecr` | exact | invokevirtual@16 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `withParameter` | `(Lnet/minecraft/util/context/ContextKey;Ljava/lang/Object;)Lnet/minecr` | exact | invokevirtual@26 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final level : Lnet/minecraft/server/level/ServerLevel;
private final params : Lnet/minecraft/util/context/ContextMap$Builder;
private final dynamicDrops : Ljava/util/Map;
private luck : F
public <init>(Lnet/minecraft/server/level/ServerLevel;)V
public getLevel()Lnet/minecraft/server/level/ServerLevel;
public withParameter(Lnet/minecraft/util/context/ContextKey;Ljava/lang/Object;)Lnet/minecraft/world/level/storage/loot/LootParams$Builder;
public withOptionalParameter(Lnet/minecraft/util/context/ContextKey;Ljava/lang/Object;)Lnet/minecraft/world/level/storage/loot/LootParams$Builder;
public getParameter(Lnet/minecraft/util/context/ContextKey;)Ljava/lang/Object;
public getOptionalParameter(Lnet/minecraft/util/context/ContextKey;)Ljava/lang/Object;
public withDynamicDrop(Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/level/storage/loot/LootParams$DynamicDrop;)Lnet/minecraft/world/level/storage/loot/LootParams$Builder;
public withLuck(F)Lnet/minecraft/world/level/storage/loot/LootParams$Builder;
public create(Lnet/minecraft/util/context/ContextKeySet;)Lnet/minecraft/world/level/storage/loot/LootParams;
```
