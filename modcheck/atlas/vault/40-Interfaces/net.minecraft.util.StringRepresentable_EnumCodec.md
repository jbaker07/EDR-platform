---
type: "interface"
fqcn: "net.minecraft.util.StringRepresentable$EnumCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.StringRepresentable$EnumCodec

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `net/minecraft/util/StringRepresentable$StringRepresentableCodec`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fieldOf` | `(Ljava/lang/String;)Lcom/mojang/serialization/MapCodec;` | inherited_exact | invokevirtual@11 in `GameRulesServiceGameRuleUpdateMixin.lambda$fabric_createTypedCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final resolver : Ljava/util/function/Function;
public <init>([Ljava/lang/Enum;Ljava/util/function/Function;)V
public byName(Ljava/lang/String;)Ljava/lang/Enum;
public byName(Ljava/lang/String;Ljava/lang/Enum;)Ljava/lang/Enum;
public byName(Ljava/lang/String;Ljava/util/function/Supplier;)Ljava/lang/Enum;
private static synthetic lambda$new$0(Ljava/lang/Enum;)I
```
