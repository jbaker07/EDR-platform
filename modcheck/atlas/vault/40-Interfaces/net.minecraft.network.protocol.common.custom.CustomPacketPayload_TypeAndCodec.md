---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/protocol/common/custom/CustomPacketP` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec()Lnet/minecraft/network/codec/StreamCodec;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec<B extends net.minecraft.network.FriendlyByteBuf, T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> extends java.lang.Record {
    private final net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T> type;
    private final net.minecraft.network.codec.StreamCodec<B, T> codec;
    public net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.minecraft.network.codec.StreamCodec<B, T>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T> type();
    public net.minecraft.network.codec.StreamCodec<B, T> codec();
}
```
