---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderingRegistry"
module: "fabric-rendering-fluids-v1"
sha256: "ce4c943f676d105275247800ce1a7ed4a6410c95b3b98ef1b39aba4dcf8a4264"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderingRegistry

Module: [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] -- kind: class

```java
public static net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderHandler get(net.minecraft.world.level.material.Fluid)
public static net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderHandler getOverride(net.minecraft.world.level.material.Fluid)
public static void register(net.minecraft.world.level.material.Fluid, net.minecraft.client.renderer.block.FluidModel$Unbaked, net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderHandler)
public static void register(net.minecraft.world.level.material.Fluid, net.minecraft.client.renderer.block.FluidModel$Unbaked)
public static void register(net.minecraft.world.level.material.Fluid, net.minecraft.world.level.material.Fluid, net.minecraft.client.renderer.block.FluidModel$Unbaked, net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderHandler)
public static void register(net.minecraft.world.level.material.Fluid, net.minecraft.world.level.material.Fluid, net.minecraft.client.renderer.block.FluidModel$Unbaked)
public static void setBlockTransparency(net.minecraft.world.level.block.Block, boolean)
public static boolean isBlockTransparent(net.minecraft.world.level.block.Block)
```
