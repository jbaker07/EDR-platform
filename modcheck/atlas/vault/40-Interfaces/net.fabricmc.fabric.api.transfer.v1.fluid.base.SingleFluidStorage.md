---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.fluid.base.SingleFluidStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.fluid.base.SingleFluidStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: abstract_class

```java
public net.fabricmc.fabric.api.transfer.v1.fluid.base.SingleFluidStorage()
public static net.fabricmc.fabric.api.transfer.v1.fluid.base.SingleFluidStorage withFixedCapacity(long, java.lang.Runnable)
protected final net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant getBlankVariant()
public void readValue(net.minecraft.world.level.storage.ValueInput)
public void writeValue(net.minecraft.world.level.storage.ValueOutput)
protected net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant getBlankVariant()
```
