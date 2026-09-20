---
type: "interface"
fqcn: "com.mojang.logging.LogUtils"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.logging.LogUtils

Package `com.mojang.logging`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLogger` | `()Lorg/slf4j/Logger;` | exact | invokestatic@0 in `SimpleUnbakedExtraModel.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getLogger` | `()Lorg/slf4j/Logger;` | exact | invokestatic@0 in `ClientPlayNetworkAddon.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getLogger` | `()Lorg/slf4j/Logger;` | exact | invokestatic@0 in `RegistryCustomContentState.<clinit>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getLogger` | `()Lorg/slf4j/Logger;` | exact | invokestatic@0 in `ResourceLoaderImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getLogger` | `()Lorg/slf4j/Logger;` | exact | invokestatic@0 in `DefaultResourcePackStorage.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `FATAL_MARKER` | `Lorg/slf4j/Marker;` | exact | getstatic@10 in `FabricDataGenHelper.run` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
