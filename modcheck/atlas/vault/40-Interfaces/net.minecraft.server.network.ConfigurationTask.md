---
type: "interface"
fqcn: "net.minecraft.server.network.ConfigurationTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ConfigurationTask

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `start` | `(Ljava/util/function/Consumer;)V` | exact | invokeinterface@87 in `ServerConfigurationPacketListenerImplMixin.pollEarlyTasks` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/server/network/ConfigurationTask$Type;` | exact | invokeinterface@25 in `ServerConfigurationPacketListenerImplMixin.onClientReady` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/server/network/ConfigurationTask$Type;` | exact | invokeinterface@32 in `ServerConfigurationPacketListenerImplMixin.pollEarlyTasks` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/server/network/ConfigurationTask$Type;` | exact | invokeinterface@24 in `ServerConfigurationPacketListenerImplMixin.completeTask` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract start(Ljava/util/function/Consumer;)V
public tick()Z
public abstract type()Lnet/minecraft/server/network/ConfigurationTask$Type;
```
