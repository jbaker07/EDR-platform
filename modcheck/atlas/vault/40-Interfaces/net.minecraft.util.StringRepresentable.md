---
type: "interface"
fqcn: "net.minecraft.util.StringRepresentable"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.StringRepresentable

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fromEnum` | `(Ljava/util/function/Supplier;)Lnet/minecraft/util/StringRepresentable` | exact | invokestatic@56 in `EventResult.<clinit>` | unknown | [[30-Mechanisms/fabric-api-base|fabric-api-base]] | direct_reference |
| calls | `fromEnum` | `(Ljava/util/function/Supplier;)Lnet/minecraft/util/StringRepresentable` | exact | invokestatic@56 in `TriState.<clinit>` | unknown | [[30-Mechanisms/fabric-api-base|fabric-api-base]] | direct_reference |
| calls | `fromEnum` | `(Ljava/util/function/Supplier;)Lnet/minecraft/util/StringRepresentable` | exact | invokestatic@41 in `SoundTypeBuilder$RegistrationType.<clinit>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `fromEnum` | `(Ljava/util/function/Supplier;)Lnet/minecraft/util/StringRepresentable` | exact | invokestatic@6 in `GameRulesServiceGameRuleUpdateMixin.lambda$fabric_createTypedCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (1 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final PRE_BUILT_MAP_THRESHOLD : I
public abstract getSerializedName()Ljava/lang/String;
public static fromEnum(Ljava/util/function/Supplier;)Lnet/minecraft/util/StringRepresentable$EnumCodec;
public static fromEnumWithMapping(Ljava/util/function/Supplier;Ljava/util/function/Function;)Lnet/minecraft/util/StringRepresentable$EnumCodec;
public static fromValues(Ljava/util/function/Supplier;)Lcom/mojang/serialization/Codec;
public static createNameLookup([Lnet/minecraft/util/StringRepresentable;)Ljava/util/function/Function;
public static createNameLookup([Ljava/lang/Object;Ljava/util/function/Function;)Ljava/util/function/Function;
public static keys([Lnet/minecraft/util/StringRepresentable;)Lcom/mojang/serialization/Keyable;
private static synthetic lambda$createNameLookup$1([Ljava/lang/Object;Ljava/util/function/Function;Ljava/lang/String;)Ljava/lang/Object;
private static synthetic lambda$createNameLookup$0(Ljava/lang/Object;)Ljava/lang/Object;
private static synthetic lambda$fromEnumWithMapping$0(Ljava/util/function/Function;Ljava/lang/Enum;)Ljava/lang/String;
private static synthetic lambda$fromEnum$0(Ljava/lang/String;)Ljava/lang/String;
```
