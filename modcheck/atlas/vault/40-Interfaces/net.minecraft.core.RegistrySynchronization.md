---
type: "interface"
fqcn: "net.minecraft.core.RegistrySynchronization"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistrySynchronization

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$ownedNetworkableRegistries$0` | `(Lnet/minecraft/core/RegistryAccess$RegistryEntry;)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `lambda$packRegistry$0` | `(Ljava/util/Set;Lnet/minecraft/resources/RegistryDataLoader$RegistryDa` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NETWORKABLE_REGISTRIES` | `Ljava/util/Set;` | exact | getstatic@64 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NETWORKABLE_REGISTRIES` | `Ljava/util/Set;` | exact | getstatic@77 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NETWORKABLE_REGISTRIES` | `Ljava/util/Set;` | exact | getstatic@86 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| writes | `NETWORKABLE_REGISTRIES` | `Ljava/util/Set;` | exact | putstatic@83 in `DynamicRegistriesImpl.addSyncedRegistry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (1 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final NETWORKABLE_REGISTRIES : Ljava/util/Set;
public <init>()V
public static packRegistries(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/core/RegistryAccess;Ljava/util/Set;Ljava/util/function/BiConsumer;)V
private static packRegistry(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lnet/minecraft/core/RegistryAccess;Ljava/util/Set;Ljava/util/function/BiConsumer;)V
private static ownedNetworkableRegistries(Lnet/minecraft/core/RegistryAccess;)Ljava/util/stream/Stream;
public static networkedRegistries(Lnet/minecraft/core/LayeredRegistryAccess;)Ljava/util/stream/Stream;
public static networkSafeRegistries(Lnet/minecraft/core/LayeredRegistryAccess;)Ljava/util/stream/Stream;
public static isNetworkable(Lnet/minecraft/resources/ResourceKey;)Z
private static synthetic lambda$ownedNetworkableRegistries$0(Lnet/minecraft/core/RegistryAccess$RegistryEntry;)Z
private static synthetic lambda$packRegistry$0(Ljava/util/Set;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lcom/mojang/serialization/DynamicOps;Ljava/util/function/BiConsumer;Lnet/minecraft/core/Registry;)V
private static synthetic lambda$packRegistry$1(Lnet/minecraft/core/Registry;Ljava/util/Set;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lcom/mojang/serialization/DynamicOps;Ljava/util/List;Lnet/minecraft/core/Holder$Reference;)V
private static synthetic lambda$packRegistry$2(Lnet/minecraft/core/Holder$Reference;Ljava/lang/String;)Ljava/lang/IllegalArgumentException;
private static synthetic lambda$packRegistries$0(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/core/RegistryAccess;Ljava/util/Set;Ljava/util/function/BiConsumer;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;)V
static <clinit>()V
```
