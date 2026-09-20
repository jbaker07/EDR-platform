---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockAndTintGetter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockAndTintGetter

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `cardinalLighting()Lnet/minecraft/world/level/CardinalLighting;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cardinalLighting()Lnet/minecraft/world/level/CardinalLighting;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/bl` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/bl` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EMPTYLnet/minecraft/client/renderer/block/BlockAndTintGetter;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTYLnet/minecraft/client/renderer/block/BlockAndTintGetter;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.block.BlockAndTintGetter extends net.minecraft.world.level.BlockAndLightGetter {
    public static final net.minecraft.client.renderer.block.BlockAndTintGetter EMPTY;
    public abstract net.minecraft.world.level.CardinalLighting cardinalLighting();
    public abstract int getBlockTint(net.minecraft.core.BlockPos, net.minecraft.world.level.ColorResolver);
    static {};
}
```
