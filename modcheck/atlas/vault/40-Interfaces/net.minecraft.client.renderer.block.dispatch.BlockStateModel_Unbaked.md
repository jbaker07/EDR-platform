---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/resources/model/ResolvableModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bake` | `(Lnet/minecraft/client/resources/model/ModelBaker;)Lnet/minecraft/clie` | exact | invokeinterface@44 in `CompositeBlockStateModelImpl$Unbaked.bake` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `resolveDependencies` | `(Lnet/minecraft/client/resources/model/ResolvableModel$Resolver;)V` | inherited_exact | invokeinterface@2 in `CompositeBlockStateModelImpl$Unbaked.lambda$resolveDependencies$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@1 in `CompositeBlockStateModelImpl$Unbaked.lambda$static$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| wraps | `<clinit>` | `()V` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| wraps | `<clinit>` | `()V` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (3 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ELEMENT_CODEC : Lcom/mojang/serialization/Codec;
public static final HARDCODED_WEIGHTED_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public abstract bake(Lnet/minecraft/client/resources/model/ModelBaker;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
public asRoot()Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot;
private static synthetic lambda$static$8(Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$Unbaked;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$9()Ljava/lang/String;
private static synthetic lambda$static$5(Lcom/mojang/datafixers/util/Either;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$Unbaked;
private static synthetic lambda$static$7(Lnet/minecraft/client/renderer/block/dispatch/SingleVariant$Unbaked;)Ljava/lang/Record;
private static synthetic lambda$static$6(Lnet/minecraft/client/renderer/block/dispatch/WeightedVariants$Unbaked;)Ljava/lang/Record;
private static synthetic lambda$static$3(Lnet/minecraft/client/renderer/block/dispatch/WeightedVariants$Unbaked;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$4()Ljava/lang/String;
private static synthetic lambda$static$1(Ljava/util/List;)Lnet/minecraft/client/renderer/block/dispatch/WeightedVariants$Unbaked;
private static synthetic lambda$static$2(Lnet/minecraft/util/random/Weighted;)Lnet/minecraft/util/random/Weighted;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
