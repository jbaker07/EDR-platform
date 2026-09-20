---
type: "interface"
fqcn: "net.minecraft.network.protocol.PacketType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.PacketType

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/protocol/PacketFlow;Lnet/minecraft/resources/I` | exact | invokespecial@14 in `PassthroughPacket.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@52 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@239 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@252 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final flow : Lnet/minecraft/network/protocol/PacketFlow;
private final id : Lnet/minecraft/resources/Identifier;
public <init>(Lnet/minecraft/network/protocol/PacketFlow;Lnet/minecraft/resources/Identifier;)V
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public flow()Lnet/minecraft/network/protocol/PacketFlow;
public id()Lnet/minecraft/resources/Identifier;
```
