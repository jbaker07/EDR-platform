---
type: "interface"
fqcn: "net.minecraft.server.ReloadableServerRegistries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.ReloadableServerRegistries

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `reload` | `(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecr` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| wraps | `reload` | `(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecr` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| wraps | `reload` | `(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecr` | name_only | @Redirect at ['FIELD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DEFAULT_REGISTRATION_INFO : Lnet/minecraft/core/RegistrationInfo;
public <init>()V
public static reload(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static createAndValidateFullContext(Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/RegistryAccess$Frozen;)Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;
private static concatenateLookups(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/HolderLookup$Provider;
private static validateLootRegistries(Lnet/minecraft/core/HolderLookup$Provider;)V
private static synthetic lambda$validateLootRegistries$1(Ljava/lang/String;Lnet/minecraft/util/ProblemReporter$Problem;)V
private static synthetic lambda$validateLootRegistries$0(Lnet/minecraft/world/level/storage/loot/ValidationContextSource;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/level/storage/loot/LootDataType;)V
private static synthetic lambda$reload$0(Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/RegistryAccess$Frozen;)Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;
static <clinit>()V
```
