---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.properties.IntegerProperty"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.properties.IntegerProperty

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `net/minecraft/world/level/block/state/properties/Property`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPossibleValues` | `()Ljava/util/List;` | exact | invokevirtual@68 in `CauldronFluidContent.registerCauldron` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final values : Lit/unimi/dsi/fastutil/ints/IntImmutableList;
private final min : I
private final max : I
private <init>(Ljava/lang/String;II)V
public getPossibleValues()Ljava/util/List;
public equals(Ljava/lang/Object;)Z
public generateHashCode()I
public static create(Ljava/lang/String;II)Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
public getValue(Ljava/lang/String;)Ljava/util/Optional;
public getName(Ljava/lang/Integer;)Ljava/lang/String;
public getInternalIndex(Ljava/lang/Integer;)I
public synthetic getInternalIndex(Ljava/lang/Comparable;)I
public synthetic getName(Ljava/lang/Comparable;)Ljava/lang/String;
```
