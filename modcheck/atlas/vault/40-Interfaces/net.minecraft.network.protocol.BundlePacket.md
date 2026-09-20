---
type: "interface"
fqcn: "net.minecraft.network.protocol.BundlePacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.BundlePacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/network/protocol/Packet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `subPackets` | `()Ljava/lang/Iterable;` | exact | invokevirtual@41 in `BundlePacketMixin.iterateBundle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Ljava/lang/Iterable;)V` | name_only | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final packets : Ljava/lang/Iterable;
protected <init>(Ljava/lang/Iterable;)V
public final subPackets()Ljava/lang/Iterable;
public abstract type()Lnet/minecraft/network/protocol/PacketType;
```
