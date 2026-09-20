---
type: "interface"
fqcn: "net.minecraft.network.chat.FilterMask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.FilterMask

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `applyWithFormatting(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;` | `` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.chat.FilterMask {
    public static final com.mojang.serialization.Codec<net.minecraft.network.chat.FilterMask> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.network.chat.FilterMask> STREAM_CODEC;
    public static final net.minecraft.network.chat.FilterMask FULLY_FILTERED;
    public static final net.minecraft.network.chat.FilterMask PASS_THROUGH;
    public static final net.minecraft.network.chat.Style FILTERED_STYLE;
    private static final char HASH;
    private final java.util.BitSet mask;
    private final net.minecraft.network.chat.FilterMask$Type type;
    private net.minecraft.network.chat.FilterMask(java.util.BitSet, net.minecraft.network.chat.FilterMask$Type);
    private net.minecraft.network.chat.FilterMask(java.util.BitSet);
    public net.minecraft.network.chat.FilterMask(int);
    private net.minecraft.network.chat.FilterMask$Type type();
    private java.util.BitSet mask();
    public void setFiltered(int);
    public java.lang.String apply(java.lang.String);
    public net.minecraft.network.chat.Component applyWithFormatting(java.lang.String);
    public boolean isEmpty();
    public boolean isFullyFiltered();
    public boolean equals(java.lang.Object);
    public int hashCode();
    static {};
}
```
