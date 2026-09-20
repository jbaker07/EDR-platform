---
type: "interface"
fqcn: "net.minecraft.server.network.config.SynchronizeRegistriesTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.config.SynchronizeRegistriesTask

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `handleResponse` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `sendRegistries` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `start` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.network.config.SynchronizeRegistriesTask implements net.minecraft.server.network.ConfigurationTask {
    public static final net.minecraft.server.network.ConfigurationTask$Type TYPE;
    private final java.util.List<net.minecraft.server.packs.repository.KnownPack> requestedPacks;
    private final net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer> registries;
    public net.minecraft.server.network.config.SynchronizeRegistriesTask(java.util.List<net.minecraft.server.packs.repository.KnownPack>, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>);
    public void start(java.util.function.Consumer<net.minecraft.network.protocol.Packet<?>>);
    private void sendRegistries(java.util.function.Consumer<net.minecraft.network.protocol.Packet<?>>, java.util.Set<net.minecraft.server.packs.repository.KnownPack>);
    public void handleResponse(java.util.List<net.minecraft.server.packs.repository.KnownPack>, java.util.function.Consumer<net.minecraft.network.protocol.Packet<?>>);
    public net.minecraft.server.network.ConfigurationTask$Type type();
    private static void lambda$sendRegistries$0(java.util.function.Consumer, net.minecraft.resources.ResourceKey, java.util.List);
    static {};
}
```
