---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.CloseableResourceManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.CloseableResourceManager

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/ResourceManager`, `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `listPacks` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@6 in `MinecraftServerMixin.init` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract close()V
```
