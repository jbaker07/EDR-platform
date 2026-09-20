---
type: "interface"
fqcn: "com.mojang.serialization.DynamicOps"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.DynamicOps

Package `com.mojang.serialization`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `convertTo` | `(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Ljava/lang/Ob` | exact | invokeinterface@30 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `empty` | `()Ljava/lang/Object;` | exact | invokeinterface@66 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getMap` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@2 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getStringValue` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@5 in `SpecialCodecs$1.lambda$decode$0` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `mapBuilder` | `()Lcom/mojang/serialization/RecordBuilder;` | exact | invokeinterface@4 in `FailSoftMapCodec.encode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
