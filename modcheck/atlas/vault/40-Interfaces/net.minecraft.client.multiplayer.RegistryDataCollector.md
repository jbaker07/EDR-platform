---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.RegistryDataCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.RegistryDataCollector

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `loadNewElementsAndTags` | `(Lnet/minecraft/server/packs/resources/ResourceProvider;Lnet/minecraft` | name_only | @WrapOperation at ['FIELD'] | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (3 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private contentsCollector : Lnet/minecraft/client/multiplayer/RegistryDataCollector$ContentsCollector;
private tagCollector : Lnet/minecraft/client/multiplayer/RegistryDataCollector$TagCollector;
public <init>()V
public appendContents(Lnet/minecraft/resources/ResourceKey;Ljava/util/List;)V
public appendTags(Ljava/util/Map;)V
private static resolveRegistryTags(Lnet/minecraft/core/RegistryAccess$Frozen;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;)Lnet/minecraft/core/Registry$PendingTags;
private loadNewElementsAndTags(Lnet/minecraft/server/packs/resources/ResourceProvider;Lnet/minecraft/client/multiplayer/RegistryDataCollector$ContentsCollector;Z)Lnet/minecraft/core/RegistryAccess;
private static addCrashDetails(Lnet/minecraft/CrashReport;Ljava/util/Map;Ljava/util/List;)V
private static loadOnlyTags(Lnet/minecraft/client/multiplayer/RegistryDataCollector$TagCollector;Lnet/minecraft/core/RegistryAccess$Frozen;Z)V
private static updateComponents(Lnet/minecraft/core/RegistryAccess$Frozen;Z)V
public collectGameRegistries(Lnet/minecraft/server/packs/resources/ResourceProvider;Lnet/minecraft/core/RegistryAccess$Frozen;Z)Lnet/minecraft/core/RegistryAccess$Frozen;
private static synthetic lambda$updateComponents$0(ZLnet/minecraft/core/component/DataComponentInitializers$PendingComponents;)V
private static synthetic lambda$loadOnlyTags$0(ZLnet/minecraft/core/RegistryAccess$Frozen;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;)V
private static synthetic lambda$addCrashDetails$3(Ljava/util/List;)Ljava/lang/String;
private static synthetic lambda$addCrashDetails$5(Lnet/minecraft/core/Registry$PendingTags;)Ljava/lang/String;
private static synthetic lambda$addCrashDetails$4(Lnet/minecraft/core/Registry$PendingTags;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$addCrashDetails$0(Ljava/util/Map;)Ljava/lang/String;
private static synthetic lambda$addCrashDetails$2(Ljava/util/Map$Entry;)Ljava/lang/String;
private static synthetic lambda$addCrashDetails$1(Ljava/util/Map$Entry;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$loadNewElementsAndTags$1(Ljava/util/Map;ZLjava/util/List;Lnet/minecraft/core/RegistryAccess$Frozen;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;)V
private static synthetic lambda$loadNewElementsAndTags$2(Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/RegistryDataLoader$NetworkedRegistryData;)Lnet/minecraft/resources/RegistryDataLoader$NetworkedRegistryData;
private static synthetic lambda$loadNewElementsAndTags$0(Ljava/util/Map;Lnet/minecraft/resources/ResourceKey;Ljava/util/List;)V
static <clinit>()V
```
