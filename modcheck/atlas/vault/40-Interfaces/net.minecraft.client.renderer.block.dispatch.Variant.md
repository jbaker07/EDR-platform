---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.Variant"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.Variant

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/client/renderer/block/dispatch/BlockStateModelPart$Unbaked`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `MAP_CODEC` | `Lcom/mojang/serialization/MapCodec;` | exact | getstatic@39 in `CustomUnbakedBlockStateModelRegistry.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (4 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final modelLocation : Lnet/minecraft/resources/Identifier;
private final modelState : Lnet/minecraft/client/renderer/block/dispatch/Variant$SimpleModelState;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/resources/Identifier;)V
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/block/dispatch/Variant$SimpleModelState;)V
public withXRot(Lcom/mojang/math/Quadrant;)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public withYRot(Lcom/mojang/math/Quadrant;)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public withZRot(Lcom/mojang/math/Quadrant;)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public withUvLock(Z)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public withModel(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public withState(Lnet/minecraft/client/renderer/block/dispatch/Variant$SimpleModelState;)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public with(Lnet/minecraft/client/renderer/block/dispatch/VariantMutator;)Lnet/minecraft/client/renderer/block/dispatch/Variant;
public bake(Lnet/minecraft/client/resources/model/ModelBaker;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;
public resolveDependencies(Lnet/minecraft/client/resources/model/ResolvableModel$Resolver;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public modelLocation()Lnet/minecraft/resources/Identifier;
public modelState()Lnet/minecraft/client/renderer/block/dispatch/Variant$SimpleModelState;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
