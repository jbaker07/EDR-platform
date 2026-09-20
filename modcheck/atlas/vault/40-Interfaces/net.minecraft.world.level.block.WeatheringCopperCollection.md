---
type: "interface"
fqcn: "net.minecraft.world.level.block.WeatheringCopperCollection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.WeatheringCopperCollection

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `weathering` | `()Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;` | exact | invokevirtual@8 in `OxidizableBlocksRegistryImpl.registerWeatheringCopperBlocks` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `zipUnwaxedWaxed` | `(Ljava/util/function/BiConsumer;)V` | exact | invokevirtual@25 in `OxidizableBlocksRegistryImpl.registerWeatheringCopperBlocks` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (4 fields, 32 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final weathering : Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
private final waxed : Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public static final STATES : Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public static final PREFIXES : Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public <init>(Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;)V
public static prefixWithState(Lnet/minecraft/world/level/block/WeatheringCopperCollection;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public static create(Ljava/lang/String;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public static same(Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public static registerBlocks(Lnet/minecraft/world/level/block/WeatheringCopperCollection;Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public static registerItems(Lnet/minecraft/world/level/block/WeatheringCopperCollection;Lnet/minecraft/world/level/block/WeatheringCopperCollection;Ljava/util/function/BiFunction;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public static createFamily(Ljava/util/function/BiFunction;Ljava/util/function/BiFunction;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public asList()Ljava/util/List;
public forEach(Ljava/util/function/Consumer;)V
public map(Ljava/util/function/Function;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public apply(Ljava/util/function/Function;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public apply(Ljava/util/function/Function;Ljava/util/function/Function;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public static zipApply(Lnet/minecraft/world/level/block/WeatheringCopperCollection;Lnet/minecraft/world/level/block/WeatheringCopperCollection;Ljava/util/function/BiConsumer;)V
public static zipMap(Lnet/minecraft/world/level/block/WeatheringCopperCollection;Lnet/minecraft/world/level/block/WeatheringCopperCollection;Ljava/util/function/BiFunction;)Lnet/minecraft/world/level/block/WeatheringCopperCollection;
public zipUnwaxedWaxed(Ljava/util/function/BiConsumer;)V
public static zipApply(Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;Ljava/util/function/BiConsumer;)V
public static zipMap(Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;Ljava/util/function/BiFunction;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public weathering()Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public waxed()Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
private static synthetic lambda$createFamily$1(Ljava/util/function/BiFunction;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
private static synthetic lambda$createFamily$0(Ljava/util/function/BiFunction;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
private static synthetic lambda$registerBlocks$3(Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
private static synthetic lambda$registerBlocks$4(Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Ljava/lang/Object;)Lnet/minecraft/world/level/block/Block;
private static synthetic lambda$registerBlocks$5(Ljava/util/function/BiFunction;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;
private static synthetic lambda$registerBlocks$0(Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
private static synthetic lambda$registerBlocks$1(Lorg/apache/commons/lang3/function/TriFunction;Ljava/util/function/BiFunction;Ljava/util/function/Function;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Ljava/lang/Object;)Lnet/minecraft/world/level/block/Block;
private static synthetic lambda$registerBlocks$2(Ljava/util/function/BiFunction;Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;
private static synthetic lambda$prefixWithState$0(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
static <clinit>()V
```
