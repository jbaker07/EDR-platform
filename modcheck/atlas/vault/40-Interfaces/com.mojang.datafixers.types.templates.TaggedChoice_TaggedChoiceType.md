---
type: "interface"
fqcn: "com.mojang.datafixers.types.templates.TaggedChoice$TaggedChoiceType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.datafixers.types.templates.TaggedChoice$TaggedChoiceType

Package `com.mojang.datafixers.types`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `getMapCodec` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| reads | `types` | `Lit/unimi/dsi/fastutil/objects/Object2ObjectMap;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | declared |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
