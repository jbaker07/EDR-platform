---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.ResourceManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.ResourceManager

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/ResourceProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getResource` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | inherited_exact | invokeinterface@13 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getNamespaces()Ljava/util/Set;
public abstract getResourceStack(Lnet/minecraft/resources/Identifier;)Ljava/util/List;
public abstract listResources(Ljava/lang/String;Lnet/minecraft/server/packs/resources/ResourceManager$Selector;)Ljava/util/Map;
public abstract listResourceStacks(Ljava/lang/String;Lnet/minecraft/server/packs/resources/ResourceManager$Selector;)Ljava/util/Map;
public abstract listPacks()Ljava/util/stream/Stream;
```
