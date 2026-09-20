---
type: "interface"
fqcn: "net.minecraft.client.color.block.BlockColors"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.color.block.BlockColors

System: [[20-Systems/net.minecraft.client.color|net.minecraft.client.color]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getTintSources(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/ut` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getTintSources(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/ut` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `register(Ljava/util/List;[Lnet/minecraft/world/level/block/Block;)V` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `createDefault` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.color.block.BlockColors {
    public static final int LILY_PAD_IN_WORLD;
    public static final int LILY_PAD_DEFAULT;
    private static final net.minecraft.client.color.block.BlockTintSource BLANK_LAYER;
    private final java.util.Map<net.minecraft.world.level.block.Block, java.util.List<net.minecraft.client.color.block.BlockTintSource>> sources;
    public net.minecraft.client.color.block.BlockColors();
    public static net.minecraft.client.color.block.BlockColors createDefault();
    public java.util.List<net.minecraft.client.color.block.BlockTintSource> getTintSources(net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.client.color.block.BlockTintSource getTintSource(net.minecraft.world.level.block.state.BlockState, int);
    public void register(java.util.List<net.minecraft.client.color.block.BlockTintSource>, net.minecraft.world.level.block.Block...);
    public java.util.Set<net.minecraft.world.level.block.state.properties.Property<?>> getColoringProperties(net.minecraft.world.level.block.Block);
    static {};
}
```
