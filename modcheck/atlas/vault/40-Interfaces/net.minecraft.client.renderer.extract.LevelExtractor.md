---
type: "interface"
fqcn: "net.minecraft.client.renderer.extract.LevelExtractor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.extract.LevelExtractor

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `allChanged` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extract` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractBlockOutline` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractBlockOutline` | `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/dispatch/BlockStateMode` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (51, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.extract.LevelExtractor implements net.minecraft.server.packs.resources.ResourceManagerReloadListener {
    private static final float CHUNK_VISIBILITY_THRESHOLD;
    private final net.minecraft.client.Minecraft minecraft;
    private final net.minecraft.client.renderer.LevelRenderer levelRenderer;
    private net.minecraft.client.multiplayer.ClientLevel level;
    private net.minecraft.client.SectionUpdateTracker sectionUpdateTracker;
    private final net.minecraft.client.renderer.state.level.LevelRenderState levelRenderState;
    public final net.minecraft.client.renderer.debug.DebugRenderer debugRenderer;
    public final net.minecraft.client.renderer.debug.GameTestBlockHighlightRenderer gameTestBlockHighlightRenderer;
    private final java.util.Deque<net.minecraft.client.renderer.extract.TransientBlock> transientBlockQueue;
    private final net.minecraft.gizmos.SimpleGizmoCollector mainThreadGizmos;
    private double prevCamRotX;
    private double prevCamRotY;
    private int lastViewDistance;
    private boolean shouldInvalidateCompiledGeometry;
    private boolean shouldResetLevelRenderData;
    private boolean shouldResetChunkLayerSampler;
    private boolean shouldResetSkyRenderer;
    public net.minecraft.client.renderer.extract.LevelExtractor(net.minecraft.client.Minecraft, net.minecraft.client.renderer.state.level.LevelRenderState, net.minecraft.client.renderer.LevelRenderer);
    public void extract(net.minecraft.client.DeltaTracker, net.minecraft.client.Camera, float);
    private void drainTransientBlockQueue();
    private void extractVisibleEntities(net.minecraft.client.Camera, net.minecraft.client.renderer.culling.Frustum, net.minecraft.client.DeltaTracker, net.minecraft.client.renderer.state.level.LevelRenderState);
    public boolean isEntityVisible(net.minecraft.world.entity.Entity, net.minecraft.client.renderer.culling.Frustum, double, double, double, float, long);
    private net.minecraft.client.renderer.entity.state.EntityRenderState extractEntity(net.minecraft.world.entity.Entity, float);
    private void extractVisibleBlockEntities(net.minecraft.client.Camera, float, net.minecraft.client.renderer.state.level.LevelRenderState);
    private void extractBlockDestroyAnimation(net.minecraft.client.Camera, net.minecraft.client.renderer.state.level.LevelRenderState);
    private void extractBlockOutline(net.minecraft.client.Camera, net.minecraft.client.renderer.state.level.LevelRenderState);
    private void extractGizmos();
    private void extractPlayerState(net.minecraft.client.Camera, net.minecraft.client.DeltaTracker, float, net.minecraft.client.renderer.state.level.PlayerRenderState);
    private static net.minecraft.world.level.block.state.BlockState getViewBlockingState(net.minecraft.client.player.LocalPlayer, net.minecraft.client.renderer.culling.Frustum);
    private void applyFrustum(net.minecraft.client.renderer.culling.Frustum);
    private static boolean shouldShowEntityOutlines(net.minecraft.client.Camera, net.minecraft.client.renderer.state.level.PlayerRenderState);
    public void onResourceManagerReload(net.minecraft.server.packs.resources.ResourceManager);
    public void setLevel(net.minecraft.client.multiplayer.ClientLevel);
    public void allChanged();
    public void resetSampler();
    public void blockChanged(net.minecraft.core.BlockPos, int);
    private void setBlockDirty(net.minecraft.core.BlockPos, boolean);
    public void setBlocksDirty(int, int, int, int, int, int);
    public void setBlockDirty(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState);
    public void setSectionDirtyWithNeighbors(int, int, int);
    public void setSectionRangeDirty(int, int, int, int, int, int);
    public void setSectionDirty(int, int, int);
    private void setSectionDirty(int, int, int, boolean);
    public void queueTransientBlock(net.minecraft.client.renderer.extract.TransientBlock);
    public net.minecraft.gizmos.Gizmos$TemporaryCollection collectPerFrameMainThreadGizmos();
    public int countRenderedSections();
    public java.lang.String sectionStatistics();
    public java.lang.String entityStatistics();
    public double totalSections();
    public double lastViewDistance();
    private void lambda$extract$0(net.minecraft.client.multiplayer.ClientChunkCache, long);
}
```
