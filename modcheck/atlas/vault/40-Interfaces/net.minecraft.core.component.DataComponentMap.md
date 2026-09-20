---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentMap

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`interface` public abstract; extends `java/lang/Object`; implements `java/lang/Iterable`, `net/minecraft/core/component/DataComponentGetter`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `()Lnet/minecraft/core/component/DataComponentMap$Builder;` | exact | invokestatic@40 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;` | inherited_exact | invokeinterface@66 in `ComposterWrapper$TopStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | inherited_exact | invokeinterface@19 in `WaterPotionStorage.isWaterPotion` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | inherited_exact | invokeinterface@20 in `BundleContentsStorage.bundleContents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | inherited_exact | invokeinterface@20 in `ItemContainerContentsStorage.container` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOrDefault` | `(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lj` | inherited_exact | invokeinterface@21 in `ItemVariantImpl.getMaxStackSize` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | exact | invokeinterface@25 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokeinterface@81 in `ContainerSlotWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `stream` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@52 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentMap;` | exact | getstatic@22 in `FluidVariantImpl.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/core/component/DataComponentMap;` | exact | getstatic@28 in `FluidVariantImpl.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/core/component/DataComponentMap;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static makeCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static makeCodecFromMap(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static composite(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecraft/core/component/DataComponentMap;)Lnet/minecraft/core/component/DataComponentMap;
public static builder()Lnet/minecraft/core/component/DataComponentMap$Builder;
public abstract keySet()Ljava/util/Set;
public has(Lnet/minecraft/core/component/DataComponentType;)Z
public iterator()Ljava/util/Iterator;
public stream()Ljava/util/stream/Stream;
public size()I
public isEmpty()Z
public filter(Ljava/util/function/Predicate;)Lnet/minecraft/core/component/DataComponentMap;
private synthetic lambda$iterator$0(Lnet/minecraft/core/component/DataComponentType;)Lnet/minecraft/core/component/TypedDataComponent;
private static synthetic lambda$makeCodecFromMap$0(Lnet/minecraft/core/component/DataComponentMap;)Lcom/mojang/serialization/DataResult;
static <clinit>()V
```
