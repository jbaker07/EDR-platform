---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.StateHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.StateHolder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/Object;[Lnet/minecraft/world/level/block/state/properties/` | exact | invokespecial@4 in `BlockBehaviourBlockStateBaseMixin.<init>` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (8 fields, 26 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final VALUE_NOT_FOUND : I
public static final ID_TAG : Ljava/lang/String;
public static final PROPERTIES_TAG : Ljava/lang/String;
protected final owner : Ljava/lang/Object;
private final propertyKeys : [Lnet/minecraft/world/level/block/state/properties/Property;
private final propertyValues : [Ljava/lang/Comparable;
private neighbors : [[Ljava/lang/Object;
static final synthetic $assertionsDisabled : Z
protected <init>(Ljava/lang/Object;[Lnet/minecraft/world/level/block/state/properties/Property;[Ljava/lang/Comparable;)V
public cycle(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lang/Object;
protected static findNextInCollection(Ljava/util/List;Ljava/lang/Object;)Ljava/lang/Object;
public toString()Ljava/lang/String;
public final equals(Ljava/lang/Object;)Z
public hashCode()I
public getProperties()Ljava/util/Collection;
private valueIndex(Lnet/minecraft/world/level/block/state/properties/Property;)I
public hasProperty(Lnet/minecraft/world/level/block/state/properties/Property;)Z
private getNullableValue(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lang/Comparable;
public getValue(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lang/Comparable;
public getOptionalValue(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/util/Optional;
public getValueOrElse(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;)Ljava/lang/Comparable;
public setValue(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;)Ljava/lang/Object;
public trySetValue(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;)Ljava/lang/Object;
private setValueInternal(Lnet/minecraft/world/level/block/state/properties/Property;ILjava/lang/Comparable;)Ljava/lang/Object;
 initializeNeighbors([[Ljava/lang/Object;)V
public isSingletonState()Z
public getValues()Ljava/util/stream/Stream;
private static createValue(Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Comparable;)Lnet/minecraft/world/level/block/state/properties/Property$Value;
protected static codec(Lcom/mojang/serialization/Codec;Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
private static synthetic lambda$codec$1(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/lang/Object;)Lcom/mojang/serialization/MapCodec;
private static synthetic lambda$codec$2(Lnet/minecraft/world/level/block/state/StateHolder;Ljava/util/Optional;)Lnet/minecraft/world/level/block/state/StateHolder;
private static synthetic lambda$codec$0(Lnet/minecraft/world/level/block/state/StateHolder;)Ljava/lang/Object;
private synthetic lambda$getValues$0(I)Lnet/minecraft/world/level/block/state/properties/Property$Value;
static <clinit>()V
```
