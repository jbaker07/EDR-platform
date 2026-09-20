---
type: "interface"
fqcn: "net.minecraft.tags.TagLoader$EntryWithSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagLoader$EntryWithSource

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/tags/TagEntry;Ljava/lang/String;)V` | exact | invokespecial@107 in `TagRemovalInternals.addRemoveEntry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/tags/TagEntry;Ljava/lang/String;)V` | exact | invokespecial@42 in `TagLoaderMixin.loadRemoveEntries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `entry` | `()Lnet/minecraft/tags/TagEntry;` | exact | invokevirtual@42 in `TagRemovalInternals.addRemoveEntry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `source` | `()Ljava/lang/String;` | exact | invokevirtual@104 in `TagRemovalInternals.addRemoveEntry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `source` | `()Ljava/lang/String;` | exact | invokevirtual@1 in `TagRemovalInternals.lambda$mergeAddedAndRemovedEntries$1` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `source` | `()Ljava/lang/String;` | exact | invokevirtual@1 in `TagRemovalInternals.lambda$mergeAddedAndRemovedEntries$0` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entry : Lnet/minecraft/tags/TagEntry;
private final source : Ljava/lang/String;
public <init>(Lnet/minecraft/tags/TagEntry;Ljava/lang/String;)V
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public entry()Lnet/minecraft/tags/TagEntry;
public source()Ljava/lang/String;
```
