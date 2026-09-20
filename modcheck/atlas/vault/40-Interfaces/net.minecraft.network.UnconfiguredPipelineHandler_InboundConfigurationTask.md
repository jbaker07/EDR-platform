---
type: "interface"
fqcn: "net.minecraft.network.UnconfiguredPipelineHandler$InboundConfigurationTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.UnconfiguredPipelineHandler$InboundConfigurationTask

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `andThen` | `(Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurati` | exact | invokeinterface@10 in `ConnectionMixin.injectFabricPacketSlitterHandlerInbound` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `andThen` | `(Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurati` | exact | invokeinterface@38 in `ConnectionMixin.injectFabricPacketSlitterHandlerInbound` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract run(Lio/netty/channel/ChannelHandlerContext;)V
public andThen(Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurationTask;)Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurationTask;
private synthetic lambda$andThen$0(Lnet/minecraft/network/UnconfiguredPipelineHandler$InboundConfigurationTask;Lio/netty/channel/ChannelHandlerContext;)V
```
