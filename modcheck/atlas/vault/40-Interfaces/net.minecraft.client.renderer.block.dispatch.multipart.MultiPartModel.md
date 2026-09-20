---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/dispatch/BlockStateModel`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `blockState` | `Lnet/minecraft/world/level/block/state/BlockState;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `models` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `shared` | `Lnet/minecraft/client/renderer/block/dispatch/multipart/MultiPartModel` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (3 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final shared : Lnet/minecraft/client/renderer/block/dispatch/multipart/MultiPartModel$SharedBakedState;
private final blockState : Lnet/minecraft/world/level/block/state/BlockState;
private models : Ljava/util/List;
private <init>(Lnet/minecraft/client/renderer/block/dispatch/multipart/MultiPartModel$SharedBakedState;Lnet/minecraft/world/level/block/state/BlockState;)V
public particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public materialFlags()I
public collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V
```
