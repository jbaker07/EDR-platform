---
type: "interface"
fqcn: "net.minecraft.client.particle.TerrainParticle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.TerrainParticle

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/mi` | `` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/mi` | `` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| wraps | `<init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/BlockStateModelSet;getP` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `createTerrainParticle` | `@Redirect at NEW (Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minec` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.particle.TerrainParticle extends net.minecraft.client.particle.SingleQuadParticle {
    private final net.minecraft.client.particle.SingleQuadParticle$Layer layer;
    private final net.minecraft.core.BlockPos pos;
    private final float uo;
    private final float vo;
    public net.minecraft.client.particle.TerrainParticle(net.minecraft.client.multiplayer.ClientLevel, double, double, double, double, double, double, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.client.particle.TerrainParticle(net.minecraft.client.multiplayer.ClientLevel, double, double, double, double, double, double, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
    public net.minecraft.client.particle.SingleQuadParticle$Layer getLayer();
    protected float getU0();
    protected float getU1();
    protected float getV0();
    protected float getV1();
    private static net.minecraft.client.particle.TerrainParticle createTerrainParticle(net.minecraft.core.particles.BlockParticleOption, net.minecraft.client.multiplayer.ClientLevel, double, double, double, double, double, double);
}
```
