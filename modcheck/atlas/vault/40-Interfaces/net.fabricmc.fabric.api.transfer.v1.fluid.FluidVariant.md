---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public static final com.mojang.serialization.Codec CODEC
public static final net.minecraft.network.codec.StreamCodec PACKET_CODEC
public static net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant blank()
public static net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant of(net.minecraft.world.level.material.Fluid)
public static net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant of(net.minecraft.world.level.material.Fluid, net.minecraft.core.component.DataComponentPatch)
public net.minecraft.world.level.material.Fluid getFluid()
public net.minecraft.core.Holder typeHolder()
public abstract net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant withComponents(net.minecraft.core.component.DataComponentPatch)
public net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant withComponents(net.minecraft.core.component.DataComponentPatch)
```
