---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry$TagSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry$TagSet

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@95 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@262 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (0 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static unbound()Lnet/minecraft/core/MappedRegistry$TagSet;
public static fromMap(Ljava/util/Map;)Lnet/minecraft/core/MappedRegistry$TagSet;
public abstract isBound()Z
public abstract get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
public abstract forEach(Ljava/util/function/BiConsumer;)V
public abstract getTags()Ljava/util/stream/Stream;
```
