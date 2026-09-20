---
type: "interface"
fqcn: "net.minecraft.core.Direction$AxisDirection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Direction$AxisDirection

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `POSITIVE` | `Lnet/minecraft/core/Direction$AxisDirection;` | exact | getstatic@19 in `GeometryHelper.isParallelQuadOnFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (5 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final POSITIVE : Lnet/minecraft/core/Direction$AxisDirection;
public static final NEGATIVE : Lnet/minecraft/core/Direction$AxisDirection;
private final step : I
private final name : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/core/Direction$AxisDirection;
public static values()[Lnet/minecraft/core/Direction$AxisDirection;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/core/Direction$AxisDirection;
private <init>(Ljava/lang/String;IILjava/lang/String;)V
public getStep()I
public getName()Ljava/lang/String;
public toString()Ljava/lang/String;
public opposite()Lnet/minecraft/core/Direction$AxisDirection;
private static synthetic $values()[Lnet/minecraft/core/Direction$AxisDirection;
static <clinit>()V
```
