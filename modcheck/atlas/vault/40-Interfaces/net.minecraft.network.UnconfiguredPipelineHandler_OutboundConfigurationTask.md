---
type: "interface"
fqcn: "net.minecraft.network.UnconfiguredPipelineHandler$OutboundConfigurationTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.UnconfiguredPipelineHandler$OutboundConfigurationTask

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `andThen(Lnet/minecraft/network/UnconfiguredPipelineHandler$Outbound` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.UnconfiguredPipelineHandler$OutboundConfigurationTask {
    public abstract void run(io.netty.channel.ChannelHandlerContext);
    public default net.minecraft.network.UnconfiguredPipelineHandler$OutboundConfigurationTask andThen(net.minecraft.network.UnconfiguredPipelineHandler$OutboundConfigurationTask);
    private void lambda$andThen$0(net.minecraft.network.UnconfiguredPipelineHandler$OutboundConfigurationTask, io.netty.channel.ChannelHandlerContext);
}
```
