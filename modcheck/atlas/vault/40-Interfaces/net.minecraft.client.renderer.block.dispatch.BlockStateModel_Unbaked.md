---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `<clinit>()V` | `@Redirect at INVOKE Lcom/mojang/serialization/Codec;flatComapMap(Ljava/util/func` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| wraps | `<clinit>()V` | `@Redirect at INVOKE Lcom/mojang/serialization/Codec;flatComapMap(Ljava/util/func` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked extends net.minecraft.client.resources.model.ResolvableModel {
    public static final com.mojang.serialization.Codec<net.minecraft.util.random.Weighted<net.minecraft.client.renderer.block.dispatch.Variant>> ELEMENT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked> HARDCODED_WEIGHTED_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked> CODEC;
    public abstract net.minecraft.client.renderer.block.dispatch.BlockStateModel bake(net.minecraft.client.resources.model.ModelBaker);
    public default net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot asRoot();
    private static com.mojang.serialization.DataResult lambda$static$8(net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked);
    private static java.lang.String lambda$static$9();
    private static net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked lambda$static$5(com.mojang.datafixers.util.Either);
    private static java.lang.Record lambda$static$7(net.minecraft.client.renderer.block.dispatch.SingleVariant$Unbaked);
    private static java.lang.Record lambda$static$6(net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked);
    private static com.mojang.serialization.DataResult lambda$static$3(net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked);
    private static java.lang.String lambda$static$4();
    private static net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked lambda$static$1(java.util.List);
    private static net.minecraft.util.random.Weighted lambda$static$2(net.minecraft.util.random.Weighted);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
