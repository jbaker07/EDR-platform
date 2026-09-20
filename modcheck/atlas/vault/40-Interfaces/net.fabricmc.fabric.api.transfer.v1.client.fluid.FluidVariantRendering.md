---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRendering"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRendering

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: class

```java
public static void register(net.minecraft.world.level.material.Fluid, net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRenderHandler)
public static net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRenderHandler getHandler(net.minecraft.world.level.material.Fluid)
public static net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRenderHandler getHandlerOrDefault(net.minecraft.world.level.material.Fluid)
public static java.util.List getTooltip(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static java.util.List getTooltip(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant, boolean)
public static java.util.List getTooltip(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant, net.minecraft.world.item.TooltipFlag)
public static java.util.List getTooltip(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant, boolean, net.minecraft.world.item.TooltipFlag)
public static int getColor(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static int getColor(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos)
```
