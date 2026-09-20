---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.InputConstants$Key"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.InputConstants$Key

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getType` | `()Lcom/mojang/blaze3d/platform/InputConstants$Type;` | exact | invokevirtual@4 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getValue` | `()I` | exact | invokevirtual@56 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getValue` | `()I` | exact | invokevirtual@60 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getValue` | `()I` | exact | invokevirtual@99 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final name : Ljava/lang/String;
private final type : Lcom/mojang/blaze3d/platform/InputConstants$Type;
private final value : I
private static final NAME_MAP : Ljava/util/Map;
private <init>(Ljava/lang/String;Lcom/mojang/blaze3d/platform/InputConstants$Type;I)V
public getType()Lcom/mojang/blaze3d/platform/InputConstants$Type;
public getValue()I
public getName()Ljava/lang/String;
public getDisplayName()Lnet/minecraft/network/chat/Component;
public getNumericKeyValue()Ljava/util/OptionalInt;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
static <clinit>()V
```
