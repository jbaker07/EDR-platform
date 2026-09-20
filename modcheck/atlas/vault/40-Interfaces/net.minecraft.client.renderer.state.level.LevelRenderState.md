---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.LevelRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.LevelRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `reset` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `blockOutlineRenderStateLnet/minecraft/client/renderer/state/level/BlockOutlineRende` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.LevelRenderState {
    public net.minecraft.client.renderer.state.level.CameraRenderState cameraRenderState;
    public final net.minecraft.client.renderer.state.level.PlayerRenderState playerRenderState;
    public final java.util.List<net.minecraft.client.renderer.state.level.SectionUpdateRenderState> sectionUpdateRenderStates;
    public final java.util.List<net.minecraft.client.renderer.entity.state.EntityRenderState> entityRenderStates;
    public final java.util.List<net.minecraft.client.renderer.blockentity.state.BlockEntityRenderState> blockEntityRenderStates;
    public net.minecraft.client.renderer.state.level.BlockOutlineRenderState blockOutlineRenderState;
    public final java.util.List<net.minecraft.client.renderer.state.level.BlockBreakingRenderState> blockBreakingRenderStates;
    public final net.minecraft.client.renderer.state.level.WeatherRenderState weatherRenderState;
    public final net.minecraft.client.renderer.state.level.WorldBorderRenderState worldBorderRenderState;
    public final net.minecraft.client.renderer.state.level.SkyRenderState skyRenderState;
    public final net.minecraft.client.renderer.state.level.ParticlesRenderState particlesRenderState;
    public long gameTime;
    public float worldPartialTicks;
    public int lastEntityRenderStateCount;
    public int cloudColor;
    public float cloudHeight;
    public boolean render3dCrosshair;
    public boolean renderWireframeTerrain;
    public boolean shouldUseMultiDrawIndirectForTerrain;
    public java.lang.Runnable playerCompiledSectionCallback;
    public net.minecraft.client.renderer.state.level.ChunkLoadingRenderState chunkLoadingRenderState;
    public boolean shouldResetChunkLayerSampler;
    public boolean shouldShowEntityOutlines;
    public boolean shouldResetSkyRenderer;
    public net.minecraft.client.renderer.state.level.LevelRenderState();
    public void reset();
}
```
