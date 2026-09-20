---
type: "interface"
fqcn: "net.minecraft.nbt.TagParser"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.TagParser

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `LENIENT_CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@19 in `CustomDataIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (9 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ERROR_TRAILING_DATA : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ERROR_EXPECTED_COMPOUND : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final ELEMENT_SEPARATOR : C
public static final NAME_VALUE_SEPARATOR : C
private static final NBT_OPS_PARSER : Lnet/minecraft/nbt/TagParser;
public static final FLATTENED_CODEC : Lcom/mojang/serialization/Codec;
public static final LENIENT_CODEC : Lcom/mojang/serialization/Codec;
private final ops : Lcom/mojang/serialization/DynamicOps;
private final grammar : Lnet/minecraft/util/parsing/packrat/commands/Grammar;
private <init>(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/util/parsing/packrat/commands/Grammar;)V
public getOps()Lcom/mojang/serialization/DynamicOps;
public static create(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/nbt/TagParser;
private static castToCompoundOrThrow(Lcom/mojang/brigadier/StringReader;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/CompoundTag;
public static parseCompoundFully(Ljava/lang/String;)Lnet/minecraft/nbt/CompoundTag;
public parseFully(Ljava/lang/String;)Ljava/lang/Object;
public parseFully(Lcom/mojang/brigadier/StringReader;)Ljava/lang/Object;
public parseAsArgument(Lcom/mojang/brigadier/StringReader;)Ljava/lang/Object;
public static parseCompoundAsArgument(Lcom/mojang/brigadier/StringReader;)Lnet/minecraft/nbt/CompoundTag;
private static synthetic lambda$static$0(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
static <clinit>()V
```
