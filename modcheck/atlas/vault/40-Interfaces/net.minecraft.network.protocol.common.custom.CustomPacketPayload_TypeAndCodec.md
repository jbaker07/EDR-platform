---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Typ` | exact | invokespecial@25 in `PayloadTypeRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `codec` | `()Lnet/minecraft/network/codec/StreamCodec;` | exact | invokevirtual@28 in `CustomPayloadStreamCodecMixin.wrapGetCodec` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final type : Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;
private final codec : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;Lnet/minecraft/network/codec/StreamCodec;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public type()Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;
public codec()Lnet/minecraft/network/codec/StreamCodec;
```
