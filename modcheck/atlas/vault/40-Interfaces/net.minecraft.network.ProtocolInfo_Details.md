---
type: "interface"
fqcn: "net.minecraft.network.ProtocolInfo$Details"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.ProtocolInfo$Details

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `listPackets` | `(Lnet/minecraft/network/ProtocolInfo$Details$PacketVisitor;)V` | exact | invokeinterface@20 in `VanillaPacketTypes.of` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract id()Lnet/minecraft/network/ConnectionProtocol;
public abstract flow()Lnet/minecraft/network/protocol/PacketFlow;
public abstract listPackets(Lnet/minecraft/network/ProtocolInfo$Details$PacketVisitor;)V
```
