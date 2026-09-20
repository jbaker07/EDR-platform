---
type: "interface"
fqcn: "com.mojang.serialization.MapLike"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.MapLike

Package `com.mojang.serialization`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `entries` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@5 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `entries` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@1 in `SpecialCodecs$1.decode` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `get` | `(Ljava/lang/String;)Ljava/lang/Object;` | exact | invokeinterface@5 in `CustomUnbakedBlockStateModelRegistry$KeyExistsCodec.decode` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `get` | `(Ljava/lang/String;)Ljava/lang/Object;` | exact | invokeinterface@5 in `SpecialCodecs$2.decode` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
