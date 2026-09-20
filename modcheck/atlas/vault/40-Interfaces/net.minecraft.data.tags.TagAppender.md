---
type: "interface"
fqcn: "net.minecraft.data.tags.TagAppender"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.tags.TagAppender

System: [[20-Systems/net.minecraft.data.tags|net.minecraft.data.tags]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/datagen/v1/provider/FabricTagAppender`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `forBuilder` | `(Lnet/minecraft/tags/TagBuilder;)Lnet/minecraft/data/tags/TagAppender;` | exact | invokestatic@7 in `FabricTagsProvider.builder` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.setReplace` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.forceAddTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.remove` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.remove` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.removeAll` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.removeAll` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getBuilder` | `()Lnet/minecraft/tags/TagBuilder;` | inherited_exact | invokeinterface@4 in `BlockItemTagAppenderMixin.removeTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (0 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract add(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public add([Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public addAll(Ljava/util/Collection;)Lnet/minecraft/data/tags/TagAppender;
public addAll(Ljava/util/stream/Stream;)Lnet/minecraft/data/tags/TagAppender;
public abstract addOptional(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/data/tags/TagAppender;
public abstract addTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
public abstract addOptionalTag(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/tags/TagAppender;
public static forBuilder(Lnet/minecraft/tags/TagBuilder;)Lnet/minecraft/data/tags/TagAppender;
```
