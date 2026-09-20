---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemStackRenderState$FoilType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemStackRenderState$FoilType

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@11 in `EncodingFormat.foilType` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@97 in `ExtendedItemFeatureRenderer.bufferMain` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | invokestatic@161 in `EncodingFormat.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | invokestatic@0 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NONE` | `Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | getstatic@42 in `ModelHelper.computeMaterialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NONE` | `Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | getstatic@173 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NONE` | `Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | getstatic@12 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SPECIAL` | `Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | getstatic@42 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `SPECIAL` | `Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | getstatic@168 in `ExtendedItemFeatureRenderer.bufferMain` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `STANDARD` | `Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;` | exact | getstatic@27 in `ExtendedItemFeatureRenderer$2.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NONE : Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
public static final STANDARD : Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
public static final SPECIAL : Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
private static final synthetic $VALUES : [Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
public static values()[Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
private <init>(Ljava/lang/String;I)V
private static synthetic $values()[Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
static <clinit>()V
```
