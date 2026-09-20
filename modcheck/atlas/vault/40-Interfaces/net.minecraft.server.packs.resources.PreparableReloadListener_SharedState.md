---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.PreparableReloadListener$SharedState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.PreparableReloadListener$SharedState

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get(Lnet/minecraft/server/packs/resources/PreparableReloadListe` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `resourceManager()Lnet/minecraft/server/packs/resources/ResourceManager;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `set(Lnet/minecraft/server/packs/resources/PreparableReloadListe` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.packs.resources.PreparableReloadListener$SharedState {
    private final net.minecraft.server.packs.resources.ResourceManager manager;
    private final java.util.Map<net.minecraft.server.packs.resources.PreparableReloadListener$StateKey<?>, java.lang.Object> state;
    public net.minecraft.server.packs.resources.PreparableReloadListener$SharedState(net.minecraft.server.packs.resources.ResourceManager);
    public net.minecraft.server.packs.resources.ResourceManager resourceManager();
    public <T> void set(net.minecraft.server.packs.resources.PreparableReloadListener$StateKey<T>, T);
    public <T> T get(net.minecraft.server.packs.resources.PreparableReloadListener$StateKey<T>);
}
```
