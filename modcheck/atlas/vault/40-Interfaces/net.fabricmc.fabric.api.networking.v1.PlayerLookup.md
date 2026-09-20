---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.PlayerLookup"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.PlayerLookup

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static java.util.Collection all(net.minecraft.server.MinecraftServer)
public static java.util.Collection level(net.minecraft.server.level.ServerLevel)
public static java.util.Collection tracking(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.ChunkPos)
public static java.util.Collection tracking(net.minecraft.world.entity.Entity)
public static java.util.Collection tracking(net.minecraft.world.level.block.entity.BlockEntity)
public static java.util.Collection tracking(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos)
public static java.util.Collection around(net.minecraft.server.level.ServerLevel, net.minecraft.world.phys.Vec3, double)
public static java.util.Collection around(net.minecraft.server.level.ServerLevel, net.minecraft.core.Vec3i, double)
```
