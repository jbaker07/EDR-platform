---
type: "interface"
fqcn: "net.minecraft.client.particle.BlockMarker"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.particle.BlockMarker

System: [[20-Systems/net.minecraft.client.particle|net.minecraft.client.particle]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `<init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/world/level/block/state/BlockState;)V` | `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/BlockStateModelSet;getP` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.particle.BlockMarker extends net.minecraft.client.particle.SingleQuadParticle {
    private final net.minecraft.client.particle.SingleQuadParticle$Layer layer;
    private net.minecraft.client.particle.BlockMarker(net.minecraft.client.multiplayer.ClientLevel, double, double, double, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.client.particle.SingleQuadParticle$Layer getLayer();
    public float getQuadSize(float);
}
```
