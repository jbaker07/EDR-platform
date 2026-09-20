---
type: "interface"
fqcn: "net.minecraft.network.ProtocolInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.ProtocolInfo

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `flow()Lnet/minecraft/network/protocol/PacketFlow;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `flow()Lnet/minecraft/network/protocol/PacketFlow;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/network/ConnectionProtocol;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/network/ConnectionProtocol;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.ProtocolInfo<T extends net.minecraft.network.PacketListener> {
    public abstract net.minecraft.network.ConnectionProtocol id();
    public abstract net.minecraft.network.protocol.PacketFlow flow();
    public abstract net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.network.protocol.Packet<? super T>> codec();
    public abstract net.minecraft.network.protocol.BundlerInfo bundlerInfo();
}
```
