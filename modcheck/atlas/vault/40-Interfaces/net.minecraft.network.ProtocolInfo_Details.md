---
type: "interface"
fqcn: "net.minecraft.network.ProtocolInfo$Details"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.ProtocolInfo$Details

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `listPackets(Lnet/minecraft/network/ProtocolInfo$Details$PacketVisitor;)` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.ProtocolInfo$Details {
    public abstract net.minecraft.network.ConnectionProtocol id();
    public abstract net.minecraft.network.protocol.PacketFlow flow();
    public abstract void listPackets(net.minecraft.network.ProtocolInfo$Details$PacketVisitor);
}
```
