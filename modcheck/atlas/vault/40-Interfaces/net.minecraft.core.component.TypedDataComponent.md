---
type: "interface"
fqcn: "net.minecraft.core.component.TypedDataComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.TypedDataComponent

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `type` | `()Lnet/minecraft/core/component/DataComponentType;` | exact | invokevirtual@19 in `EnchantmentUtil.lambda$modify$0` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/core/component/DataComponentType;` | exact | invokevirtual@59 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `EnchantmentUtil.lambda$modify$0` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@53 in `ComponentsIngredient.test` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (3 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final type : Lnet/minecraft/core/component/DataComponentType;
private final value : Ljava/lang/Object;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)V
static fromEntryUnchecked(Ljava/util/Map$Entry;)Lnet/minecraft/core/component/TypedDataComponent;
public static createUnchecked(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Lnet/minecraft/core/component/TypedDataComponent;
public applyTo(Lnet/minecraft/core/component/PatchedDataComponentMap;)V
public encodeValue(Lcom/mojang/serialization/DynamicOps;)Lcom/mojang/serialization/DataResult;
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public type()Lnet/minecraft/core/component/DataComponentType;
public value()Ljava/lang/Object;
private synthetic lambda$encodeValue$0()Ljava/lang/String;
static <clinit>()V
```
