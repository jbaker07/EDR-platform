---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/util/random/WeightedList;)V` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `entries()Lnet/minecraft/util/random/WeightedList;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked extends java.lang.Record implements net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked {
    private final net.minecraft.util.random.WeightedList<net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked> entries;
    public net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked(net.minecraft.util.random.WeightedList<net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked>);
    public net.minecraft.client.renderer.block.dispatch.BlockStateModel bake(net.minecraft.client.resources.model.ModelBaker);
    public void resolveDependencies(net.minecraft.client.resources.model.ResolvableModel$Resolver);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.util.random.WeightedList<net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked> entries();
    private static void lambda$resolveDependencies$0(net.minecraft.client.resources.model.ResolvableModel$Resolver, net.minecraft.util.random.Weighted);
    private static net.minecraft.client.renderer.block.dispatch.BlockStateModel lambda$bake$0(net.minecraft.client.resources.model.ModelBaker, net.minecraft.client.renderer.block.dispatch.BlockStateModel$Unbaked);
}
```
