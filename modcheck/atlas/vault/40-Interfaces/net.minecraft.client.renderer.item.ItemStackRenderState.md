---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemStackRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemStackRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `clear` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `visitExtents` | `(Ljava/util/function/Consumer;)V` | exact | @Inject at ['NEW'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `visitExtents` | `(Ljava/util/function/Consumer;)V` | exact | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (6 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
 displayContext : Lnet/minecraft/world/item/ItemDisplayContext;
private activeLayerCount : I
private animated : Z
private oversizedInGui : Z
private cachedModelBoundingBox : Lnet/minecraft/world/phys/AABB;
private layers : [Lnet/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState;
public <init>()V
public ensureCapacity(I)V
public newLayer()Lnet/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState;
public clear()V
public setAnimated()V
public isAnimated()Z
public appendModelIdentityElement(Ljava/lang/Object;)V
private firstLayer()Lnet/minecraft/client/renderer/item/ItemStackRenderState$LayerRenderState;
public isEmpty()Z
public usesBlockLight()Z
public pickParticleMaterial(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public visitExtents(Ljava/util/function/Consumer;)V
public submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
public getModelBoundingBox()Lnet/minecraft/world/phys/AABB;
public setOversizedInGui(Z)V
public isOversizedInGui()Z
```
