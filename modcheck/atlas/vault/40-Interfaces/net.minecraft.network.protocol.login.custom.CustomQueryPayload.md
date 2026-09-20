---
type: "interface"
fqcn: "net.minecraft.network.protocol.login.custom.CustomQueryPayload"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.login.custom.CustomQueryPayload

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@17 in `ClientLoginNetworkAddon.handlePacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@15 in `ServerLoginNetworkAddon.registerOutgoingPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract id()Lnet/minecraft/resources/Identifier;
public abstract write(Lnet/minecraft/network/FriendlyByteBuf;)V
```
