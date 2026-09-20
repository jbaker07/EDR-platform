---
type: "interface"
fqcn: "net.minecraft.server.Main"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.Main

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `main` | `([Ljava/lang/String;)V` | exact | invokestatic@4 in `DedicatedServerImplUtil.lambda$start$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `([Ljava/lang/String;)V` | name_only | @Inject at ['NEW'] | server | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `main` | `([Ljava/lang/String;)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | server | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `([Ljava/lang/String;)V` | name_only | @Inject at ['INVOKE_ASSIGN'] | server | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `([Ljava/lang/String;)V` | name_only | @Inject at ['INVOKE'] | server | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `main` | `([Ljava/lang/String;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `LOGGER` | `Lorg/slf4j/Logger;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| wraps | `main` | `([Ljava/lang/String;)V` | name_only | @WrapWithCondition at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (1 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public <init>()V
public static main([Ljava/lang/String;)V
private static createNewWorldData(Lnet/minecraft/server/dedicated/DedicatedServerSettings;Lnet/minecraft/server/WorldLoader$DataLoadContext;Lnet/minecraft/core/Registry;ZZ)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
private static writePidFile(Ljava/nio/file/Path;)V
private static loadOrCreateConfig(Lnet/minecraft/server/dedicated/DedicatedServerProperties;Lcom/mojang/serialization/Dynamic;ZLnet/minecraft/server/packs/repository/PackRepository;)Lnet/minecraft/server/WorldLoader$InitConfig;
private static forceUpgrade(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/datafixers/DataFixer;ZLjava/util/function/BooleanSupplier;Lnet/minecraft/core/RegistryAccess;Z)V
private static synthetic lambda$main$3(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/dedicated/DedicatedServerSettings;Lnet/minecraft/server/Services;Lnet/minecraft/server/jsonrpc/ManagementServer;Lnet/minecraft/server/notifications/NotificationManager;Ljoptsimple/OptionSet;Ljoptsimple/OptionSpec;Ljoptsimple/OptionSpec;Ljoptsimple/OptionSpec;Ljoptsimple/OptionSpec;Ljoptsimple/OptionSpec;Ljava/lang/Thread;)Lnet/minecraft/server/dedicated/DedicatedServer;
private static synthetic lambda$main$2()Z
private static synthetic lambda$main$0(Lnet/minecraft/server/WorldLoader$InitConfig;Lcom/mojang/serialization/Dynamic;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/dedicated/DedicatedServerSettings;Ljoptsimple/OptionSet;Ljoptsimple/OptionSpec;Ljoptsimple/OptionSpec;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$main$1(Lcom/mojang/serialization/Dynamic;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/dedicated/DedicatedServerSettings;Ljoptsimple/OptionSet;Ljoptsimple/OptionSpec;Ljoptsimple/OptionSpec;Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
static <clinit>()V
```
