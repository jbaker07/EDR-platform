---
type: "interface"
fqcn: "net.minecraft.Optionull"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.Optionull

System: [[20-Systems/net.minecraft|net.minecraft]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isNullOrEmpty` | `([Ljava/lang/Object;)Z` | exact | invokestatic@1 in `GameRuleBuilder$EnumRuleBuilder.supportedValues` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;` | exact | invokestatic@43 in `ClientGameTestContextImpl.lambda$clickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mapOrElse` | `(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/Su` | exact | invokestatic@15 in `ResourceConditionType.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (0 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static orElse(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
public static map(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/Object;
public static mapOrDefault(Ljava/lang/Object;Ljava/util/function/Function;Ljava/lang/Object;)Ljava/lang/Object;
public static mapOrElse(Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/Supplier;)Ljava/lang/Object;
public static first(Ljava/util/Collection;)Ljava/lang/Object;
public static firstOrDefault(Ljava/util/Collection;Ljava/lang/Object;)Ljava/lang/Object;
public static firstOrElse(Ljava/util/Collection;Ljava/util/function/Supplier;)Ljava/lang/Object;
public static isNullOrEmpty([Ljava/lang/Object;)Z
public static isNullOrEmpty([Z)Z
public static isNullOrEmpty([B)Z
public static isNullOrEmpty([C)Z
public static isNullOrEmpty([S)Z
public static isNullOrEmpty([I)Z
public static isNullOrEmpty([J)Z
public static isNullOrEmpty([F)Z
public static isNullOrEmpty([D)Z
```
