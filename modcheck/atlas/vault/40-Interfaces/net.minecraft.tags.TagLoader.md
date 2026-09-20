---
type: "interface"
fqcn: "net.minecraft.tags.TagLoader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagLoader

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `build` | `(Ljava/util/Map;)Ljava/util/Map;` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `lambda$build$0` | `(Lnet/minecraft/util/DependencySorter;Lnet/minecraft/resources/Identif` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| wraps | `lambda$build$1` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/Map;Lnet/minecraft/res` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| wraps | `loadTagsForRegistry` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| wraps | `tryBuildTag` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/List;)Lcom/mojang/data` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (3 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final elementLookup : Lnet/minecraft/tags/TagLoader$ElementLookup;
private final directory : Ljava/lang/String;
public <init>(Lnet/minecraft/tags/TagLoader$ElementLookup;Ljava/lang/String;)V
public load(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;
private tryBuildTag(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/List;)Lcom/mojang/datafixers/util/Either;
public build(Ljava/util/Map;)Ljava/util/Map;
public static loadTagsFromNetwork(Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;Lnet/minecraft/core/Registry;)Ljava/util/Map;
public static loadTagsForExistingRegistries(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/RegistryAccess;)Ljava/util/List;
public static loadTagsForRegistry(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/WritableRegistry;)V
public static loadTagsForRegistry(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagLoader$ElementLookup;)Ljava/util/Map;
private static wrapTags(Lnet/minecraft/resources/ResourceKey;Ljava/util/Map;)Ljava/util/Map;
private static loadPendingTags(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/Registry;)Ljava/util/Optional;
public static buildUpdatedLookups(Lnet/minecraft/core/RegistryAccess$Frozen;Ljava/util/List;)Ljava/util/List;
private static findTagsForRegistry(Ljava/util/List;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry$PendingTags;
private static synthetic lambda$buildUpdatedLookups$0(Ljava/util/List;Ljava/util/List;Lnet/minecraft/core/RegistryAccess$RegistryEntry;)V
private static synthetic lambda$wrapTags$0(Lnet/minecraft/resources/ResourceKey;Ljava/util/Map$Entry;)Lnet/minecraft/tags/TagKey;
private static synthetic lambda$loadTagsForExistingRegistries$0(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/RegistryAccess$RegistryEntry;)Ljava/util/Optional;
private synthetic lambda$build$1(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/Map;Lnet/minecraft/resources/Identifier;Lnet/minecraft/tags/TagLoader$SortingEntry;)V
private static synthetic lambda$build$3(Ljava/util/Map;Lnet/minecraft/resources/Identifier;Ljava/util/List;)V
private static synthetic lambda$build$2(Lnet/minecraft/resources/Identifier;Ljava/util/List;)V
private static synthetic lambda$build$0(Lnet/minecraft/util/DependencySorter;Lnet/minecraft/resources/Identifier;Ljava/util/List;)V
private static synthetic lambda$load$1(Ljava/util/List;Ljava/lang/String;Lnet/minecraft/tags/TagEntry;)V
private static synthetic lambda$load$0(Lnet/minecraft/resources/Identifier;)Ljava/util/List;
static <clinit>()V
```
