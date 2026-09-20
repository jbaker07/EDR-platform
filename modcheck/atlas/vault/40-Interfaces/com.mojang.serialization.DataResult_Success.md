---
type: "interface"
fqcn: "com.mojang.serialization.DataResult$Success"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.DataResult$Success

Package `com.mojang.serialization`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `lifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokevirtual@249 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@235 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@60 in `BundleContentsStorage$BundleSlotWrapper.getWeight` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
