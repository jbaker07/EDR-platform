---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.Monitor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.Monitor

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPreferredVideoMode` | `(Ljava/util/Optional;)Lcom/mojang/blaze3d/platform/VideoMode;` | exact | invokevirtual@70 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@116 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@130 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@149 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@157 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `y` | `()I` | exact | invokevirtual@170 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `y` | `()I` | exact | invokevirtual@184 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `y` | `()I` | exact | invokevirtual@203 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `y` | `()I` | exact | invokevirtual@211 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (10 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final name : Ljava/lang/String;
private final id : I
private final videoModes : Ljava/util/List;
private final currentMode : Lcom/mojang/blaze3d/platform/VideoMode;
private final x : I
private final y : I
private final w : I
private final h : I
private static final LOGGER : Lorg/slf4j/Logger;
private static final HEX_FORMAT : Ljava/util/HexFormat;
public <init>(Ljava/lang/String;ILjava/util/List;Lcom/mojang/blaze3d/platform/VideoMode;IIII)V
public static tryCreate(I)Lcom/mojang/blaze3d/platform/Monitor;
private static queryMonitorName(I)Ljava/lang/String;
public getPreferredVideoMode(Ljava/util/Optional;)Lcom/mojang/blaze3d/platform/VideoMode;
public indexOfMode(Lcom/mojang/blaze3d/platform/VideoMode;)I
public mode(I)Lcom/mojang/blaze3d/platform/VideoMode;
public modeCount()I
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public name()Ljava/lang/String;
public id()I
public videoModes()Ljava/util/List;
public currentMode()Lcom/mojang/blaze3d/platform/VideoMode;
public x()I
public y()I
public w()I
public h()I
static <clinit>()V
```
