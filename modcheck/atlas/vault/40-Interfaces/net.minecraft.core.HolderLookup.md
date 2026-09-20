---
type: "interface"
fqcn: "net.minecraft.core.HolderLookup"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderLookup

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/HolderGetter`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `listElements` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@33 in `LootUtil.getEntryOrDirect` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract listElements()Ljava/util/stream/Stream;
public listElementIds()Ljava/util/stream/Stream;
public abstract listTags()Ljava/util/stream/Stream;
public listTagIds()Ljava/util/stream/Stream;
```
