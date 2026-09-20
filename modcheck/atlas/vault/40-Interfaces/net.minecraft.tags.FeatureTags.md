---
type: "interface"
fqcn: "net.minecraft.tags.FeatureTags"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.FeatureTags

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CAN_SPAWN_FROM_BONE_MEAL` | `Lnet/minecraft/tags/TagKey;` | exact | getstatic@1 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$rebuildF | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CAN_SPAWN_FROM_BONE_MEAL : Lnet/minecraft/tags/TagKey;
private <init>()V
private static create(Ljava/lang/String;)Lnet/minecraft/tags/TagKey;
static <clinit>()V
```
