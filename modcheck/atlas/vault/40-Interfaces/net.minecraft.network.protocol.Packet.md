---
type: "interface"
fqcn: "net.minecraft.network.protocol.Packet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.Packet

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isTerminal` | `()Z` | exact | invokeinterface@425 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isTerminal` | `()Z` | exact | invokeinterface@1 in `FabricPacketMerger.ensureNotTransitioning` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isTerminal` | `()Z` | exact | invokeinterface@53 in `FabricPacketSplitter.encode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/PacketType;` | exact | invokeinterface@47 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/PacketType;` | exact | invokeinterface@221 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/network/protocol/PacketType;` | exact | invokeinterface@234 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract type()Lnet/minecraft/network/protocol/PacketType;
public abstract handle(Lnet/minecraft/network/PacketListener;)V
public isSkippable()Z
public isTerminal()Z
public static codec(Lnet/minecraft/network/codec/StreamMemberEncoder;Lnet/minecraft/network/codec/StreamDecoder;)Lnet/minecraft/network/codec/StreamCodec;
```
