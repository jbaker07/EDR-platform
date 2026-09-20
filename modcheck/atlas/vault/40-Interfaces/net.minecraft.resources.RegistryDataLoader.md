---
type: "interface"
fqcn: "net.minecraft.resources.RegistryDataLoader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryDataLoader

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$load$0` | `(Ljava/util/List;Lnet/minecraft/resources/RegistryDataLoader$LoaderFac` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `lambda$load$2` | `(Ljava/util/List;Ljava/util/Map;Ljava/lang/Void;)Lnet/minecraft/core/R` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `load` | `(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `DIMENSION_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@62 in `DynamicRegistriesImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RELOADABLE_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@17 in `DynamicRegistriesImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RELOADABLE_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@74 in `DynamicRegistriesImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `SYNCHRONIZED_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@21 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `SYNCHRONIZED_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@34 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `SYNCHRONIZED_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@43 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `WORLD_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@4 in `DynamicRegistriesImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `WORLD_REGISTRIES` | `Ljava/util/List;` | exact | getstatic@50 in `DynamicRegistriesImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `lambda$load$0` | `(Ljava/util/List;Lnet/minecraft/resources/RegistryDataLoader$LoaderFac` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| writes | `SYNCHRONIZED_REGISTRIES` | `Ljava/util/List;` | exact | putstatic@40 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (6 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final ERROR_KEY_COMPARATOR : Ljava/util/Comparator;
public static final WORLD_REGISTRIES : Ljava/util/List;
public static final DIMENSION_REGISTRIES : Ljava/util/List;
public static final RELOADABLE_REGISTRIES : Ljava/util/List;
public static final SYNCHRONIZED_REGISTRIES : Ljava/util/List;
public <init>()V
public static load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
public static load(Ljava/util/Map;Lnet/minecraft/server/packs/resources/ResourceProvider;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static load(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static createContext(Ljava/util/List;Ljava/util/List;)Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;
private static logErrors(Ljava/util/Map;)Lnet/minecraft/ReportedException;
private static printFullDetailsToLog(Ljava/util/Map;)V
private static createReportWithBriefInfo(Ljava/util/Map;)Lnet/minecraft/ReportedException;
private static synthetic lambda$createReportWithBriefInfo$0(Ljava/util/Map;)Ljava/lang/String;
private static synthetic lambda$createReportWithBriefInfo$1(Ljava/lang/StringBuilder;Ljava/util/Map$Entry;)V
private static synthetic lambda$printFullDetailsToLog$2(Ljava/io/PrintWriter;Ljava/util/Map$Entry;)V
private static synthetic lambda$printFullDetailsToLog$3(Ljava/io/PrintWriter;Ljava/util/Map$Entry;)V
private static synthetic lambda$printFullDetailsToLog$1(Ljava/util/Map$Entry;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$printFullDetailsToLog$0(Ljava/util/Map$Entry;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$createContext$1(Ljava/util/Map;Lnet/minecraft/resources/RegistryLoadTask;)V
private static synthetic lambda$createContext$0(Ljava/util/Map;Lnet/minecraft/core/HolderLookup$RegistryLookup;)V
private static synthetic lambda$load$5(Ljava/util/concurrent/CompletableFuture;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$load$0(Ljava/util/List;Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$load$2(Ljava/util/List;Ljava/util/Map;Ljava/lang/Void;)Lnet/minecraft/core/RegistryAccess$Frozen;
private static synthetic lambda$load$4(Ljava/util/Map;Lnet/minecraft/resources/RegistryLoadTask;)Ljava/util/stream/Stream;
private static synthetic lambda$load$3(Ljava/util/Map;Lnet/minecraft/resources/RegistryLoadTask;)Z
private static synthetic lambda$load$1(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/Map;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;)Lnet/minecraft/resources/RegistryLoadTask;
static <clinit>()V
```
