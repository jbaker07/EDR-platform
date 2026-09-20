---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.SingleVariant"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.SingleVariant

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/dispatch/BlockStateModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;)V` | exact | invokespecial@10 in `SimpleUnbakedExtraModel.lambda$blockStateModel$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `model` | `Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final model : Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;
public <init>(Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;)V
public collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V
public particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public materialFlags()I
```
