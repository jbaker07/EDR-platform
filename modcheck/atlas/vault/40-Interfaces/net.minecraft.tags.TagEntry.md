---
type: "interface"
fqcn: "net.minecraft.tags.TagEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagEntry

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;ZZ)V` | exact | invokespecial@4 in `ForcedTagEntry.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Consumer;)Z` | exact | invokevirtual@293 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Consumer;)Z` | exact | invokevirtual@347 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `build` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Consumer;)Z` | exact | invokevirtual@22 in `TagLoaderMixin.removeEntriesFromTags` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `element` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;` | exact | invokestatic@5 in `TagBuilderMixin.fabric_removeElement` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `optionalElement` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;` | exact | invokestatic@76 in `TagRemovalInternals.addRemoveEntry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `optionalTag` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;` | exact | invokestatic@64 in `TagRemovalInternals.addRemoveEntry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `tag` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;` | exact | invokestatic@5 in `TagBuilderMixin.fabric_removeTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@11 in `TagRemovalInternals.lambda$modifyTagFileCodec$0` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (5 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final FULL_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final id : Lnet/minecraft/resources/Identifier;
private final tag : Z
private final required : Z
private <init>(Lnet/minecraft/resources/Identifier;ZZ)V
private <init>(Lnet/minecraft/util/ExtraCodecs$TagOrElementLocation;Z)V
private elementOrTag()Lnet/minecraft/util/ExtraCodecs$TagOrElementLocation;
public static element(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;
public static optionalElement(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;
public static tag(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;
public static optionalTag(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagEntry;
public build(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Consumer;)Z
public visitRequiredDependencies(Ljava/util/function/Consumer;)V
public visitOptionalDependencies(Ljava/util/function/Consumer;)V
public verifyIfPresent(Ljava/util/function/Predicate;Ljava/util/function/Predicate;)Z
public toString()Ljava/lang/String;
private static synthetic lambda$static$5(Lnet/minecraft/tags/TagEntry;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$static$2(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/tags/TagEntry;
private static synthetic lambda$static$4(Lnet/minecraft/tags/TagEntry;)Lnet/minecraft/tags/TagEntry;
private static synthetic lambda$static$3(Lnet/minecraft/util/ExtraCodecs$TagOrElementLocation;)Lnet/minecraft/tags/TagEntry;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$1(Lnet/minecraft/tags/TagEntry;)Ljava/lang/Boolean;
static <clinit>()V
```
