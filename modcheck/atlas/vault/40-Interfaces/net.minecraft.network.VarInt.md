---
type: "interface"
fqcn: "net.minecraft.network.VarInt"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.VarInt

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getByteSize(I)I` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getByteSize(I)I` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `read(Lio/netty/buffer/ByteBuf;)I` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `read(Lio/netty/buffer/ByteBuf;)I` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `write(Lio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.VarInt {
    public static final int MAX_VARINT_SIZE;
    private static final int DATA_BITS_MASK;
    private static final int CONTINUATION_BIT_MASK;
    private static final int DATA_BITS_PER_BYTE;
    public net.minecraft.network.VarInt();
    public static int getByteSize(int);
    public static boolean hasContinuationBit(byte);
    public static int read(io.netty.buffer.ByteBuf);
    public static io.netty.buffer.ByteBuf write(io.netty.buffer.ByteBuf, int);
}
```
