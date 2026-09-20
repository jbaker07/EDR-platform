---
type: "interface"
fqcn: "net.minecraft.world.level.storage.loot.LootContext$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.loot.LootContext$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/storage/loot/LootParams;)V` | exact | invokespecial@35 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/Optional;)Lnet/minecraft/world/level/storage/loot/LootCont` | exact | invokevirtual@41 in `ComposterWrapper.getLayersToAdd` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final params : Lnet/minecraft/world/level/storage/loot/LootParams;
private random : Lnet/minecraft/util/RandomSource;
public <init>(Lnet/minecraft/world/level/storage/loot/LootParams;)V
public withOptionalRandomSeed(J)Lnet/minecraft/world/level/storage/loot/LootContext$Builder;
public withOptionalRandomSource(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/storage/loot/LootContext$Builder;
public getLevel()Lnet/minecraft/server/level/ServerLevel;
public create(Ljava/util/Optional;)Lnet/minecraft/world/level/storage/loot/LootContext;
private static synthetic lambda$create$0(Ljava/util/Optional;Lnet/minecraft/server/MinecraftServer;)Ljava/util/Optional;
```
