---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/WritableRegistry`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecy` | exact | invokespecial@9 in `FabricRegistryBuilder.create` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `containsKey` | `(Lnet/minecraft/resources/Identifier;)Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| calls | `createTag` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |
| calls | `refreshTagsInHolders` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |
| calls | `toString` | `()Ljava/lang/String;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| calls | `validateWrite` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| injects_into | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecy` | exact | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `containsKey` | `(Lnet/minecraft/resources/Identifier;)Z` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `containsKey` | `(Lnet/minecraft/resources/ResourceKey;)Z` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `get` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `getOrCreateHolderOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | name_only | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `getValue` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `register` | `?` | ambiguous | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `registrationInfo` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | name_only | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `allTags` | `Lnet/minecraft/core/MappedRegistry$TagSet;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |
| reads | `byId` | `Lit/unimi/dsi/fastutil/objects/ObjectList;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `byKey` | `Ljava/util/Map;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `byLocation` | `Ljava/util/Map;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `key` | `Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `key` | `Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |
| reads | `registrationInfos` | `Ljava/util/Map;` | exact | getfield@307 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `registrationInfos` | `Ljava/util/Map;` | exact | getfield@341 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `toId` | `Lit/unimi/dsi/fastutil/objects/Reference2IntMap;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |

## Declared members (13 fields, 60 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final key : Lnet/minecraft/resources/ResourceKey;
private final byId : Lit/unimi/dsi/fastutil/objects/ObjectList;
private final toId : Lit/unimi/dsi/fastutil/objects/Reference2IntMap;
private final byLocation : Ljava/util/Map;
private final byKey : Ljava/util/Map;
private final byValue : Ljava/util/Map;
private final registrationInfos : Ljava/util/Map;
private registryLifecycle : Lcom/mojang/serialization/Lifecycle;
private final frozenTags : Ljava/util/Map;
private allTags : Lnet/minecraft/core/MappedRegistry$TagSet;
private componentLookup : Lnet/minecraft/core/component/DataComponentLookup;
private frozen : Z
private unregisteredIntrusiveHolders : Ljava/util/Map;
public listTags()Ljava/util/stream/Stream;
public <init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;)V
public <init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;Z)V
public key()Lnet/minecraft/resources/ResourceKey;
public toString()Ljava/lang/String;
private validateWrite()V
private validateWrite(Lnet/minecraft/resources/ResourceKey;)V
public register(Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;Lnet/minecraft/core/RegistrationInfo;)Lnet/minecraft/core/Holder$Reference;
public getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;
public getResourceKey(Ljava/lang/Object;)Ljava/util/Optional;
public getId(Ljava/lang/Object;)I
public getValue(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;
public byId(I)Ljava/lang/Object;
public get(I)Ljava/util/Optional;
public get(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public getAny()Ljava/util/Optional;
public wrapAsHolder(Ljava/lang/Object;)Lnet/minecraft/core/Holder;
private getOrCreateHolderOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Reference;
public size()I
public registrationInfo(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public registryLifecycle()Lcom/mojang/serialization/Lifecycle;
public iterator()Ljava/util/Iterator;
public getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;
private static getValueFromNullable(Lnet/minecraft/core/Holder$Reference;)Ljava/lang/Object;
public keySet()Ljava/util/Set;
public registryKeySet()Ljava/util/Set;
public entrySet()Ljava/util/Set;
public listElements()Ljava/util/stream/Stream;
public getTags()Ljava/util/stream/Stream;
private getOrCreateTagForRegistration(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;
private createTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;
public isEmpty()Z
public getRandom(Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
public containsKey(Lnet/minecraft/resources/Identifier;)Z
public containsKey(Lnet/minecraft/resources/ResourceKey;)Z
public componentLookup()Lnet/minecraft/core/component/DataComponentLookup;
public freeze()Lnet/minecraft/core/Registry;
public createIntrusiveHolder(Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
public get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
private validateAndUnwrapTagElement(Lnet/minecraft/tags/TagKey;Lnet/minecraft/core/Holder;)Lnet/minecraft/core/Holder$Reference;
public bindTags(Ljava/util/Map;)V
private refreshTagsInHolders()V
public bindAllTagsToEmpty()V
public createRegistrationLookup()Lnet/minecraft/core/HolderGetter;
public prepareTagReload(Lnet/minecraft/tags/TagLoader$LoadResult;)Lnet/minecraft/core/Registry$PendingTags;
private synthetic lambda$prepareTagReload$0(Lcom/google/common/collect/ImmutableMap$Builder;Ljava/util/Map;Lnet/minecraft/tags/TagKey;Ljava/util/List;)V
private static synthetic lambda$bindAllTagsToEmpty$0(Lnet/minecraft/core/HolderSet$Named;)V
private synthetic lambda$refreshTagsInHolders$1(Ljava/util/Map;Lnet/minecraft/tags/TagKey;Lnet/minecraft/core/HolderSet$Named;)V
private static synthetic lambda$refreshTagsInHolders$0(Ljava/util/Map;Lnet/minecraft/core/Holder$Reference;)V
private synthetic lambda$bindTags$0(Lnet/minecraft/tags/TagKey;Ljava/util/List;)V
private synthetic lambda$createIntrusiveHolder$0(Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
private static synthetic lambda$freeze$4(Ljava/util/Map$Entry;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$freeze$3(Ljava/util/Map$Entry;)Z
private static synthetic lambda$freeze$2(Ljava/util/Map$Entry;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$freeze$1(Ljava/util/Map$Entry;)Z
private static synthetic lambda$freeze$0(Ljava/lang/Object;Lnet/minecraft/core/Holder$Reference;)V
private synthetic lambda$getOrCreateHolderOrThrow$0(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Reference;
private synthetic lambda$register$0(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Reference;
private static synthetic lambda$new$0(Lit/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap;)V
```
