---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.LevelRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.LevelRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `reset` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `blockOutlineRenderState` | `Lnet/minecraft/client/renderer/state/level/BlockOutlineRenderState;` | exact | getfield@20 in `LevelRendererMixin.beforeRenderBlockOutline` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `cameraRenderState` | `Lnet/minecraft/client/renderer/state/level/CameraRenderState;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (24 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public cameraRenderState : Lnet/minecraft/client/renderer/state/level/CameraRenderState;
public final playerRenderState : Lnet/minecraft/client/renderer/state/level/PlayerRenderState;
public final sectionUpdateRenderStates : Ljava/util/List;
public final entityRenderStates : Ljava/util/List;
public final blockEntityRenderStates : Ljava/util/List;
public blockOutlineRenderState : Lnet/minecraft/client/renderer/state/level/BlockOutlineRenderState;
public final blockBreakingRenderStates : Ljava/util/List;
public final weatherRenderState : Lnet/minecraft/client/renderer/state/level/WeatherRenderState;
public final worldBorderRenderState : Lnet/minecraft/client/renderer/state/level/WorldBorderRenderState;
public final skyRenderState : Lnet/minecraft/client/renderer/state/level/SkyRenderState;
public final particlesRenderState : Lnet/minecraft/client/renderer/state/level/ParticlesRenderState;
public gameTime : J
public worldPartialTicks : F
public lastEntityRenderStateCount : I
public cloudColor : I
public cloudHeight : F
public render3dCrosshair : Z
public renderWireframeTerrain : Z
public shouldUseMultiDrawIndirectForTerrain : Z
public playerCompiledSectionCallback : Ljava/lang/Runnable;
public chunkLoadingRenderState : Lnet/minecraft/client/renderer/state/level/ChunkLoadingRenderState;
public shouldResetChunkLayerSampler : Z
public shouldShowEntityOutlines : Z
public shouldResetSkyRenderer : Z
public <init>()V
public reset()V
```
