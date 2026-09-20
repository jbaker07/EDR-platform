---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.WindowEventHandler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.WindowEventHandler

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `resizeGui` | `()V` | exact | invokeinterface@271 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract framebufferSizeChanged()V
public abstract resizeGui()V
public abstract cursorEntered()V
public abstract fullscreenStateChanged(Z)V
```
