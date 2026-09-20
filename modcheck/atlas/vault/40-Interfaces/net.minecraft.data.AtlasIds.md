---
type: "interface"
fqcn: "net.minecraft.data.AtlasIds"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.AtlasIds

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@1 in `QuadAtlas.ofId` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@10 in `QuadAtlas.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@1 in `MaterialBakerMixin.spriteFinder` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `ITEMS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@15 in `QuadAtlas.ofId` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `ITEMS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@29 in `QuadAtlas.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `ITEMS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@19 in `MaterialBakerMixin.spriteFinder` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `PARTICLES` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@6 in `FabricSpriteSetImpl.getAtlas` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (12 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final BANNER_PATTERNS : Lnet/minecraft/resources/Identifier;
public static final BLOCKS : Lnet/minecraft/resources/Identifier;
public static final ITEMS : Lnet/minecraft/resources/Identifier;
public static final CHESTS : Lnet/minecraft/resources/Identifier;
public static final DECORATED_POT : Lnet/minecraft/resources/Identifier;
public static final GUI : Lnet/minecraft/resources/Identifier;
public static final MAP_DECORATIONS : Lnet/minecraft/resources/Identifier;
public static final PAINTINGS : Lnet/minecraft/resources/Identifier;
public static final PARTICLES : Lnet/minecraft/resources/Identifier;
public static final SHIELD_PATTERNS : Lnet/minecraft/resources/Identifier;
public static final SHULKER_BOXES : Lnet/minecraft/resources/Identifier;
public static final CELESTIALS : Lnet/minecraft/resources/Identifier;
public <init>()V
static <clinit>()V
```
