---
type: "interface"
fqcn: "net.minecraft.core.IdMapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.IdMapper

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/IdMap`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addMapping` | `(Ljava/lang/Object;I)V` | exact | invokevirtual@28 in `IdMapperTracker.onEntryAdded` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `idToT` | `Ljava/util/List;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `nextId` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |
| reads | `tToId` | `Lit/unimi/dsi/fastutil/objects/Reference2IntMap;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | declared |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private nextId : I
private final tToId : Lit/unimi/dsi/fastutil/objects/Reference2IntMap;
private final idToT : Ljava/util/List;
public <init>()V
public <init>(I)V
public addMapping(Ljava/lang/Object;I)V
public add(Ljava/lang/Object;)V
public getId(Ljava/lang/Object;)I
public final byId(I)Ljava/lang/Object;
public iterator()Ljava/util/Iterator;
public contains(I)Z
public size()I
```
