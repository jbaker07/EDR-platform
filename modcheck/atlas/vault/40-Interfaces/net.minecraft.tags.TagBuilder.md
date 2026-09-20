---
type: "interface"
fqcn: "net.minecraft.tags.TagBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagBuilder

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `add` | `(Lnet/minecraft/tags/TagEntry;)Lnet/minecraft/tags/TagBuilder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | declared |
| calls | `build` | `()Ljava/util/List;` | exact | invokevirtual@30 in `FabricTagsProvider$ItemTagsProvider.copy` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entries : Ljava/util/List;
private replace : Z
public <init>()V
public static create()Lnet/minecraft/tags/TagBuilder;
public build()Ljava/util/List;
public shouldReplace()Z
public setReplace(Z)Lnet/minecraft/tags/TagBuilder;
public add(Lnet/minecraft/tags/TagEntry;)Lnet/minecraft/tags/TagBuilder;
public addElement(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagBuilder;
public addOptionalElement(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagBuilder;
public addTag(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagBuilder;
public addOptionalTag(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagBuilder;
```
