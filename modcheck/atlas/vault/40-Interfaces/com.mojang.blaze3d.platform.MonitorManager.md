---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.MonitorManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.MonitorManager

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `findBestMonitor` | `(Lcom/mojang/blaze3d/platform/Window;)Lcom/mojang/blaze3d/platform/Mon` | exact | invokevirtual@57 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final monitors : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
public <init>()V
public onDisplayConnected(I)V
public onDisplayDisconnected(I)V
public onDisplayModeChanged(I)V
private addDisplay(I)Lcom/mojang/blaze3d/platform/Monitor;
public getMonitor(I)Lcom/mojang/blaze3d/platform/Monitor;
public findBestMonitor(Lcom/mojang/blaze3d/platform/Window;)Lcom/mojang/blaze3d/platform/Monitor;
static <clinit>()V
```
