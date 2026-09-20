---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.fluid.CauldronFluidContent"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.fluid.CauldronFluidContent

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: class

```java
public final net.minecraft.world.level.block.Block block
public final net.minecraft.world.level.material.Fluid fluid
public final long amountPerLevel
public final int maxLevel
public final net.minecraft.world.level.block.state.properties.IntegerProperty levelProperty
public static net.fabricmc.fabric.api.transfer.v1.fluid.CauldronFluidContent getForBlock(net.minecraft.world.level.block.Block)
public static net.fabricmc.fabric.api.transfer.v1.fluid.CauldronFluidContent getForFluid(net.minecraft.world.level.material.Fluid)
public static synchronized net.fabricmc.fabric.api.transfer.v1.fluid.CauldronFluidContent registerCauldron(net.minecraft.world.level.block.Block, net.minecraft.world.level.material.Fluid, long, net.minecraft.world.level.block.state.properties.IntegerProperty)
public int currentLevel(net.minecraft.world.level.block.state.BlockState)
```
