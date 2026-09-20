---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.DisplayData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.DisplayData

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `height` | `()I` | exact | invokevirtual@10 in `WindowMixin.onInit` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `width` | `()I` | exact | invokevirtual@2 in `WindowMixin.onInit` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (5 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final width : I
private final height : I
private final fullscreenWidth : Ljava/util/OptionalInt;
private final fullscreenHeight : Ljava/util/OptionalInt;
private final isFullscreen : Z
public <init>(IILjava/util/OptionalInt;Ljava/util/OptionalInt;Z)V
public withSize(II)Lcom/mojang/blaze3d/platform/DisplayData;
public withFullscreen(Z)Lcom/mojang/blaze3d/platform/DisplayData;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public width()I
public height()I
public fullscreenWidth()Ljava/util/OptionalInt;
public fullscreenHeight()Ljava/util/OptionalInt;
public isFullscreen()Z
```
