---
type: "interface"
fqcn: "net.minecraft.client.renderer.Panorama"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.Panorama

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `holdSpin` | `()V` | exact | invokevirtual@9 in `ScreenMixin.disableRotatingPanoramaForClientGameTests` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (3 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final PANORAMA_OVERLAY : Lnet/minecraft/resources/Identifier;
private spin : F
private shouldSpin : Z
public <init>()V
public startSpin()V
public holdSpin()V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
static <clinit>()V
```
