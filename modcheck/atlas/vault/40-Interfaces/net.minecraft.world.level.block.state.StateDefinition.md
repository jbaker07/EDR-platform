---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.StateDefinition"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.StateDefinition

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@4 in `OxidizableBlocksRegistryImpl.refreshRandomTickCache` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@23 in `ModelLoadingEventDispatcher.resolveBlockStates` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@37 in `PoiHelper.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@4 in `BlockInitTracker.lambda$postFreeze$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@4 in `BlocksMixin.lambda$initShapeCache$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@4 in `BootstrapMixin.lambda$afterInitialize$1` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getPossibleStates` | `()Lcom/google/common/collect/ImmutableList;` | exact | invokevirtual@4 in `BootstrapMixin.lambda$afterInitialize$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (9 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final NAME_PATTERN : Ljava/util/regex/Pattern;
private static final EMPTY_VALUES : [Ljava/lang/Comparable;
private static final EMPTY_KEYS : [Lnet/minecraft/world/level/block/state/properties/Property;
private static final EMPTY_NEIGHBORS : [[Lnet/minecraft/world/level/block/state/StateHolder;
private final owner : Ljava/lang/Object;
private final propertiesByName : Lcom/google/common/collect/ImmutableSortedMap;
private final states : Lcom/google/common/collect/ImmutableList;
private final propertiesCodec : Lcom/mojang/serialization/MapCodec;
static final synthetic $assertionsDisabled : Z
protected <init>(Ljava/util/function/Function;Ljava/lang/Object;Lnet/minecraft/world/level/block/state/StateDefinition$Factory;Ljava/util/Map;)V
private static createCodec(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/Map;)Lcom/mojang/serialization/MapCodec;
private static createSingletonState(Ljava/lang/Object;Lnet/minecraft/world/level/block/state/StateDefinition$Factory;)Lcom/google/common/collect/ImmutableList;
private static createSinglePropertyStates(Ljava/lang/Object;Lnet/minecraft/world/level/block/state/StateDefinition$Factory;Ljava/util/Map;)Lcom/google/common/collect/ImmutableList;
private static createSinglePropertyStates(Ljava/lang/Object;Lnet/minecraft/world/level/block/state/StateDefinition$Factory;Lnet/minecraft/world/level/block/state/properties/Property;)Lcom/google/common/collect/ImmutableList;
private static createMultiPropertyStates(Ljava/lang/Object;Lnet/minecraft/world/level/block/state/StateDefinition$Factory;Ljava/util/Map;)Lcom/google/common/collect/ImmutableList;
private static emptyNeighbors()[[Lnet/minecraft/world/level/block/state/StateHolder;
private static appendPropertyCodec(Lcom/mojang/serialization/MapCodec;Ljava/util/function/Supplier;Ljava/lang/String;Lnet/minecraft/world/level/block/state/properties/Property;)Lcom/mojang/serialization/MapCodec;
public getPossibleStates()Lcom/google/common/collect/ImmutableList;
public any()Lnet/minecraft/world/level/block/state/StateHolder;
public propertiesCodec()Lcom/mojang/serialization/MapCodec;
public getOwner()Ljava/lang/Object;
public getProperties()Ljava/util/Collection;
public toString()Ljava/lang/String;
public getProperty(Ljava/lang/String;)Lnet/minecraft/world/level/block/state/properties/Property;
public isSingletonState()Z
private static synthetic lambda$appendPropertyCodec$3(Lnet/minecraft/world/level/block/state/properties/Property;Lnet/minecraft/world/level/block/state/StateHolder;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$appendPropertyCodec$2(Lnet/minecraft/world/level/block/state/properties/Property;Lcom/mojang/datafixers/util/Pair;)Lnet/minecraft/world/level/block/state/StateHolder;
private static synthetic lambda$appendPropertyCodec$1(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/function/Supplier;)Lnet/minecraft/world/level/block/state/properties/Property$Value;
private static synthetic lambda$appendPropertyCodec$0(Ljava/lang/String;)V
private static synthetic lambda$createMultiPropertyStates$0(Lnet/minecraft/world/level/block/state/StateDefinition$StateCollection;[Lnet/minecraft/world/level/block/state/properties/Property;Ljava/util/List;Lnet/minecraft/world/level/block/state/StateHolder;)V
private static synthetic lambda$createCodec$0(Ljava/util/function/Function;Ljava/lang/Object;)Lnet/minecraft/world/level/block/state/StateHolder;
static <clinit>()V
```
