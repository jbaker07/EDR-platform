---
type: "interface"
fqcn: "net.minecraft.data.tags.TagAppender$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.tags.TagAppender$1

System: [[20-Systems/net.minecraft.data.tags|net.minecraft.data.tags]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/data/tags/TagAppender`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `val$builder` | `Lnet/minecraft/tags/TagBuilder;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | declared |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$builder : Lnet/minecraft/tags/TagBuilder;
 <init>(Lnet/minecraft/tags/TagBuilder;)V
public add(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public addOptional(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public addTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
public addOptionalTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
```
