---
type: "interface"
fqcn: "net.minecraft.client.renderer.extract.LevelExtractor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.extract.LevelExtractor

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/ResourceManagerReloadListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `allChanged` | `()V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extract` | `(Lnet/minecraft/client/DeltaTracker;Lnet/minecraft/client/Camera;F)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractBlockOutline` | `(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/state/lev` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `levelRenderState` | `Lnet/minecraft/client/renderer/state/level/LevelRenderState;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `extractBlockOutline` | `(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/state/lev` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `extractPlayerState` | `(Lnet/minecraft/client/Camera;Lnet/minecraft/client/DeltaTracker;FLnet` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `getViewBlockingState` | `(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/client/render` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (17 fields, 34 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final CHUNK_VISIBILITY_THRESHOLD : F
private final minecraft : Lnet/minecraft/client/Minecraft;
private final levelRenderer : Lnet/minecraft/client/renderer/LevelRenderer;
private level : Lnet/minecraft/client/multiplayer/ClientLevel;
private sectionUpdateTracker : Lnet/minecraft/client/SectionUpdateTracker;
private final levelRenderState : Lnet/minecraft/client/renderer/state/level/LevelRenderState;
public final debugRenderer : Lnet/minecraft/client/renderer/debug/DebugRenderer;
public final gameTestBlockHighlightRenderer : Lnet/minecraft/client/renderer/debug/GameTestBlockHighlightRenderer;
private final transientBlockQueue : Ljava/util/Deque;
private final mainThreadGizmos : Lnet/minecraft/gizmos/SimpleGizmoCollector;
private prevCamRotX : D
private prevCamRotY : D
private lastViewDistance : I
private shouldInvalidateCompiledGeometry : Z
private shouldResetLevelRenderData : Z
private shouldResetChunkLayerSampler : Z
private shouldResetSkyRenderer : Z
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/renderer/state/level/LevelRenderState;Lnet/minecraft/client/renderer/LevelRenderer;)V
public extract(Lnet/minecraft/client/DeltaTracker;Lnet/minecraft/client/Camera;F)V
private drainTransientBlockQueue()V
private extractVisibleEntities(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/culling/Frustum;Lnet/minecraft/client/DeltaTracker;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V
public isEntityVisible(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/client/renderer/culling/Frustum;DDDFJ)Z
private extractEntity(Lnet/minecraft/world/entity/Entity;F)Lnet/minecraft/client/renderer/entity/state/EntityRenderState;
private extractVisibleBlockEntities(Lnet/minecraft/client/Camera;FLnet/minecraft/client/renderer/state/level/LevelRenderState;)V
private extractBlockDestroyAnimation(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V
private extractBlockOutline(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V
private extractGizmos()V
private extractPlayerState(Lnet/minecraft/client/Camera;Lnet/minecraft/client/DeltaTracker;FLnet/minecraft/client/renderer/state/level/PlayerRenderState;)V
private static getViewBlockingState(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/client/renderer/culling/Frustum;)Lnet/minecraft/world/level/block/state/BlockState;
private applyFrustum(Lnet/minecraft/client/renderer/culling/Frustum;)V
private static shouldShowEntityOutlines(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/state/level/PlayerRenderState;)Z
public onResourceManagerReload(Lnet/minecraft/server/packs/resources/ResourceManager;)V
public setLevel(Lnet/minecraft/client/multiplayer/ClientLevel;)V
public allChanged()V
public resetSampler()V
public blockChanged(Lnet/minecraft/core/BlockPos;I)V
private setBlockDirty(Lnet/minecraft/core/BlockPos;Z)V
public setBlocksDirty(IIIIII)V
public setBlockDirty(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V
public setSectionDirtyWithNeighbors(III)V
public setSectionRangeDirty(IIIIII)V
public setSectionDirty(III)V
private setSectionDirty(IIIZ)V
public queueTransientBlock(Lnet/minecraft/client/renderer/extract/TransientBlock;)V
public collectPerFrameMainThreadGizmos()Lnet/minecraft/gizmos/Gizmos$TemporaryCollection;
public countRenderedSections()I
public sectionStatistics()Ljava/lang/String;
public entityStatistics()Ljava/lang/String;
public totalSections()D
public lastViewDistance()D
private synthetic lambda$extract$0(Lnet/minecraft/client/multiplayer/ClientChunkCache;J)V
```
