---
type: "interface"
fqcn: "net.minecraft.client.renderer.chunk.RenderSectionRegion"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.chunk.RenderSectionRegion

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/BlockAndTintGetter`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | declared |

## Declared members (9 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RADIUS : I
public static final SIZE : I
private final minSectionX : I
private final minSectionY : I
private final minSectionZ : I
private final sections : [Lnet/minecraft/client/renderer/chunk/SectionCopy;
private final level : Lnet/minecraft/client/multiplayer/ClientLevel;
private final cardinalLighting : Lnet/minecraft/world/level/CardinalLighting;
private final lightEngine : Lnet/minecraft/world/level/lighting/LevelLightEngine;
public <init>(Lnet/minecraft/client/multiplayer/ClientLevel;III[Lnet/minecraft/client/renderer/chunk/SectionCopy;)V
public getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/FluidState;
public cardinalLighting()Lnet/minecraft/world/level/CardinalLighting;
public getLightEngine()Lnet/minecraft/world/level/lighting/LevelLightEngine;
public getBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
private getSection(III)Lnet/minecraft/client/renderer/chunk/SectionCopy;
public getBlockTint(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver;)I
public getMinY()I
public getHeight()I
public static index(IIIIII)I
```
