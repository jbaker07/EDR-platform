---
type: "interface"
fqcn: "net.minecraft.world.level.ChunkPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.ChunkPos

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `pack(Lnet/minecraft/core/BlockPos;)J` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `x()I` | `` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `x()I` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `z()I` | `` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `z()I` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (64, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.ChunkPos extends java.lang.Record {
    private final int x;
    private final int z;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.ChunkPos> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.world.level.ChunkPos> STREAM_CODEC;
    private static final int SAFETY_MARGIN;
    public static final long INVALID_CHUNK_POS;
    public static final net.minecraft.world.level.ChunkPos ZERO;
    private static final long COORD_BITS;
    private static final long COORD_MASK;
    private static final int REGION_BITS;
    public static final int REGION_SIZE;
    private static final int REGION_MASK;
    public static final int REGION_MAX_INDEX;
    private static final int HASH_A;
    private static final int HASH_C;
    private static final int HASH_Z_XOR;
    public net.minecraft.world.level.ChunkPos(int, int);
    public static net.minecraft.world.level.ChunkPos containing(net.minecraft.core.BlockPos);
    public static net.minecraft.world.level.ChunkPos unpack(long);
    public static net.minecraft.world.level.ChunkPos minFromRegion(int, int);
    public static net.minecraft.world.level.ChunkPos maxFromRegion(int, int);
    public boolean isValid();
    public static boolean isValid(int, int);
    public long pack();
    public static long pack(int, int);
    public static long fromSectionNode(long);
    public static long pack(net.minecraft.core.BlockPos);
    public static int getX(long);
    public static int getZ(long);
    public int hashCode();
    public static int hash(int, int);
    public int getMiddleBlockX();
    public int getMiddleBlockZ();
    public int getMinBlockX();
    public int getMinBlockZ();
    public int getMaxBlockX();
    public int getMaxBlockZ();
    public int getRegionX();
    public int getRegionZ();
    public static int getRegionX(long);
    public static int getRegionZ(long);
    public int getRegionLocalX();
    public int getRegionLocalZ();
    public net.minecraft.core.BlockPos getBlockAt(int, int, int);
    public int getBlockX(int);
    public int getBlockZ(int);
    public net.minecraft.core.BlockPos getMiddleBlockPosition(int);
    public boolean contains(net.minecraft.core.BlockPos);
    public java.lang.String toString();
    public net.minecraft.core.BlockPos getWorldPosition();
    public int getChessboardDistance(net.minecraft.world.level.ChunkPos);
    public int getChessboardDistance(int, int);
    public int distanceSquared(net.minecraft.world.level.ChunkPos);
    public int distanceSquared(long);
    private int distanceSquared(int, int);
    public static java.util.stream.Stream<net.minecraft.world.level.ChunkPos> rangeClosed(net.minecraft.world.level.ChunkPos, int);
    public static java.util.stream.Stream<net.minecraft.world.level.ChunkPos> rangeClosed(net.minecraft.world.level.ChunkPos, net.minecraft.world.level.ChunkPos);
    public final boolean equals(java.lang.Object);
    public int x();
    public int z();
    private static java.util.stream.IntStream lambda$static$2(net.minecraft.world.level.ChunkPos);
    private static com.mojang.serialization.DataResult lambda$static$0(java.util.stream.IntStream);
    private static net.minecraft.world.level.ChunkPos lambda$static$1(int[]);
    static {};
}
```
