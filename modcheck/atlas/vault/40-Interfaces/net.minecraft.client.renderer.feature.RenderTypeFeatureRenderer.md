---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/renderer/feature/FeatureRenderer`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@1 in `MovingBlockFeatureRendererMixin.<init>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@1 in `ExtendedBlockModelFeatureRenderer.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@1 in `ExtendedItemFeatureRenderer.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private currentGroup : Lnet/minecraft/client/renderer/feature/RenderTypeFeatureRenderer$Group;
private final groups : Ljava/util/List;
public <init>()V
protected abstract buildGroup(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util/List;)V
protected final getVertexBuilder(Lnet/minecraft/client/renderer/rendertype/RenderType;)Lcom/mojang/blaze3d/vertex/VertexConsumer;
private currentGroup()Lnet/minecraft/client/renderer/feature/RenderTypeFeatureRenderer$Group;
public final prepareGroup(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util/List;Z)V
public executeGroup(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Lnet/minecraft/client/renderer/oit/OitStage;Lcom/mojang/renderpearl/api/commands/RenderPass;ILjava/util/List;Z)V
public finishExecute(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;)V
```
