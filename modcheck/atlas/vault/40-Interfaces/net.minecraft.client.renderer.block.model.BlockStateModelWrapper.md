---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.model.BlockStateModelWrapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.model.BlockStateModelWrapper

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/model/BlockModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `updateTints` | `(Lnet/minecraft/client/renderer/block/BlockModelRenderState;Lnet/minec` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `model` | `Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `transformation` | `Lorg/joml/Matrix4fc;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| replaces | `update` | `(Lnet/minecraft/client/renderer/block/BlockModelRenderState;Lnet/minec` | exact | @Overwrite | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (3 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final model : Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
private final tints : Ljava/util/List;
private final transformation : Lorg/joml/Matrix4fc;
public <init>(Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;Ljava/util/List;Lorg/joml/Matrix4fc;)V
public update(Lnet/minecraft/client/renderer/block/BlockModelRenderState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/model/BlockDisplayContext;J)V
private updateTints(Lnet/minecraft/client/renderer/block/BlockModelRenderState;Lnet/minecraft/world/level/block/state/BlockState;)V
```
