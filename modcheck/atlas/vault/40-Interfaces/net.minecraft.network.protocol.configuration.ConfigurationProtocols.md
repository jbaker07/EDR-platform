---
type: "interface"
fqcn: "net.minecraft.network.protocol.configuration.ConfigurationProtocols"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.configuration.ConfigurationProtocols

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CLIENTBOUND_TEMPLATE` | `Lnet/minecraft/network/protocol/SimpleUnboundProtocol;` | exact | getstatic@18 in `VanillaPacketTypes.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `SERVERBOUND_TEMPLATE` | `Lnet/minecraft/network/protocol/SimpleUnboundProtocol;` | exact | getstatic@27 in `VanillaPacketTypes.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SERVERBOUND_TEMPLATE : Lnet/minecraft/network/protocol/SimpleUnboundProtocol;
public static final SERVERBOUND : Lnet/minecraft/network/ProtocolInfo;
public static final CLIENTBOUND_TEMPLATE : Lnet/minecraft/network/protocol/SimpleUnboundProtocol;
public static final CLIENTBOUND : Lnet/minecraft/network/ProtocolInfo;
public <init>()V
private static synthetic lambda$static$1(Lnet/minecraft/network/protocol/ProtocolInfoBuilder;)V
private static synthetic lambda$static$0(Lnet/minecraft/network/protocol/ProtocolInfoBuilder;)V
static <clinit>()V
```
