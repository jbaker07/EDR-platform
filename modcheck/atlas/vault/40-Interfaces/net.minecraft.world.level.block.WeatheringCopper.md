---
type: "interface"
fqcn: "net.minecraft.world.level.block.WeatheringCopper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.WeatheringCopper

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/block/ChangeOverTimeBlock`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$static$0` | `()Lcom/google/common/collect/BiMap;` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `NEXT_BY_BLOCK` | `Ljava/util/function/Supplier;` | exact | getstatic@14 in `OxidizableBlocksRegistryImpl.registerNextStage` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (2 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NEXT_BY_BLOCK : Ljava/util/function/Supplier;
public static final PREVIOUS_BY_BLOCK : Ljava/util/function/Supplier;
public static getPrevious(Lnet/minecraft/world/level/block/Block;)Ljava/util/Optional;
public static getFirst(Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/block/Block;
public static getPrevious(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/Optional;
public static getNext(Lnet/minecraft/world/level/block/Block;)Ljava/util/Optional;
public static getFirst(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
public getNext(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/Optional;
public getChanceModifier()F
private static synthetic lambda$getNext$0(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$getPrevious$0(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Block;)Lnet/minecraft/world/level/block/state/BlockState;
private static synthetic lambda$static$2()Lcom/google/common/collect/BiMap;
private static synthetic lambda$static$0()Lcom/google/common/collect/BiMap;
private static synthetic lambda$static$1(Lcom/google/common/collect/ImmutableBiMap$Builder;Lnet/minecraft/world/level/block/WeatheringCopperCollection;)V
static <clinit>()V
```
