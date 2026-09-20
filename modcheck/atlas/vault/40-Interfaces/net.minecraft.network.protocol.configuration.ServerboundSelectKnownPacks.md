---
type: "interface"
fqcn: "net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<clinit>` | `@ModifyArg at INVOKE Lnet/minecraft/network/codec/ByteBufCodecs;list(I)Lnet/mine` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener> {
    private final java.util.List<net.minecraft.server.packs.repository.KnownPack> knownPacks;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks> STREAM_CODEC;
    public net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks(java.util.List<net.minecraft.server.packs.repository.KnownPack>);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks> type();
    public void handle(net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.List<net.minecraft.server.packs.repository.KnownPack> knownPacks();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
