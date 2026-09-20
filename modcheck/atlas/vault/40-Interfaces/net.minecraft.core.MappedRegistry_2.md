---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry$2"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry$2

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/core/HolderLookup$RegistryLookup$Delegate`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `parent` | `()Lnet/minecraft/core/HolderLookup$RegistryLookup;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | declared |

## Declared members (2 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$pendingTags : Lcom/google/common/collect/ImmutableMap;
final synthetic this$0 : Lnet/minecraft/core/MappedRegistry;
 <init>(Lnet/minecraft/core/MappedRegistry;Lcom/google/common/collect/ImmutableMap;)V
public parent()Lnet/minecraft/core/HolderLookup$RegistryLookup;
public get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
public listTags()Ljava/util/stream/Stream;
```
