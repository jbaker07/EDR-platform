---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.BlockBreakingRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.BlockBreakingRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `blockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@33 in `LevelRendererMixin.submitBreakingBlockModelProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `blockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@38 in `LevelRendererMixin.submitBreakingBlockModelProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final blockPos : Lnet/minecraft/core/BlockPos;
private final blockState : Lnet/minecraft/world/level/block/state/BlockState;
private final progress : I
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public blockPos()Lnet/minecraft/core/BlockPos;
public blockState()Lnet/minecraft/world/level/block/state/BlockState;
public progress()I
```
