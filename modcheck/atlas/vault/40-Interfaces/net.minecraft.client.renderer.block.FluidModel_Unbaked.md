---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.FluidModel$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.FluidModel$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `bake(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;` | `` | client | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.block.FluidModel$Unbaked extends java.lang.Record {
    private final net.minecraft.client.resources.model.sprite.Material stillMaterial;
    private final net.minecraft.client.resources.model.sprite.Material flowingMaterial;
    private final net.minecraft.client.resources.model.sprite.Material overlayMaterial;
    private final net.minecraft.client.color.block.BlockTintSource tintSource;
    public net.minecraft.client.renderer.block.FluidModel$Unbaked(net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.color.block.BlockTintSource);
    public net.minecraft.client.renderer.block.FluidModel bake(net.minecraft.client.resources.model.sprite.MaterialBaker, net.minecraft.client.resources.model.ModelDebugName);
    private net.minecraft.client.resources.model.sprite.Material$Baked getAndValidateMaterial(net.minecraft.client.resources.model.sprite.Material, net.minecraft.client.resources.model.sprite.MaterialBaker, java.lang.String, net.minecraft.client.resources.model.ModelDebugName);
    private static com.mojang.blaze3d.platform.Transparency getTransparency(net.minecraft.client.resources.model.sprite.Material$Baked);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.client.resources.model.sprite.Material stillMaterial();
    public net.minecraft.client.resources.model.sprite.Material flowingMaterial();
    public net.minecraft.client.resources.model.sprite.Material overlayMaterial();
    public net.minecraft.client.color.block.BlockTintSource tintSource();
}
```
