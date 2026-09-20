---
type: "interface"
fqcn: "com.mojang.datafixers.util.Pair"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.datafixers.util.Pair

Package `com.mojang.datafixers.util`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFirst` | `()Ljava/lang/Object;` | exact | invokevirtual@6 in `FailSoftMapCodec.lambda$decode$2` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getFirst` | `()Ljava/lang/Object;` | exact | invokevirtual@93 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getFirst` | `()Ljava/lang/Object;` | exact | invokevirtual@9 in `TransformCopyingModel.setupAnim` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getFirst` | `()Ljava/lang/Object;` | exact | invokevirtual@2 in `SpecialCodecs$1.lambda$decode$0` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `getSecond` | `()Ljava/lang/Object;` | exact | invokevirtual@22 in `FailSoftMapCodec.lambda$decode$2` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getSecond` | `()Ljava/lang/Object;` | exact | invokevirtual@75 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getSecond` | `()Ljava/lang/Object;` | exact | invokevirtual@38 in `TransformCopyingModel.setupAnim` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@86 in `NetherBiomeData.withModdedBiomeEntries` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@71 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@2 in `FailSoftMapCodec.lambda$decode$1` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@6 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@15 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@24 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@33 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@11 in `ArmorRenderer.submitTransformCopyingModel` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `of` | `(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair` | exact | invokestatic@11 in `ArmorRenderer.submitTransformCopyingModel` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
