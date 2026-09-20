---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.OverlayTexture"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.OverlayTexture

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `NO_OVERLAY` | `I` | exact | getstatic@94 in `MovingBlockFeatureRendererMixin$1.accept` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NO_OVERLAY` | `I` | exact | getstatic@15 in `SectionCompilerMixin.lambda$beforeLoopCompile$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NO_OVERLAY` | `I` | exact | getstatic@103 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NO_OVERLAY` | `I` | exact | getstatic@94 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `NO_OVERLAY` | `I` | exact | getstatic@158 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (6 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SIZE : I
public static final NO_WHITE_U : I
public static final RED_OVERLAY_V : I
public static final WHITE_OVERLAY_V : I
public static final NO_OVERLAY : I
private final texture : Lnet/minecraft/client/renderer/texture/DynamicTexture;
public <init>()V
public close()V
public static u(F)I
public static v(Z)I
public static pack(II)I
public static pack(FZ)I
public getTextureView()Lcom/mojang/renderpearl/api/textures/GpuTextureView;
static <clinit>()V
```
