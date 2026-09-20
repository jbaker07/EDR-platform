---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Overwrites

`@Overwrite` replaces a vanilla method body. What that does to other mods' injections into the same method was executed, not assumed: see [[30-Mechanisms/Transformation_Tests]] (HEAD/TAIL/RETURN injections survive; INVOKE-point ones are refused unless they outrank the overwrite).

| vanilla method | module | mixin | env |
|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.block.model.BlockStateModelWrapper|BlockStateModelWrapper]].`update(Lnet/minecraft/client/renderer/block/BlockModelRenderState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/model/BlockDisplayContext;J)V` | fabric-renderer-api-v1 | `BlockStateModelWrapperMixin` | client |
