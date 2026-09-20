---
type: "interface"
fqcn: "net.minecraft.network.protocol.BundlePacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.BundlePacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `subPackets()Ljava/lang/Iterable;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `@ModifyVariable at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.network.protocol.BundlePacket<T extends net.minecraft.network.PacketListener> implements net.minecraft.network.protocol.Packet<T> {
    private final java.lang.Iterable<net.minecraft.network.protocol.Packet<? super T>> packets;
    protected net.minecraft.network.protocol.BundlePacket(java.lang.Iterable<net.minecraft.network.protocol.Packet<? super T>>);
    public final java.lang.Iterable<net.minecraft.network.protocol.Packet<? super T>> subPackets();
    public abstract net.minecraft.network.protocol.PacketType<? extends net.minecraft.network.protocol.BundlePacket<T>> type();
}
```
