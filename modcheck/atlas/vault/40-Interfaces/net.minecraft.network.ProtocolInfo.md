---
type: "interface"
fqcn: "net.minecraft.network.ProtocolInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.ProtocolInfo

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `flow` | `()Lnet/minecraft/network/protocol/PacketFlow;` | exact | invokeinterface@41 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `flow` | `()Lnet/minecraft/network/protocol/PacketFlow;` | exact | invokeinterface@65 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `flow` | `()Lnet/minecraft/network/protocol/PacketFlow;` | exact | invokeinterface@41 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `flow` | `()Lnet/minecraft/network/protocol/PacketFlow;` | exact | invokeinterface@65 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/network/ConnectionProtocol;` | exact | invokeinterface@4 in `PayloadTypeRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/network/ConnectionProtocol;` | exact | invokeinterface@4 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/network/ConnectionProtocol;` | exact | invokeinterface@93 in `VanillaPacketTypes.get` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract id()Lnet/minecraft/network/ConnectionProtocol;
public abstract flow()Lnet/minecraft/network/protocol/PacketFlow;
public abstract codec()Lnet/minecraft/network/codec/StreamCodec;
public abstract bundlerInfo()Lnet/minecraft/network/protocol/BundlerInfo;
```
