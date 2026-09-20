---
type: "interface"
fqcn: "net.minecraft.server.WorldLoader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.WorldLoader

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$load$0` | `(Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$Worl` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (0 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static load(Lnet/minecraft/server/WorldLoader$InitConfig;Lnet/minecraft/server/WorldLoader$WorldDataSupplier;Lnet/minecraft/server/WorldLoader$ResultFactory;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$load$0(Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$WorldDataSupplier;Lnet/minecraft/server/WorldLoader$InitConfig;Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$ResultFactory;Lcom/mojang/datafixers/util/Pair;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$load$1(Ljava/util/List;Lnet/minecraft/server/packs/resources/CloseableResourceManager;Ljava/util/concurrent/Executor;Lcom/mojang/datafixers/util/Pair;Lnet/minecraft/server/WorldLoader$WorldDataSupplier;Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/server/WorldLoader$InitConfig;Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$ResultFactory;Lnet/minecraft/core/RegistryAccess$Frozen;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$load$2(Lcom/mojang/datafixers/util/Pair;Ljava/util/List;Lnet/minecraft/server/WorldLoader$WorldDataSupplier;Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/core/RegistryAccess$Frozen;Ljava/util/List;Lnet/minecraft/server/WorldLoader$InitConfig;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$ResultFactory;Lnet/minecraft/core/RegistryAccess$Frozen;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$load$4(Lnet/minecraft/server/WorldLoader$ResultFactory;Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/server/WorldLoader$DataLoadOutput;Lnet/minecraft/server/ReloadableServerResources;)Ljava/lang/Object;
private static synthetic lambda$load$3(Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;Ljava/lang/Throwable;)V
```
