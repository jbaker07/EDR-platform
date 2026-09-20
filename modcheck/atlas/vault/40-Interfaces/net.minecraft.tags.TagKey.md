---
type: "interface"
fqcn: "net.minecraft.tags.TagKey"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagKey

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`record` public final; extends `java/lang/Record`; implements `net/fabricmc/fabric/api/tag/FabricTagKey`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `codec` | `(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/Codec` | exact | invokestatic@1 in `TagAliasGroup.codec` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@9 in `BlockFunctionalityTags.create` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@10 in `TagRegistration.registerFabric` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@10 in `TagRegistration.registerC` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@12 in `FabricTagsProvider$AliasGroupBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@33 in `FabricTagsProvider$AliasGroupBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@84 in `ResourceConditionsImpl.tagsPopulated` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@8 in `ClientTagsLoader$1.tag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@8 in `ClientTagsLoader$2.tag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getTranslationKey` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@24 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarni | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getTranslationKey` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@2 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `isFor` | `(Lnet/minecraft/resources/ResourceKey;)Z` | exact | invokevirtual@64 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `isFor` | `(Lnet/minecraft/resources/ResourceKey;)Z` | exact | invokevirtual@25 in `ClientTagsImpl.getHolder` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@29 in `FabricTagKey.getTranslationKey` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `FabricTagKey.getName` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@126 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarn | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarnin | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@24 in `BlockItemTagAppenderMixin.forceAddTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@24 in `BlockItemTagAppenderMixin.removeTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `TagAppenderMixin$TagAppender1Mixin.forceAddTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `TagAppenderMixin$TagAppender1Mixin.removeTag` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@21 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@137 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `AdvancementHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `AdvancementHolderProvider.get` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@21 in `FabricTagKey.getTranslationKey` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@16 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@32 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `FabricTagsProvider$AliasGroupBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `LootTableHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `LootTableHolderProvider.get` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@4 in `TagsPopulatedResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@11 in `ClientTags.isInLocal` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@42 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@64 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@4 in `ClientTagsLoader$1.tag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@4 in `ClientTagsLoader$2.tag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@17 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@142 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (3 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final registry : Lnet/minecraft/resources/ResourceKey;
private final location : Lnet/minecraft/resources/Identifier;
private static final VALUES : Lcom/google/common/collect/Interner;
public <init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)V
public static codec(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/Codec;
public static hashedCodec(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/Codec;
public static streamCodec(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/StreamCodec;
public static create(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagKey;
public isFor(Lnet/minecraft/resources/ResourceKey;)Z
public cast(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public registry()Lnet/minecraft/resources/ResourceKey;
public location()Lnet/minecraft/resources/Identifier;
private static synthetic lambda$streamCodec$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagKey;
private static synthetic lambda$hashedCodec$3(Lnet/minecraft/tags/TagKey;)Ljava/lang/String;
private static synthetic lambda$hashedCodec$0(Lnet/minecraft/resources/ResourceKey;Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$hashedCodec$2()Ljava/lang/String;
private static synthetic lambda$hashedCodec$1(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagKey;
private static synthetic lambda$codec$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/TagKey;
static <clinit>()V
```
