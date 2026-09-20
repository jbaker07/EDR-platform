---
type: "interface"
fqcn: "com.mojang.serialization.codecs.PrimitiveCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.codecs.PrimitiveCodec

Package `com.mojang.serialization.codecs`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `comapFlatMap` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojan` | inherited_exact | invokeinterface@14 in `GameRuleBuilder$EnumRuleBuilder.createEnumCodec` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `listOf` | `()Lcom/mojang/serialization/Codec;` | inherited_exact | invokeinterface@4 in `AllModsLoadedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `listOf` | `()Lcom/mojang/serialization/Codec;` | inherited_exact | invokeinterface@4 in `AnyModsLoadedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `listOf` | `()Lcom/mojang/serialization/Codec;` | inherited_exact | invokeinterface@4 in `DefaultResourcePackStorage.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `optionalFieldOf` | `(Ljava/lang/String;)Lcom/mojang/serialization/MapCodec;` | inherited_exact | invokeinterface@51 in `SoundTypeBuilderImpl$SoundType.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `optionalFieldOf` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/serialization/MapCod` | inherited_exact | invokeinterface@125 in `SoundTypeBuilderImpl$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `optionalFieldOf` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/serialization/MapCod` | inherited_exact | invokeinterface@147 in `SoundTypeBuilderImpl$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `optionalFieldOf` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/serialization/MapCod` | inherited_exact | invokeinterface@169 in `SoundTypeBuilderImpl$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `optionalFieldOf` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/serialization/MapCod` | inherited_exact | invokeinterface@33 in `SoundTypeBuilderImpl$SoundType.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `validate` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | inherited_exact | invokeinterface@9 in `OverlayConditionsMetadata$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `xmap` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojan` | inherited_exact | invokeinterface@13 in `SpecialCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `xmap` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojan` | inherited_exact | invokeinterface@34 in `SpecialCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
