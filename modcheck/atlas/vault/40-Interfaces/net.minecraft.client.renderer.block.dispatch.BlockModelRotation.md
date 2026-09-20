---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockModelRotation"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockModelRotation

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/dispatch/ModelState`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `IDENTITY` | `Lnet/minecraft/client/renderer/block/dispatch/BlockModelRotation;` | exact | getstatic@1 in `SimpleUnbakedExtraModel.blockStateModel` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `IDENTITY` | `Lnet/minecraft/client/renderer/block/dispatch/BlockModelRotation;` | exact | getstatic@12 in `ModelStateHelper.of` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (7 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final BY_GROUP_ORDINAL : Ljava/util/Map;
public static final IDENTITY : Lnet/minecraft/client/renderer/block/dispatch/BlockModelRotation;
private final orientation : Lcom/mojang/math/OctahedralGroup;
private final transformation : Lcom/mojang/math/Transformation;
private final faceMapping : Ljava/util/Map;
private final inverseFaceMapping : Ljava/util/Map;
private final withUvLock : Lnet/minecraft/client/renderer/block/dispatch/BlockModelRotation$WithUvLock;
private <init>(Lcom/mojang/math/OctahedralGroup;)V
public transformation()Lcom/mojang/math/Transformation;
public static get(Lcom/mojang/math/OctahedralGroup;)Lnet/minecraft/client/renderer/block/dispatch/BlockModelRotation;
public withUvLock()Lnet/minecraft/client/renderer/block/dispatch/ModelState;
public toString()Ljava/lang/String;
static <clinit>()V
```
