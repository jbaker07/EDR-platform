---
type: "interface"
fqcn: "com.mojang.renderpearl.api.pipeline.PrimitiveTopology"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.pipeline.PrimitiveTopology

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `connectedPrimitives` | `Z` | exact | getfield@8 in `GuiRendererMixin.uploadPrimitivesIndividually` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (12 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LINES : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final DEBUG_LINES : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final DEBUG_LINE_STRIP : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final POINTS : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final TRIANGLES : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final TRIANGLE_STRIP : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final TRIANGLE_FAN : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static final QUADS : Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public final primitiveLength : I
public final primitiveStride : I
public final connectedPrimitives : Z
private static final synthetic $VALUES : [Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static values()[Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
public static valueOf(Ljava/lang/String;)Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
private <init>(Ljava/lang/String;IIIZ)V
public indexCount(I)I
private static synthetic $values()[Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;
static <clinit>()V
```
