---
type: "interface"
fqcn: "net.minecraft.network.protocol.PacketType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.PacketType

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/protocol/PacketFlow;Lnet/minecraft/r` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.PacketType<T extends net.minecraft.network.protocol.Packet<?>> extends java.lang.Record {
    private final net.minecraft.network.protocol.PacketFlow flow;
    private final net.minecraft.resources.Identifier id;
    public net.minecraft.network.protocol.PacketType(net.minecraft.network.protocol.PacketFlow, net.minecraft.resources.Identifier);
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.protocol.PacketFlow flow();
    public net.minecraft.resources.Identifier id();
}
```
