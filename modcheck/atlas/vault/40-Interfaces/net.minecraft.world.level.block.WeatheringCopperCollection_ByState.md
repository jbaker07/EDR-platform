---
type: "interface"
fqcn: "net.minecraft.world.level.block.WeatheringCopperCollection$ByState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.WeatheringCopperCollection$ByState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `progressMapping` | `(Ljava/util/function/BiConsumer;)V` | exact | invokevirtual@16 in `OxidizableBlocksRegistryImpl.registerWeatheringCopperBlocks` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (4 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final unaffected : Ljava/lang/Object;
private final exposed : Ljava/lang/Object;
private final weathered : Ljava/lang/Object;
private final oxidized : Ljava/lang/Object;
public <init>(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V
public static create(Ljava/lang/Object;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public map(Ljava/util/function/Function;)Lnet/minecraft/world/level/block/WeatheringCopperCollection$ByState;
public pick(Lnet/minecraft/world/level/block/WeatheringCopper$WeatherState;)Ljava/lang/Object;
public forEach(Ljava/util/function/Consumer;)V
public progressMapping(Ljava/util/function/BiConsumer;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public unaffected()Ljava/lang/Object;
public exposed()Ljava/lang/Object;
public weathered()Ljava/lang/Object;
public oxidized()Ljava/lang/Object;
```
