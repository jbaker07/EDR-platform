---
type: "interface"
fqcn: "net.minecraft.server.network.ConfigurationTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ConfigurationTask

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `start(Ljava/util/function/Consumer;)V` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/server/network/ConfigurationTask$Type;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.network.ConfigurationTask {
    public abstract void start(java.util.function.Consumer<net.minecraft.network.protocol.Packet<?>>);
    public default boolean tick();
    public abstract net.minecraft.server.network.ConfigurationTask$Type type();
}
```
