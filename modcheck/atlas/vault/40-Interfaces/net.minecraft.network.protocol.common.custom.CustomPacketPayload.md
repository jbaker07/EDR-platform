---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `codec(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getClass()Ljava/lang/Class;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getClass()Ljava/lang/Class;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.protocol.common.custom.CustomPacketPayload {
    public abstract net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<? extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> type();
    public static <B extends io.netty.buffer.ByteBuf, T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> net.minecraft.network.codec.StreamCodec<B, T> codec(net.minecraft.network.codec.StreamMemberEncoder<B, T>, net.minecraft.network.codec.StreamDecoder<B, T>);
    public static <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T> createType(java.lang.String);
    public static <B extends net.minecraft.network.FriendlyByteBuf> net.minecraft.network.codec.StreamCodec<B, net.minecraft.network.protocol.common.custom.CustomPacketPayload> codec(net.minecraft.network.protocol.common.custom.CustomPacketPayload$FallbackProvider<B>, java.util.List<net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec<? super B, ?>>);
    private static net.minecraft.resources.Identifier lambda$codec$0(net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec);
}
```
