---
type: "interface"
fqcn: "net.minecraft.core.SectionPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.SectionPos

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `of(J)Lnet/minecraft/core/SectionPos;` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord(I)I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `x()I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `y()I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `z()I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |

## Declared members (75, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.SectionPos extends net.minecraft.core.Vec3i {
    public static final int SECTION_BITS;
    public static final int SECTION_SIZE;
    public static final int SECTION_BLOCK_COUNT;
    public static final int SECTION_MASK;
    public static final int SECTION_HALF_SIZE;
    public static final int SECTION_MAX_INDEX;
    private static final int PACKED_X_LENGTH;
    private static final int PACKED_Y_LENGTH;
    private static final int PACKED_Z_LENGTH;
    private static final long PACKED_X_MASK;
    private static final long PACKED_Y_MASK;
    private static final long PACKED_Z_MASK;
    private static final int Y_OFFSET;
    private static final int Z_OFFSET;
    private static final int X_OFFSET;
    private static final int RELATIVE_X_SHIFT;
    private static final int RELATIVE_Y_SHIFT;
    private static final int RELATIVE_Z_SHIFT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.core.SectionPos> STREAM_CODEC;
    private net.minecraft.core.SectionPos(int, int, int);
    public static net.minecraft.core.SectionPos of(int, int, int);
    public static net.minecraft.core.SectionPos of(net.minecraft.core.BlockPos);
    public static net.minecraft.core.SectionPos of(net.minecraft.world.level.ChunkPos, int);
    public static net.minecraft.core.SectionPos of(net.minecraft.world.level.entity.EntityAccess);
    public static net.minecraft.core.SectionPos of(net.minecraft.core.Position);
    public static net.minecraft.core.SectionPos of(long);
    public static net.minecraft.core.SectionPos bottomOf(net.minecraft.world.level.chunk.ChunkAccess);
    public static long offset(long, net.minecraft.core.Direction);
    public static long offset(long, int, int, int);
    public static int posToSectionCoord(double);
    public static int blockToSectionCoord(int);
    public static int blockToSectionCoord(double);
    public static int sectionRelative(int);
    public static short sectionRelativePos(net.minecraft.core.BlockPos);
    public static int sectionRelativeX(short);
    public static int sectionRelativeY(short);
    public static int sectionRelativeZ(short);
    public int relativeToBlockX(short);
    public int relativeToBlockY(short);
    public int relativeToBlockZ(short);
    public net.minecraft.core.BlockPos relativeToBlockPos(short);
    public static int sectionToBlockCoord(int);
    public static int sectionToBlockCoord(int, int);
    public static int x(long);
    public static int y(long);
    public static int z(long);
    public int x();
    public int y();
    public int z();
    public int minBlockX();
    public int minBlockY();
    public int minBlockZ();
    public int maxBlockX();
    public int maxBlockY();
    public int maxBlockZ();
    public static long blockToSection(long);
    public static long getZeroNode(int, int);
    public static long getZeroNode(long);
    public static long sectionToChunk(long);
    public net.minecraft.core.BlockPos origin();
    public net.minecraft.core.BlockPos center();
    public net.minecraft.world.level.ChunkPos chunk();
    public static long asLong(net.minecraft.core.BlockPos);
    public static long asLong(int, int, int);
    public long asLong();
    public net.minecraft.core.SectionPos offset(int, int, int);
    public java.util.stream.Stream<net.minecraft.core.BlockPos> blocksInside();
    public static java.util.stream.Stream<net.minecraft.core.SectionPos> cube(net.minecraft.core.SectionPos, int);
    public static java.util.stream.Stream<net.minecraft.core.SectionPos> aroundChunk(net.minecraft.world.level.ChunkPos, int, int, int);
    public static java.util.stream.Stream<net.minecraft.core.SectionPos> betweenClosedStream(int, int, int, int, int, int);
    public static void aroundAndAtBlockPos(net.minecraft.core.BlockPos, it.unimi.dsi.fastutil.longs.LongConsumer);
    public static void aroundAndAtBlockPos(long, it.unimi.dsi.fastutil.longs.LongConsumer);
    public static void aroundAndAtBlockPos(int, int, int, it.unimi.dsi.fastutil.longs.LongConsumer);
    public net.minecraft.core.Vec3i offset(int, int, int);
    static {};
}
```
