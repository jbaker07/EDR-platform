---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.BrandPayload"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.BrandPayload

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `brand()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.common.custom.BrandPayload extends java.lang.Record implements net.minecraft.network.protocol.common.custom.CustomPacketPayload {
    private final java.lang.String brand;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.common.custom.BrandPayload> STREAM_CODEC;
    public static final net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<net.minecraft.network.protocol.common.custom.BrandPayload> TYPE;
    private net.minecraft.network.protocol.common.custom.BrandPayload(net.minecraft.network.FriendlyByteBuf);
    public net.minecraft.network.protocol.common.custom.BrandPayload(java.lang.String);
    private void write(net.minecraft.network.FriendlyByteBuf);
    public net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<net.minecraft.network.protocol.common.custom.BrandPayload> type();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String brand();
    static {};
}
```
