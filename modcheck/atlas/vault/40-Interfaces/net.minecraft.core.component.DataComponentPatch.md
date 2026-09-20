---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentPatch"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentPatch

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | exact | invokestatic@2 in `DefaultCustomIngredients.components` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | exact | invokestatic@0 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | exact | invokestatic@0 in `TransferApiImpl.mergePatches` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | exact | invokestatic@156 in `BundleContentsStorage$BundleSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | exact | invokestatic@71 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentPatch$Builder;` | exact | invokestatic@77 in `ItemContainerContentsStorage$ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@51 in `ComponentsIngredient.equals` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@5 in `ComponentsIngredient.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@6 in `TransferVariant.hasComponents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@88 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@15 in `ItemVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `split` | `()Lnet/minecraft/core/component/DataComponentPatch$SplitResult;` | exact | invokevirtual@17 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `split` | `()Lnet/minecraft/core/component/DataComponentPatch$SplitResult;` | exact | invokevirtual@1 in `TransferApiImpl.writeChangesTo` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@19 in `ComponentsIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@24 in `VariantCodecs.lambda$static$1` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@24 in `VariantCodecs.lambda$static$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@1 in `FluidVariant.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@1 in `ItemVariant.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@29 in `VariantCodecs.lambda$static$1` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@29 in `VariantCodecs.lambda$static$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@16 in `FluidVariantImpl.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@13 in `FluidMixin.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentPatch;` | exact | getstatic@13 in `ItemMixin.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@29 in `ComponentsIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@38 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@79 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (6 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/core/component/DataComponentPatch;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final DELIMITED_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final REMOVED_PREFIX : Ljava/lang/String;
final map : Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
private static createStreamCodec(Lnet/minecraft/core/component/DataComponentPatch$CodecGetter;)Lnet/minecraft/network/codec/StreamCodec;
 <init>(Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;)V
public static builder()Lnet/minecraft/core/component/DataComponentPatch$Builder;
public get(Lnet/minecraft/core/component/DataComponentGetter;Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
static getFromPatchAndPrototype(Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;Lnet/minecraft/core/component/DataComponentGetter;Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
public size()I
public forget(Ljava/util/function/Predicate;)Lnet/minecraft/core/component/DataComponentPatch;
public isEmpty()Z
public split()Lnet/minecraft/core/component/DataComponentPatch$SplitResult;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
static toString(Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;)Ljava/lang/String;
private static synthetic lambda$split$0(Lnet/minecraft/core/component/DataComponentMap$Builder;Ljava/util/Set;Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)V
private static synthetic lambda$static$1(Lnet/minecraft/core/component/DataComponentPatch;)Ljava/util/Map;
private static synthetic lambda$static$0(Ljava/util/Map;)Lnet/minecraft/core/component/DataComponentPatch;
static <clinit>()V
```
