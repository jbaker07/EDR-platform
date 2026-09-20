---
type: "interface"
fqcn: "net.minecraft.world.phys.BlockHitResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.phys.BlockHitResult

System: [[20-Systems/net.minecraft.world.phys|net.minecraft.world.phys]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getBlockPos()Lnet/minecraft/core/BlockPos;` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlockPos()Lnet/minecraft/core/BlockPos;` | `` | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.phys.BlockHitResult extends net.minecraft.world.phys.HitResult {
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.world.phys.BlockHitResult> STREAM_CODEC;
    private final net.minecraft.core.Direction direction;
    private final net.minecraft.core.BlockPos blockPos;
    private final boolean miss;
    private final boolean inside;
    private final boolean worldBorderHit;
    public static net.minecraft.world.phys.BlockHitResult miss(net.minecraft.world.phys.Vec3, net.minecraft.core.Direction, net.minecraft.core.BlockPos);
    public net.minecraft.world.phys.BlockHitResult(net.minecraft.world.phys.Vec3, net.minecraft.core.Direction, net.minecraft.core.BlockPos, boolean);
    public net.minecraft.world.phys.BlockHitResult(net.minecraft.world.phys.Vec3, net.minecraft.core.Direction, net.minecraft.core.BlockPos, boolean, boolean);
    private net.minecraft.world.phys.BlockHitResult(boolean, net.minecraft.world.phys.Vec3, net.minecraft.core.Direction, net.minecraft.core.BlockPos, boolean, boolean);
    public net.minecraft.world.phys.BlockHitResult withDirection(net.minecraft.core.Direction);
    public net.minecraft.world.phys.BlockHitResult withPosition(net.minecraft.core.BlockPos);
    public net.minecraft.world.phys.BlockHitResult hitBorder();
    public net.minecraft.core.BlockPos getBlockPos();
    public net.minecraft.core.Direction getDirection();
    public net.minecraft.world.phys.HitResult$Type getType();
    public boolean isInside();
    public boolean isWorldBorderHit();
    static {};
}
```
