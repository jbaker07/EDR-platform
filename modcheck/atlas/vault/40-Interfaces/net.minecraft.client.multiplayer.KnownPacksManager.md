---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.KnownPacksManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.KnownPacksManager

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `<init>` | `@Redirect at INVOKE Lnet/minecraft/server/packs/repository/ServerPacksSource;cre` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.KnownPacksManager {
    private final net.minecraft.server.packs.repository.PackRepository repository;
    private final java.util.Map<net.minecraft.server.packs.repository.KnownPack, java.lang.String> knownPackToId;
    public net.minecraft.client.multiplayer.KnownPacksManager();
    public java.util.List<net.minecraft.server.packs.repository.KnownPack> trySelectingPacks(java.util.List<net.minecraft.server.packs.repository.KnownPack>);
    public net.minecraft.server.packs.resources.CloseableResourceManager createResourceManager();
    private static void lambda$new$0(com.google.common.collect.ImmutableMap$Builder, net.minecraft.server.packs.repository.Pack);
    private static void lambda$new$1(com.google.common.collect.ImmutableMap$Builder, net.minecraft.server.packs.PackLocationInfo, net.minecraft.server.packs.repository.KnownPack);
}
```
