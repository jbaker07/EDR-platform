---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributes"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributes

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: class

```java
public static void register(net.minecraft.world.level.material.Fluid, net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributeHandler)
public static net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributeHandler getHandler(net.minecraft.world.level.material.Fluid)
public static net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributeHandler getHandlerOrDefault(net.minecraft.world.level.material.Fluid)
public static net.minecraft.network.chat.Component getName(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static net.minecraft.network.chat.Component getColoredName(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static int getAssociatedColor(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static net.minecraft.sounds.SoundEvent getFillSound(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static net.minecraft.sounds.SoundEvent getEmptySound(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static int getLuminance(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static int getTemperature(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static int getViscosity(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant, net.minecraft.world.level.Level)
public static boolean isLighterThanAir(net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant)
public static void enableColoredVanillaFluidNames()
static {}
```
