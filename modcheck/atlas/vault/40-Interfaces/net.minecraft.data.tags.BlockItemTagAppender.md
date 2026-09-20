---
type: "interface"
fqcn: "net.minecraft.data.tags.BlockItemTagAppender"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.tags.BlockItemTagAppender

System: [[20-Systems/net.minecraft.data.tags|net.minecraft.data.tags]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/data/tags/TagAppender`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/data/tags/TagAppender;)V` | exact | invokespecial@9 in `FabricTagsProvider$BlockTagsProvider$1.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/data/tags/TagAppender;)V` | exact | invokespecial@9 in `FabricTagsProvider$ItemTagsProvider$1.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `original` | `Lnet/minecraft/data/tags/TagAppender;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | declared |

## Declared members (1 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final original : Lnet/minecraft/data/tags/TagAppender;
public <init>(Lnet/minecraft/data/tags/TagAppender;)V
protected abstract convertElement(Lnet/minecraft/references/BlockItemId;)Lnet/minecraft/resources/ResourceKey;
public add(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public add([Lnet/minecraft/references/BlockItemId;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public addAll(Lnet/minecraft/world/level/block/ColorCollection;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public addAll(Lnet/minecraft/world/level/block/WeatheringCopperCollection;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public final add([Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public addOptional(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public addTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public addOptionalTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/BlockItemTagAppender;
public synthetic addOptionalTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
public synthetic addTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
public synthetic addOptional(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public synthetic add([Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public synthetic add(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
```
