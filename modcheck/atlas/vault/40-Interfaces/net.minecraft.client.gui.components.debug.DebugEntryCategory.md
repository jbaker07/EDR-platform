---
type: "interface"
fqcn: "net.minecraft.client.gui.components.debug.DebugEntryCategory"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.debug.DebugEntryCategory

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `SCREEN_TEXT` | `Lnet/minecraft/client/gui/components/debug/DebugEntryCategory;` | exact | getstatic@0 in `DebugOverlayClient$ActiveRendererDebugOverlayEntry.category` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final label : Lnet/minecraft/network/chat/Component;
private final sortKey : F
public static final SCREEN_TEXT : Lnet/minecraft/client/gui/components/debug/DebugEntryCategory;
public static final RENDERER : Lnet/minecraft/client/gui/components/debug/DebugEntryCategory;
public <init>(Lnet/minecraft/network/chat/Component;F)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public label()Lnet/minecraft/network/chat/Component;
public sortKey()F
static <clinit>()V
```
